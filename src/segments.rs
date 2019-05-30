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
    Constant,
}

#[derive(Debug)]
pub enum SegmentType {
    Variable(Segment),
    Fixed(Segment),
    Static,
    Constant,
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
            "constant" => Ok(Segment::Constant),
            _ => Err(TranslateError::Error),
        }
    }
}

impl Segment {
    pub fn to_label(self) -> String {
        match self {
            Segment::Local => String::from("LCL"),
            Segment::Argument => String::from("ARG"),
            Segment::This => String::from("THIS"),
            Segment::That => String::from("THAT"),
            _ => String::from(""),
        }
    }

    pub fn get_index(self, index: u16) -> u16 {
        match self {
            Segment::Pointer => 3 + index,
            Segment::Temp => 5 + index,
            _ => 0
        }
    }

    pub fn into_typed(self) -> SegmentType {
        match self {
            Segment::Local | Segment::Argument | Segment::This | Segment::That => {
                SegmentType::Variable(self)
            }
            Segment::Temp | Segment::Pointer => SegmentType::Fixed(self),
            Segment::Static => SegmentType::Static,
            Segment::Constant => SegmentType::Constant,
        }
    }
}
