use std::iter::once;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{DefaultBodyLimit, Host, MatchedPath};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{Request, StatusCode, Uri};
use axum::http::header::AUTHORIZATION;
use axum::response::{Redirect, Response};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::trace::TraceLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::{debug, error, info, info_span, instrument, Span, warn};

use rs2_utils::duration_extension::DurationExt;
use rs2_utils::unrecoverable_error::unrecoverable_error;
use srv_mod_config::handlers::HandlerConfig;
use srv_mod_config::SharedConfig;
use srv_mod_database::{humantime, Pool};
use state::HttpHandlerSharedState;

mod routes;
mod state;

#[instrument(name = "HTTP handler", skip_all)]
pub async fn start(
	config: Arc<HandlerConfig>,
	cancellation_token: CancellationToken,
	pool: Pool,
) -> anyhow::Result<()> {
	// create a shared state for the server
	let shared_state: HttpHandlerSharedState = Arc::new(state::HttpHandlerState {
		config: config.clone(),
		db_pool: pool,
	});

	// init the router
	let app = Router::new()
		.merge(routes::public::make_routes(shared_state.clone()))
		.with_state(shared_state)
		.layer((
			// add log tracing
			TraceLayer::new_for_http()
				.make_span_with(|request: &Request<_>| {
					let matched_path = if let Some(path) = request
						.extensions()
						.get::<MatchedPath>()
						.map(MatchedPath::as_str)
					{
						path
					} else {
						"None"
					};

					info_span!(
                        "http_request",
                        method = %request.method(),
                        path = %request.uri().path(),
                        matched_path,
                        latency = tracing::field::Empty,
                        status = tracing::field::Empty,
                    )
				})
				.on_request(|_request: &Request<_>, _span: &Span| {
					info!("Started processing request")
				})
				.on_response(|response: &Response<_>, latency: Duration, span: &Span| {
					let status = response.status();

					span.record(
						"latency",
						humantime::format_duration(latency.round()).to_string(),
					);
					span.record(
						"status",
						format!(
							"{} {}",
							status.as_str(),
							status.canonical_reason().unwrap_or("")
						),
					);

					info!("Request processed")
				}),
			// catch panics (if any) most likely from external crates, just to avoid crashing the server
			CatchPanicLayer::new(),
			// add compression support
			CompressionLayer::new(),
			// normalize paths before routing trimming trailing slashes
			NormalizePathLayer::trim_trailing_slash(),
			// limit request body size to 50mb
			DefaultBodyLimit::disable(),
			RequestBodyLimitLayer::new(0x6400000), /* 100mb = (100 * 1024 * 1024) */
			// validate request headers for content type accepting only text/plain, this is to avoid allowing potential
			// blue-teams to identify the protocol used during the communication simply by looking at the content type
			ValidateRequestHeaderLayer::accept("text/plain"),
		));

	if let Some(tls_config) = config.tls.as_ref() {
		info!("Starting HTTP handler with TLS support");
		warn!("Plain http server will be automatically redirected to https");

		tokio::spawn(redirect_http_to_https(
			config.host.clone(),
			config.port,
			tls_config.port,
			cancellation_token.clone(),
		));

		let rustls_config = RustlsConfig::from_pem_file(tls_config.cert.clone(), tls_config.key.clone()).await?;

		let listener = tokio::net::TcpListener::bind(format!(
			"{}:{}",
			if let Some(tls_host) = tls_config.host.clone() {
				tls_host
			} else {
				config.host.clone()
			},
			tls_config.port
		))
			.await;

		let listener = unwrap_listener_or_fail(
			config.host.clone(),
			tls_config.port,
			listener,
		);

		info!(address = %listener.local_addr().unwrap(), "HTTP handler with tls listening");

		select! {
            _ = axum_server::from_tcp_rustls(listener.into_std()?, rustls_config).serve(app.into_make_service()) => {},
            _ = handle_graceful_shutdown("HTTPS", cancellation_token) => {},
        }

		return Ok(());
	}

	// start listening on the provided address
	let listener = tokio::net::TcpListener::bind(format!(
		"{}:{}",
		config.host, config.port
	))
		.await;

	let listener = unwrap_listener_or_fail(
		config.host.clone(),
		config.port,
		listener,
	);

	info!(address = %listener.local_addr().unwrap(), "HTTP handler listening");

	// start serving requests
	axum::serve(listener, app)
		.with_graceful_shutdown(handle_graceful_shutdown("HTTP", cancellation_token))
		.await?;

	Ok(())
}

/// Unwraps the listener or fails with an unrecoverable error
fn unwrap_listener_or_fail(
	host: String,
	port: u16,
	listener: std::io::Result<tokio::net::TcpListener>,
) -> tokio::net::TcpListener {
	if listener.is_err() {
		error!("Cannot bind to {} at port {}", host, port);
		unrecoverable_error().unwrap();
	}

	listener.unwrap()
}

/// Handle the shutdown signal gracefully closing all connections and waiting for all requests to complete
async fn handle_graceful_shutdown(context: &str, cancellation_token: CancellationToken) {
	cancellation_token.cancelled().await;
	warn!("{context} HTTP handler shutting down");
}

/// Redirects all http requests to https
async fn redirect_http_to_https(
	host: String,
	http_port: u16,
	https_port: u16,
	cancellation_token: CancellationToken,
) -> anyhow::Result<()> {
	let redirect = move |Host(host): Host, uri: Uri| async move {
		match make_https(host, uri.clone(), http_port, https_port) {
			Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
			Err(error) => {
				warn!(%error, %uri, "Failed to convert URI to HTTPS");
				Err(StatusCode::BAD_REQUEST)
			}
		}
	};

	// start listening on the provided address
	let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, http_port)).await;

	let listener = unwrap_listener_or_fail(host.clone(), http_port, listener);

	info!(address = %listener.local_addr().unwrap(), "HTTP handler listening");

	axum::serve(listener, redirect.into_make_service())
		.with_graceful_shutdown(handle_graceful_shutdown("HTTP", cancellation_token))
		.await
		.unwrap();

	Ok(())
}

/// Converts a http uri to https
fn make_https(host: String, uri: Uri, http_port: u16, https_port: u16) -> anyhow::Result<Uri> {
	let mut parts = uri.into_parts();

	parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

	if parts.path_and_query.is_none() {
		parts.path_and_query = Some("/".parse().unwrap());
	}

	let https_host = host.replace(&http_port.to_string(), &https_port.to_string());
	parts.authority = Some(https_host.parse()?);

	Ok(Uri::from_parts(parts)?)
}
