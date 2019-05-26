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
			command => Err(TranslateError::IncorrectCommand(command.to_owned())),
		}
	}

	pub fn arg_1(&self) -> Result<&str> {
		self.current_line
			.get(1)
			.map(|&token| token)
			.ok_or(TranslateError::WrongIndex(1))
	}

	pub fn arg_2(&self) -> Result<&str> {
		self.current_line
			.get(2)
			.map(|&token| token)
			.ok_or(TranslateError::WrongIndex(2))
	}
}
