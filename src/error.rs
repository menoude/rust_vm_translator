use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum TranslateError {
	Io(io::Error),
	WrongIndex,
	WrongFilePath,
}

impl Display for TranslateError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let message = match &self {
			TranslateError::Io(e) => e.description(),
			WrongIndex => "Try to access wrong index",
			WrongFilePath => "Wrong file path",
		};
		write!(f, "Error: {}", message)
	}
}

impl std::error::Error for TranslateError {}

impl From<std::io::Error> for TranslateError {
	fn from(e: std::io::Error) -> Self {
		TranslateError::Io(e)
	}
}
