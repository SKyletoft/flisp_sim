use std::{error::Error, fmt, result};

pub type Result<T> = result::Result<T, FlispError>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlispError {
	InvalidOpCode(u32),
	InvalidLineConversion(u32),
	FormatError,
}

impl fmt::Display for FlispError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl From<fmt::Error> for FlispError {
	fn from(_: fmt::Error) -> Self {
		FlispError::FormatError
	}
}

impl Error for FlispError {}
