use std::{error, fmt};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum RunTimeError {
	InvalidDeviceType,
	InvalidIOPort,
	InvalidCommand,
	MissingArgument,
	MalformedArgument,
	BadFilePath,
	BadFile,
}

impl fmt::Display for RunTimeError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			RunTimeError::InvalidDeviceType => "Invalid IO device type",
			RunTimeError::InvalidIOPort => "Invalid IO port",
			RunTimeError::InvalidCommand => "Invalid command\n  Type \"help\" or \"?\" for help",
			RunTimeError::MissingArgument => "Missing argument",
			RunTimeError::MalformedArgument => "Malformed argument or unparsable number",
			RunTimeError::BadFilePath => "Cannot find file specified",
			RunTimeError::BadFile => "Error while loading file",
		};
		write!(f, "{}", s)
	}
}

impl error::Error for RunTimeError {}
