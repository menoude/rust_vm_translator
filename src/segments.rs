use crate::*;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub enum Segment {
    Local,
    Argument,
    This,
    That,
    Temp,
    Pointer,
    Static,
}

impl TryFrom<&str> for Segment {
    type Error = TranslateError;
    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        match s {
            "local" => Ok(Segment::Local),
            "argument" => Ok(Segment::Argument),
            "this" => Ok(Segment::This),
            "that" => Ok(Segment::That),
            "temp" => Ok(Segment::Temp),
            "pointer" => Ok(Segment::Pointer),
            "static" => Ok(Segment::Static),
            s => Err(TranslateError::IncorrectCommand(s.to_owned())),
        }
    }
}

impl Segment {
    fn to_label(self) -> String {
        match self {
            Segment::Local => String::from("LCL"),
            Segment::Argument => String::from("ARG"),
            Segment::This => String::from("THIS"),
            Segment::That => String::from("THAT"),
            Segment::Temp => String::from(""),
            Segment::Pointer => String::from(""),
            Segment::Static => String::from(""),
        }
    }
}