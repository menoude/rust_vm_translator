use crate::*;
use std::convert::TryFrom;


pub enum Operator {
	Binary(BinaryOperator),
	Unary(UnaryOperator),
	Compararison(ComparisonOperator),
}

pub enum BinaryOperator {
	Add,
	Sub,
	And,
	Or,
}

pub enum UnaryOperator {
	Neg,
	Not,
}

pub enum ComparisonOperator {
	r#Eq,
	Gt,
	Lt,
}

impl BinaryOperator {
	pub fn to_asm(&self) -> String {
		let specific = match self {
			BinaryOperator::Add => "M=D+M",
			BinaryOperator::Sub => "M=M-D",
			BinaryOperator::And => "M=D&M",
			BinaryOperator::Or => "M=D|M",
		};
		format!(
			"@SP\n\
			 M=M-1\n\
			 A=M\n\
			 D=M\n\
			 @SP\n\
			 A=M-1\n\
			 {}\n",
			specific
		)
	}
}

impl UnaryOperator {
	pub fn to_asm(&self) -> String {
		let specific = match self {
			UnaryOperator::Neg => "M=-M",
			UnaryOperator::Not => "M=!M",
		};
		format!(
			"@SP\n\
			 A=M-1\n\
			 {}\n",
			specific
		)
	}
}

impl ComparisonOperator {
	pub fn to_asm(&self) -> String {
		unimplemented!()
	}
}

impl TryFrom<String> for Operator {
	type Error = TranslateError;
	fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
		match s.as_str() {
			"add" => Ok(Operator::Binary(BinaryOperator::Add)),
			"sub" => Ok(Operator::Binary(BinaryOperator::Sub)),
			"neg" => Ok(Operator::Unary(UnaryOperator::Neg)),
			"eq" => Ok(Operator::Compararison(ComparisonOperator::Eq)),
			"gt" => Ok(Operator::Compararison(ComparisonOperator::Gt)),
			"lt" => Ok(Operator::Compararison(ComparisonOperator::Lt)),
			"and" => Ok(Operator::Binary(BinaryOperator::And)),
			"or" => Ok(Operator::Binary(BinaryOperator::Or)),
			"not" => Ok(Operator::Unary(UnaryOperator::Not)),
			_ => Err(TranslateError::Error),
		}
	}
}