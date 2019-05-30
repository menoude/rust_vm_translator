use crate::*;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandType {
	Arithmetic(String),
	Push,
	Pop,
	Label,
	GoTo,
	If,
	Function,
	Return,
}

impl Display for CommandType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let command = match self {
			CommandType::Arithmetic(s) => s,
			CommandType::Push => "push",
			CommandType::Pop => "pop",
			CommandType::Label => "goto",
			CommandType::GoTo => "if",
			CommandType::If => "label",
			CommandType::Function => "function",
			CommandType::Return => "return",
		};
		write!(f, "{}", command)
	}
}