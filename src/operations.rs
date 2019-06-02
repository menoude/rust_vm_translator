use crate::*;

pub enum Operator {
	Binary(BinaryOperator),
	Unary(UnaryOperator),
	Comparison(ComparisonOperator),
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
	pub fn specific_instruction(&self) -> &'static str {
		match self {
			BinaryOperator::Add => "M=D+M",
			BinaryOperator::Sub => "M=M-D",
			BinaryOperator::And => "M=D&M",
			BinaryOperator::Or => "M=D|M",
		}
	}
}

impl UnaryOperator {
	pub fn specific_instruction(&self) -> &'static str {
		match self {
			UnaryOperator::Neg => "M=-M",
			UnaryOperator::Not => "M=!M",
		}
	}
}

impl ComparisonOperator {
	pub fn specific_instruction(&self) -> &'static str {
		match self {
			ComparisonOperator::Eq => "D;JNE",
			ComparisonOperator::Gt => "D;JLE",
			ComparisonOperator::Lt => "D;JGE",
		}
	}
}

impl Operator {
	pub fn new(s: String, original_line_nb: u16) -> Result<Self> {
		match s.as_str() {
			"add" => Ok(Operator::Binary(BinaryOperator::Add)),
			"sub" => Ok(Operator::Binary(BinaryOperator::Sub)),
			"neg" => Ok(Operator::Unary(UnaryOperator::Neg)),
			"eq" => Ok(Operator::Comparison(ComparisonOperator::Eq)),
			"gt" => Ok(Operator::Comparison(ComparisonOperator::Gt)),
			"lt" => Ok(Operator::Comparison(ComparisonOperator::Lt)),
			"and" => Ok(Operator::Binary(BinaryOperator::And)),
			"or" => Ok(Operator::Binary(BinaryOperator::Or)),
			"not" => Ok(Operator::Unary(UnaryOperator::Not)),
			other => Err(TranslateError::IncorrectCommand(
				other.to_owned(),
				original_line_nb,
			)),
		}

	}

	pub fn to_asm(&self, labels_count: u32) -> String {
		match self {
			Operator::Binary(kind) => {
				let specific = kind.specific_instruction();
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
			Operator::Unary(kind) => {
				let specific = kind.specific_instruction();
				format!(
					"@SP\n\
					 A=M-1\n\
					 {}\n",
					specific
				)
			}
			Operator::Comparison(kind) => {
				let specific = kind.specific_instruction();
				let false_label = format!("FALSE_{}", labels_count);
				let end_label = format!("END_{}", labels_count);
				format!(
					"@SP\n\
					 M=M-1\n\
					 A=M\n\
					 D=M\n\
					 @SP\n\
					 A=M-1\n\
					 D=M-D\n\
					 M=-1\n\
					 @{}\n\
					 {}\n\
					 @{}\n\
					 0;JMP\n\
					 ({})\n\
					 @SP\n\
					 A=M-1\n\
					 M=0\n\
					 ({})\n",
					false_label, specific, end_label, false_label, end_label
				)
			}
		}
	}
}
