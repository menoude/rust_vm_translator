use crate::*;

use std::convert::TryFrom;
use std::fs::File;
use std::ops::Deref;
use std::str::FromStr;
use std::io::Write;

pub struct CodeWriter {
    name: String,
    buf: BufWriter<File>,
    labels_count: u32,
}

impl CodeWriter {
    pub fn new(file_data: &FileData) -> Result<CodeWriter> {
        let f = File::create(file_data.destination_path.deref())?;
        let code_writer = CodeWriter {
            buf: BufWriter::new(f),
            labels_count: 0,
            name: file_data.destination_name.clone(),
        };
        Ok(code_writer)
    }

    pub fn write_file_name(&mut self) -> Result<()> {
        let comment_line = format!("// {}\n", self.name);
        self.buf.write(comment_line.as_bytes())?;
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
        let segment = segments::Segment::try_from(arg_1)?;
        let index = u16::from_str(arg_2)?;
        match command {
            CommandType::Push => self.write_push(segment, index)?,
            CommandType::Pop => self.write_pop(segment, index)?,
            _ => {}
        }
        Ok(())
    }

    pub fn write_push(&mut self, segment: Segment, index: u16) -> Result<()> {
        println!("wrote a push");
        Ok(())
        // let asm_command: String;

        // if let Some(label) = map_variable_segments(&segment) {
        //     // asm_command = ;
        // } else if let Some(mut address) = map_fixed_segments(&segment) {
        //     address += index;
        // // asm_command = ;
        // } else if segment == "static" {
        //     let label = format!("{}.{}", self.name, index);
        //     asm_command = format!(
        //         "\
        //          @{}\n\
        //          D=M\n\
        //          @SP\n\
        //          M=M+1\n\
        //          A=M-1\n\
        //          M=D\n\
        //          ",
        //         label
        //     );
        // } else {
        //     asm_command = format!(
        //         "\
        //          @{}\n\
        //          D=A\n\
        //          @SP\n\
        //          M=M+1\n\
        //          A=M-1\n\
        //          M=D\n\
        //          ",
        //         index
        //     );
        // }
        // let text = format!("// push {} {}\n{}", segment, index, asm_command);
        // self.buf.write(text.as_bytes()).unwrap();
    }

    pub fn write_pop(&mut self, segment: Segment, index: u16) -> Result<()> {
        println!("wrote a pop");
        Ok(())
        // let asm_command: String;

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
