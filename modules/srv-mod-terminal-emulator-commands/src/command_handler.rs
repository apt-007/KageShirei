use std::fmt::Debug;

use anyhow::Result;
use serde::Serialize;

pub trait CommandHandler: Debug {
	/// Handle the command
	fn handle_command(&self, session_id: &str) -> Result<String>;
}

pub trait SerializableCommandHandler: CommandHandler + Serialize {}