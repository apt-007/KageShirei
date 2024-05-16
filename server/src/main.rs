use clap::Parser;
use log::{debug, error, info, trace, warn};
use validator::ValidationErrors;

use crate::async_main::async_main;
use crate::cli::base::{CliArguments, Commands};
use crate::config::config::RootConfig;

mod compile;
mod cli;
mod print_validation_error;
mod async_main;
mod config;
mod generate;
mod database;

fn setup_logging(debug_level: u8) -> anyhow::Result<()> {
	let mut base_config = fern::Dispatch::new()
		.format(|out, message, record| {
			let level_padding = if record.level().to_string().len() < 5 {
				" ".repeat(5 - record.level().to_string().len() + 1).to_string()
			} else {
				" ".to_string()
			};

			let colors = fern::colors::ColoredLevelConfig::new()
				.info(fern::colors::Color::Green)
				.warn(fern::colors::Color::Yellow)
				.error(fern::colors::Color::Red)
				.debug(fern::colors::Color::Blue)
				.trace(fern::colors::Color::Magenta);

			let additional_info = if record.level() > log::LevelFilter::Debug {
				format!(" [{}:{}]", record.file().unwrap_or(""), record.line().unwrap_or(0))
			} else {
				"".to_string()
			};

			out.finish(format_args!(
				"[{}]{}[{}]{} {}",
				colors.color(record.level()),
				level_padding,
				humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
				additional_info,
				message
			))
		})
		.level(log::LevelFilter::Trace)
		.chain(std::io::stdout());

	base_config = match debug_level {
		0 => base_config.level(log::LevelFilter::Info),
		1 => base_config.level(log::LevelFilter::Debug),
		_ => base_config.level(log::LevelFilter::Trace),
	};

	base_config.apply()?;
	Ok(())
}

fn main() -> anyhow::Result<()> {
	let args = CliArguments::parse();

	setup_logging(args.debug)?;
	trace!("Parsed arguments: {:?}", args);

	match args.command {
		Commands::Compile(compile_args) => {
			match compile_args.command {
				cli::compile::CompileSubcommands::Agent => {
					unimplemented!("Agent compilation not implemented yet");
				}
				cli::compile::CompileSubcommands::Gui => {
					compile::c2_gui::compile()?;
				}
			}
		}
		Commands::Generate(generate_args) => {
			match generate_args.command {
				cli::generate::GenerateSubcommands::Jwt => {
					generate::jwt::generate_jwt()?;
				}
			}
		}
		Commands::Run(run_args) => {
			let config = RootConfig::load(&run_args.config);

			if let Err(e) = config {
				error!("Failed to load configuration");

				let e = e.downcast::<ValidationErrors>();

				if e.is_err() {
					error!("Cannot parse configuration file: {:?}", e.err().unwrap());
					return Err(anyhow::anyhow!("Unrecoverable error(s) detected, exiting.")); // Exit with error state
				}
				let e = e.unwrap();

				print_validation_error::print_validation_error(e)?;
				return Err(anyhow::anyhow!("Unrecoverable error(s) detected, exiting.")); // Exit with error state
			}
			let config = config.unwrap();

			tokio::runtime::Builder::new_multi_thread()
				.enable_all()
				.build()
				.unwrap()
				.block_on(async {
					if args.debug > config.read().await.debug_level.unwrap_or(0) {
						warn!("Command line debug level is higher than the defined in the configuration file, debug level will be overridden");
						config.write().await.debug_level = Some(args.debug);
					}

					trace!("Loaded configuration: {:?}", config);
					info!("Configuration successfully loaded!");
					warn!("Switching context to the async server runtime...");
					async_main(config).await.unwrap();
				});
		}
	}

	Ok(())
}
