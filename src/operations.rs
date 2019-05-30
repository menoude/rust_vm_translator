use crate::*;
use std::convert::TryFrom;


pub enum Operation {
	Add,
	Sub,
	Neg,
	r#Eq,
	Gt,
	Lt,
	And,
	Or,
	Not,
}

impl TryFrom<String> for Operation {
	type Error = TranslateError;
	fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
		match s.as_str() {
			"add" => Ok(Operation::Add),
			"sub" => Ok(Operation::Sub),
			"neg" => Ok(Operation::Neg),
			"eq" => Ok(Operation::Eq),
			"gt" => Ok(Operation::Gt),
			"lt" => Ok(Operation::Lt),
			"and" => Ok(Operation::And),
			"or" => Ok(Operation::Or),
			"not" => Ok(Operation::Not),
			_ => Err(TranslateError::Error),
		}
	}
}