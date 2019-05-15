// enum segment

trait Pushable {
    fn push_asm_command(&self, index: u16) -> String;
}

#[derive(Copy, Clone)]
pub enum Segment {
    Local,
    Argument,
    This,
    That,
    Temp,
    Pointer,
    Static,
}

impl Segment {
    fn new(segment: &str) -> Option<Self> {
        match segment {
            "local" => Some(Segment::Local),
            "argument" => Some(Segment::Argument),
            "this" => Some(Segment::This),
            "that" => Some(Segment::That),
            "temp" => Some(Segment::Temp),
            "pointer" => Some(Segment::Pointer),
            "static" => Some(Segment::Static),
            _ => None,
        }
    }

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
