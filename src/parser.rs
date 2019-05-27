use super::*;
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;
use std::ops::Deref;
use std::str::Lines;

pub fn read_content(file_data: &FileData) -> Result<String> {
	let mut file = File::open(file_data.original_path.deref())?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;
	Ok(content)
}

#[derive(Debug)]
pub struct Parser<'a> {
	lines: Peekable<Lines<'a>>,
	pub current_line: Vec<&'a str>,
	line_index: u16,
}

impl<'a> Parser<'a> {
	pub fn new(content: &'a str) -> Result<Self> {
		let parser = Parser {
			lines: content.lines().peekable(),
			current_line: vec![&content],
			line_index: 0,
		};
		Ok(parser)
	}

	pub fn advance(&mut self) -> bool {
		while let Some(line) = self.lines.next() {
			self.line_index += 1;
			if !line.is_empty() && !line.trim_start().starts_with("//") {
				self.current_line = line.split_whitespace().collect();
				return true;
			}
		}
		false
	}

	pub fn get_line_index(&self) -> u16 {
		self.line_index
	}

	pub fn command_type(&mut self) -> Result<CommandType> {
		match self.current_line[0] {
			"add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
				Ok(CommandType::Arithmetic(self.current_line[0].to_owned()))
			}
			"push" => Ok(CommandType::Push),
			"pop" => Ok(CommandType::Pop),
			"goto" => Ok(CommandType::GoTo),
			"if-goto" => Ok(CommandType::If),
			"label" => Ok(CommandType::Label),
			"function" => Ok(CommandType::Function),
			"return" => Ok(CommandType::Return),
			command => Err(TranslateError::IncorrectCommand(
				command.to_owned(),
				self.line_index,
			)),
		}
	}

	pub fn arg_1(&self) -> Result<&str> {
		self.current_line
			.get(1)
			.cloned()
			.ok_or_else(|| TranslateError::WrongIndex(1, self.line_index))
	}

	pub fn arg_2(&self) -> Result<&str> {
		self.current_line
			.get(2)
			.cloned()
			.ok_or_else(|| TranslateError::WrongIndex(2, self.line_index))
	}
}
