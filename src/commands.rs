use crate::*;

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
