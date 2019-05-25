use super::*;
use crate::error::*;
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;
use std::str::Lines;

pub fn read_content(file_data: FileData) -> Result<String> {
	let mut file = File::open(file_data.original_path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;
	Ok(content)
}

#[derive(Debug)]
pub struct Parser<'a> {
	lines: Peekable<Lines<'a>>,
	pub current_line: Vec<&'a str>,
}

impl<'a> Parser<'a> {
	pub fn new(content: &'a str) -> Result<Self> {
		let parser = Parser {
			lines: content.lines().peekable(),
			current_line: vec![&content],
		};
		Ok(parser)
	}

	pub fn advance(&mut self) -> bool {
		while let Some(line) = self.lines.next() {
			if !line.is_empty() {
				self.current_line = line.split_whitespace().collect();
				return true;
			}
		}
		false
	}

	pub fn command_type(&mut self) -> &str {
		self.current_line[0]
	}

	pub fn arg_1(&mut self) -> Result<&str> {
		self.current_line
			.get(1)
			.map(|&token| token)
			.ok_or(TranslateError::new(
				TranslateErrorKind::WrongIndex,
				"Wrong index",
			))
	}

	pub fn arg_2(&mut self) -> Result<&str> {
		self.current_line
			.get(2)
			.map(|&token| token)
			.ok_or(TranslateError::new(
				TranslateErrorKind::WrongIndex,
				"Wrong index",
			))
	}
}
