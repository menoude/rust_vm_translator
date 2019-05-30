use crate::*;

use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;

use std::convert::TryInto;
use std::ops::Deref;
use std::str::FromStr;
pub struct CodeWriter {
    name: String,
    buf: BufWriter<File>,
    labels_count: u32,
    original_line_nb: u16,
}

impl CodeWriter {
    pub fn new(file_data: &FileData) -> Result<CodeWriter> {
        println!(
            "Translate {} into {}",
            file_data.original_path.display(),
            file_data.destination_path.display()
        );
        let f = File::create(file_data.destination_path.deref())?;
        let code_writer = CodeWriter {
            buf: BufWriter::new(f),
            labels_count: 0,
            name: file_data.destination_name.clone(),
            original_line_nb: 0,
        };
        Ok(code_writer)
    }

    pub fn set_original_line_index(&mut self, index: u16) {
        self.original_line_nb = index;
    }

    pub fn write_file_name(&mut self) -> Result<()> {
        let comment_line = format!("// {}\n", self.name);
        self.buf.write_all(comment_line.as_bytes())?;
        Ok(())
    }

    pub fn write_arithmetic(&mut self, command: String) {
        unimplemented!()
    }

    pub fn write_push_or_pop(
        &mut self,
        command: CommandType,
        arg_1: &str,
        arg_2: &str,
    ) -> Result<()> {
        let segment = segments::Segment::try_from(arg_1).or_else(|_| {
            Err(TranslateError::IncorrectCommand(
                arg_1.to_owned(),
                self.original_line_nb,
            ))
        })?;
        let index = u16::from_str(arg_2)?;
        let segment_type = segment.into_typed();

        let instructions = match command {
            CommandType::Push => self.push_instructions(segment_type, index)?,
            CommandType::Pop => self.pop_instructions(segment_type, index)?,
            _ => String::new(),
        };
        let comment = format!("// {} {} {}\n", command, arg_1, arg_2);
        self.buf.write_all(comment.as_bytes())?;
        self.buf.write_all(instructions.as_bytes())?;
        Ok(())
    }

    pub fn push_instructions(&mut self, segment_type: SegmentType, index: u16) -> Result<String> {
        let asm_command = match segment_type {
            SegmentType::Variable(segment) => format!(
                "@{}\n\
                 D=A\n\
                 @{}\n\
                 A=D+M\n\
                 D=M\n\
                 @SP\n\
                 M=M+1\n\
                 A=M-1\n\
                 M=D",
                index,
                segment.to_label()
            ),
            SegmentType::Fixed(segment) => format!(
                "@{}\n\
                 D=M\n\
                 @SP\n\
                 M=M+1\n\
                 A=M-1\n\
                 M=D\n",
                segment.get_index(index)
            ),
            SegmentType::Static => format!(
                "@{}.{}\n\
                 D=M\n\
                 @SP\n\
                 M=M+1\n\
                 A=M-1\n\
                 M=D\n",
                self.name, index
            ),
            SegmentType::Constant => format!(
                "\
                 @{}\n\
                 D=A\n\
                 @SP\n\
                 M=M+1\n\
                 A=M-1\n\
                 M=D\n\
                 ",
                index
            ),
        };
        Ok(asm_command)
    }

    pub fn pop_instructions(&mut self, segment_type: SegmentType, index: u16) -> Result<String> {
        let asm_command = match segment_type {
            SegmentType::Variable(segment) => format!(
                "\
                 @${}\n\
                 D=A\n\
                 @${}\n\
                 D=D+M\n\
                 @SP\n\
                 AM=M-1\n\
                 D=D+M\n\
                 A=D-M\n\
                 M=D-A\n\
                 ",
                segment.to_label(),
                index
            ),
            SegmentType::Fixed(segment) => format!(
                "\
                 @${}\n\
                 D=A\n\
                 @SP\n\
                 AM=M-1\n\
                 D=D+M\n\
                 A=D-M\n\
                 M=D-A\n\
                 ",
                segment.get_index(index)
            ),
            SegmentType::Static => format!(
                "\
                 @SP\n\
                 AM=M-1\n\
                 D=M\n\
                 @{}.{}\n\
                 M=D\n\
                 ",
                self.name, index
            ),
            _ => String::new(),
        };
        Ok(asm_command)
    }
    // if let Some(label) = map_variable_segments(&segment) {
    //     asm_command = format!(
    //         "\
    //          @${}\n\
    //          D=A\n\
    //          @${}\n\
    //          D=D+M\n\
    //          @SP\n\
    //          AM=M-1\n\
    //          D=D+M\n\
    //          A=D-M\n\
    //          M=D-A\n\
    //          ",
    //         index, label
    //     );
    // } else if let Some(mut address) = map_fixed_segments(&segment) {
    //     address += index;
    //     asm_command = format!(
    //         "\
    //     @${}\n
    //     D=A\n\
    //     @SP\n\
    //     AM=M-1\n\
    //     D=D+M\n\
    //     A=D-M\n\
    //     M=D-A\n\
    //     ",
    //         address
    //     );
    // } else {
    //     let label = format!("{}.{}", self.name, index);
    //     asm_command = format!(
    //         "\
    //          @SP\n\
    //          AM=M-1\n\
    //          D=M\n\
    //          @{}\n\
    //          M=D\n\
    //          ",
    //         label
    //     );
    // }
    // let text = format!("// pop {} {}\n{}", segment, index, asm_command);
    // self.buf.write(text.as_bytes()).unwrap();
}

// #[cfg(test)]
// mod test {
//     use std::path::Path;

//     #[test]
//     fn test_writer() {
//         let mut writer = super::CodeWriter::new(Path::new("hello")).unwrap();
//         let file_name = "testFile";
//         writer.set_file_name(&file_name);
//         assert_eq!(file_name, "testFile");
//         writer.write_push(format!("local"), 4);
//         writer.write_push(format!("static"), 4);
//         writer.write_pop(format!("static"), 3);
//     }
// }
