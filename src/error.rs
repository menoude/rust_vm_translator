use crate::*;
use std::fmt;
use std::fmt::Display;
use std::num;
use std::path::PathBuf;
use std::string::ToString;
use std::io;

#[derive(Debug)]
pub enum TranslateError {
	Error,
	Io(io::Error),
	Parsing(num::ParseIntError),
	WrongIndex(u16, u16),
	WrongFilePath(PathBuf),
	NoVmFile(PathBuf),
	IncorrectCommand(String, u16),
}

impl Display for TranslateError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let message = match &self {
			TranslateError::Error => "".to_owned(),
			TranslateError::Io(e) => e.to_string(),
			TranslateError::Parsing(e) => e.to_string(),
			TranslateError::WrongIndex(index, line) => format!(
				"Try to access index {} which is invalid, line {}",
				index, line
			),
			TranslateError::WrongFilePath(path) => format!("Wrong file path '{}'", path.display()),
			TranslateError::NoVmFile(path) => format!("No vm file in directory {}", path.display()),
			TranslateError::IncorrectCommand(command, line) => {
				format!("Incorrect command '{}' at line {}", command, line)
			}
		};
		write!(f, "{}", message)
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
