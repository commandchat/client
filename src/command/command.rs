use super::CommandError;
use regex::Regex;

pub enum Command {
	Join(String),
}

impl Command {
	pub fn new(line: &String) -> Option<Result<Command, CommandError>> {
		lazy_static! {
			static ref COMMAND: Regex = Regex::new(r"^/([a-z]+)\s+(.+)$").unwrap();
		}

		let captures = COMMAND.captures(line)?;
		let value = String::from(captures[2].trim());

		Some(match &captures[1] {
			"join" => Ok(Command::Join(value)),
			_ => Err(CommandError::NotFound),
		})
	}
}
