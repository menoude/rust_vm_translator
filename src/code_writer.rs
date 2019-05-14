use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use super::constants::{Command, FixedSeg, VariableSeg};

pub struct CodeWriter {
    name: String,
    buf: BufWriter<File>,
    labels_count: u32,
}

impl CodeWriter {
    pub fn new(path: &Path) -> std::io::Result<CodeWriter> {
        let f = File::create(path)?;
        let code_writer = CodeWriter {
            buf: BufWriter::new(f),
            labels_count: 0,
            name: String::new(),
        };
        Ok(code_writer)
    }

    pub fn set_file_name(&mut self, name: &str) {
        self.name.push_str(name);
        let comment_line = format!("// {}\n", self.name);
        self.buf.write(comment_line.as_bytes()).unwrap();
    }

    pub fn write_push_or_pop(&mut self, command: String, segment: String, index: u16) {
        if command == format!("C_PUSH") {
            self.write_push(segment, index);
        }
    }

    fn write_push(&mut self, segment: String, index: u16) {
        let asm_command: String;

        if let Some(label) = constants::map_variable_segments(&segment) {
            // asm_command = ;
        } else if let Some(mut address) = constants::map_fixed_segments(&segment) {
            address += index;
        // asm_command = ;
        } else if segment == "static" {
            let label = format!("{}.{}", self.name, index);
            asm_command = format!(
                "\
                 @{}\n\
                 D=M\n\
                 @SP\n\
                 M=M+1\n\
                 A=M-1\n\
                 M=D\n\
                 ",
                label
            );
        } else {
            asm_command = format!(
                "\
                 @{}\n\
                 D=A\n\
                 @SP\n\
                 M=M+1\n\
                 A=M-1\n\
                 M=D\n\
                 ",
                index
            );
        }
        let text = format!("// push {} {}\n{}", segment, index, asm_command);
        self.buf.write(text.as_bytes()).unwrap();
    }

    fn write_pop(&mut self, segment: String, index: u16) {
        let asm_command: String;

        if let Some(label) = constants::map_variable_segments(&segment) {
            asm_command = format!(
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
                index, label
            );
        } else if let Some(mut address) = constants::map_fixed_segments(&segment) {
            address += index;
            asm_command = format!(
                "\
            @${}\n
            D=A\n\
            @SP\n\
            AM=M-1\n\
            D=D+M\n\
            A=D-M\n\
            M=D-A\n\
            ",
                address
            );
        } else {
            let label = format!("{}.{}", self.name, index);
            asm_command = format!(
                "\
                 @SP\n\
                 AM=M-1\n\
                 D=M\n\
                 @{}\n\
                 M=D\n\
                 ",
                label
            );
        }
        let text = format!("// pop {} {}\n{}", segment, index, asm_command);
        self.buf.write(text.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    #[test]
    fn test_writer() {
        let mut writer = super::CodeWriter::new(Path::new("hello")).unwrap();
        let file_name = "testFile";
        writer.set_file_name(&file_name);
        assert_eq!(file_name, "testFile");
        writer.write_push(format!("local"), 4);
        writer.write_push(format!("static"), 4);
        writer.write_pop(format!("static"), 3);
    }
}
