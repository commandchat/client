use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum CommandError {
	NotFound,
}

impl Display for CommandError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		match self {
			CommandError::NotFound => write!(formatter, "Command not found"),
		}
	}
}
