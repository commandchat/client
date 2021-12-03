use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum CommandError {
	NotFound,
}

impl Display for CommandError {
	fn fmt(&self, formatter: &mut Formatter) -> Result {
		match self {
			CommandError::NotFound => write!(formatter, "Command not found"),
		}
	}
}
