use super::FileData;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;
use std::str::Lines;

pub fn read_content(file_data: FileData) -> Result<String, Box<dyn Error>> {
	let mut file = File::open(file_data.original_path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;
	Ok(content)
}

#[derive(Debug)]
pub struct Parser<'a> {
	lines: Peekable<Lines<'a>>,
	current_line: String,
}

impl<'a> Parser<'a> {
	pub fn new(content: &'a str) -> Result<Self, Box<dyn Error>> {
		let parser = Parser {
			lines: content.lines().peekable(),
			current_line: String::new(),
		};
		Ok(parser)
	}

	pub fn has_more_commands(&mut self) -> bool {
		self.lines.peek() != None
	}

	pub fn advance(&mut self) {
		self.current_line = self.lines.next().unwrap().to_owned();
	}

	pub fn command_type(&self) -> &str {
		&self.current_line
	}

	pub fn arg_1(&self) -> &str {
		&self.current_line
	}

	pub fn arg_2(&self) -> &str {
		&self.current_line
	}
}
