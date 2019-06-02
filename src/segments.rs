use crate::*;
use std::string::ToString;

#[derive(Debug)]
pub enum Segment {
    Variable(VariableSegment),
    Fixed(FixedSegment),
    Static,
    Constant,
}

#[derive(Debug)]
pub enum VariableSegment {
    Local,
    Argument,
    This,
    That,
}

#[derive(Debug)]
pub enum FixedSegment {
    Pointer,
    Temp,
}


impl ToString for VariableSegment {
    fn to_string(&self) -> String {
        match self {
            VariableSegment::Local => String::from("LCL"),
            VariableSegment::Argument => String::from("ARG"),
            VariableSegment::This => String::from("THIS"),
            VariableSegment::That => String::from("THAT"),
        }
    }
}

impl VariableSegment {
    pub fn to_asm_push(&self, index: u16) -> String {
        format!(
            "@{}\n\
             D=A\n\
             @{}\n\
             A=D+M\n\
             D=M\n\
             @SP\n\
             M=M+1\n\
             A=M-1\n\
             M=D\n",
            index,
            self.to_string()
        )
    }

    pub fn to_asm_pop(&self, index: u16) -> String {
        format!(
            "@{}\n\
             D=A\n\
             @{}\n\
             D=D+M\n\
             @SP\n\
             AM=M-1\n\
             D=D+M\n\
             A=D-M\n\
             M=D-A\n",
            index,
            self.to_string()
        )
    }
}

impl FixedSegment {
    pub fn get_absolute_index(&self, index: u16) -> u16 {
        match self {
            FixedSegment::Pointer => 3 + index,
            FixedSegment::Temp => 5 + index,
        }
    }

    pub fn to_asm_push(&self, index: u16) -> String {
        format!(
            "@{}\n\
             D=M\n\
             @SP\n\
             M=M+1\n\
             A=M-1\n\
             M=D\n",
            self.get_absolute_index(index)
        )
    }

    pub fn to_asm_pop(&self, index: u16) -> String {
        format!(
            "@{}\n\
             D=A\n\
             @SP\n\
             AM=M-1\n\
             D=D+M\n\
             A=D-M\n\
             M=D-A\n",
            self.get_absolute_index(index)
        )
    }
}

impl Segment {
    pub fn new(s: &str, original_line_nb: u16) -> Result<Self> {
        match s {
            "local" => Ok(Segment::Variable(VariableSegment::Local)),
            "argument" => Ok(Segment::Variable(VariableSegment::Argument)),
            "this" => Ok(Segment::Variable(VariableSegment::This)),
            "that" => Ok(Segment::Variable(VariableSegment::That)),
            "temp" => Ok(Segment::Fixed(FixedSegment::Temp)),
            "pointer" => Ok(Segment::Fixed(FixedSegment::Pointer)),
            "static" => Ok(Segment::Static),
            "constant" => Ok(Segment::Constant),
            other => Err(TranslateError::IncorrectCommand(
                other.to_owned(),
                original_line_nb,
            )),
        }
    }
}