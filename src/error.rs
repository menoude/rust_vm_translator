use std::convert::From;
use std::fmt;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub struct TranslateError {
	kind: TranslateErrorKind,
	message: String,
}
#[derive(Debug)]
pub enum TranslateErrorKind {
	Io(io::ErrorKind),
	WrongIndex,
	WrongFilePath,
}

impl TranslateError {
	pub fn new(kind: TranslateErrorKind, message: &str) -> Self {
		TranslateError {
			kind,
			message: message.to_string(),
		}
	}
}

impl Display for TranslateError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error: {}", self.message)
	}
}

impl std::error::Error for TranslateError {}

impl From<std::io::Error> for TranslateError {
	fn from(e: std::io::Error) -> Self {
		TranslateError {
			kind: TranslateErrorKind::Io(e.kind()),
			message: e.to_string(),
		}
	}
}
