use crate::*;
use std::fmt;
use std::io;
use std::num;
use std::path::PathBuf;
use std::string::ToString;

#[derive(Debug)]
pub enum TranslateError {
	Io(io::Error),
	Parsing(num::ParseIntError),
	WrongIndex(u16),
	WrongFilePath(PathBuf),
	IncorrectCommand(String),
}

impl Display for TranslateError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let message = match &self {
			TranslateError::Io(e) => e.to_string(),
			TranslateError::Parsing(e) => e.to_string(),
			TranslateError::WrongIndex(index) => {
				format!("Try to access index {} which is invalid", index)
			}
			TranslateError::WrongFilePath(path) => {
				format!("Wrong file path '{}'", path.to_str().unwrap_or(""))
			}
			TranslateError::IncorrectCommand(command) => format!("Incorrect command '{}'", command),
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

impl From<std::num::ParseIntError> for TranslateError {
	fn from(e: std::num::ParseIntError) -> Self {
		TranslateError::Parsing(e)
	}
}
