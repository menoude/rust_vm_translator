use crate::*;

pub enum Operator<'a> {
	Binary(BinaryOperator),
	Unary(UnaryOperator),
	Comparison {
		operator: ComparisonOperator,
		labels_count: &'a mut u32,
	},
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

impl<'a> Operator<'a> {
	pub fn new(s: String, labels_count: &'a mut u32, original_line_nb: u16) -> Result<Self> {
		match s.as_str() {
			"add" => Ok(Operator::Binary(BinaryOperator::Add)),
			"sub" => Ok(Operator::Binary(BinaryOperator::Sub)),
			"neg" => Ok(Operator::Unary(UnaryOperator::Neg)),
			"eq" => Ok(Operator::Comparison {
				operator: ComparisonOperator::Eq,
				labels_count,
			}),
			"gt" => Ok(Operator::Comparison {
				operator: ComparisonOperator::Gt,
				labels_count,
			}),
			"lt" => Ok(Operator::Comparison {
				operator: ComparisonOperator::Lt,
				labels_count,
			}),
			"and" => Ok(Operator::Binary(BinaryOperator::And)),
			"or" => Ok(Operator::Binary(BinaryOperator::Or)),
			"not" => Ok(Operator::Unary(UnaryOperator::Not)),
			_ => Err(TranslateError::IncorrectCommand(
				s.to_owned(),
				original_line_nb,
			)),
		}

	}

	pub fn into_asm(self) -> String {
		match self {
			Operator::Binary(op) => op.to_asm(),
			Operator::Unary(op) => op.to_asm(),
			Operator::Comparison {
				operator,
				labels_count,
			} => {
				let specific = match operator {
					ComparisonOperator::Eq => "D;JNE\n",
					ComparisonOperator::Gt => "D;JLE\n",
					ComparisonOperator::Lt => "D;JGE\n",
				};
				let false_label = format!("FALSE_{}", labels_count);
				let end_label = format!("END_{}", labels_count);
				let asm = format!(
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
					 {}\n\
					 0;JMP\n\
					 {}\n\
					 @SP\n\
					 A=M-1\n\
					 M=0\n\
					 {}\n",
					false_label, specific, end_label, false_label, end_label
				);
				*labels_count += 1;
				asm
			}
		}
	}
}
