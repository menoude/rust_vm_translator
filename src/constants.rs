use std::convert::TryFrom;

#[derive(Debug)]
pub enum ArithmeticCommand {
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

impl Segment {
    fn to_str(&self, name: &str, index: u16) -> String {
        match self {
            Segment::Local => format!("LCL"),
            Segment::Argument => format!("ARG"),
            Segment::This => format!("THIS"),
            Segment::That => format!("THAT"),
            Segment::Temp => format!("{}", 5 + index),
            Segment::Pointer => format!("{}", 3 + index),
            Segment::Static => format!("{}.{}", name, index),
        }
    }
}

impl Pushable for VariableSeg {
    fn push_asm_command(&self, index: u16) -> String {
        format!(
            "\
@{}
D=A
@{}
A=D+M
D=M
@SP
M=M+1
A=M-1
M=D",
            index,
            self.to_str()
        )
    }
}

impl FixedSeg {
    pub fn push_asm_command(&self, index: u16) -> String {
        format!(
            "\
@{}
D=M
@SP
M=M+1
A=M-1
M=D",
            index + self.to_mem_index()
        )
    }
}

#[derive(Copy, Clone)]
pub enum Command {
    Add,
    Sub,
    And,
    Or,
}

impl Command {
    pub fn add_sub_and_or(self) -> String {
        match self {
            Command::Add => format!("M=D+M\n"),
            Command::Sub => format!("M=M-D\n"),
            Command::And => format!("M=D&M\n"),
            Command::Or => format!("M=D|M\n"),
        }
    }
}
