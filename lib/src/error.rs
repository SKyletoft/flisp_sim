use crate::*;
use std::{error::Error, fmt, result};

pub type Result<T> = result::Result<T, FlispError>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlispError {
	InvalidOpCode(u32),
	InvalidLineConversion(u32),
}

impl fmt::Display for FlispError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Error for FlispError {}
