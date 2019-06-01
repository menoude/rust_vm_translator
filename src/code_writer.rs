use crate::*;

use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Write;

use std::fs::File;
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

    pub fn write_arithmetic(&mut self, command: String) -> Result<()> {
        let comment = format!("// {}\n", command);
        let operation = Operator::try_from(command)?;
        let asm_command = match operation {
            Operator::Binary(op) => op.to_asm(),
            Operator::Unary(op) => op.to_asm(),
            Operator::Compararison(op) => {
                let specific = match op {
                    ComparisonOperator::Eq => "D;JNE\n",
                    ComparisonOperator::Gt => "D;JLE\n",
                    ComparisonOperator::Lt => "D;JGE\n",
                };
                let false_label = format!("FALSE_{}", self.labels_count);
                let end_label = format!("END_{}", self.labels_count);

                self.labels_count += 1;
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
                     {}\n\
                     0;JMP\n\
                     {}\n\
                     @SP\n\
                     A=M-1\n\
                     M=0\n\
                     {}\n",
                    false_label, specific, end_label, false_label, end_label
                )
            }
        };
        self.buf.write_all(comment.as_bytes())?;
        write!(self.buf, "{}", asm_command)?;
        Ok(())
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

        let instructions = match command {
            CommandType::Push => self.push_instructions(segment, index)?,
            CommandType::Pop => self.pop_instructions(segment, index)?,
            _ => panic!(),
        };
        let comment = format!("// {} {} {}\n", command, arg_1, arg_2);
        self.buf.write_all(comment.as_bytes())?;
        self.buf.write_all(instructions.as_bytes())?;
        Ok(())
    }

    pub fn push_instructions(&mut self, segment: Segment, index: u16) -> Result<String> {
        let asm_command = match segment {
            Segment::Variable(var_seg) => var_seg.to_asm_push(index),
            Segment::Fixed(fixed_seg) => fixed_seg.to_asm_push(index),
            Segment::Static => format!(
                "@{}.{}\n\
                 D=M\n\
                 @SP\n\
                 M=M+1\n\
                 A=M-1\n\
                 M=D\n",
                self.name, index
            ),
            Segment::Constant => format!(
                "@{}\n\
                 D=A\n\
                 @SP\n\
                 M=M+1\n\
                 A=M-1\n\
                 M=D\n",
                index
            ),
        };
        Ok(asm_command)
    }

    pub fn pop_instructions(&mut self, segment: Segment, index: u16) -> Result<String> {
        let asm_command = match segment {
            Segment::Variable(var_seg) => var_seg.to_asm_pop(index),
            Segment::Fixed(fixed_seg) => fixed_seg.to_asm_pop(index),
            Segment::Static => format!(
                "@SP\n\
                 AM=M-1\n\
                 D=M\n\
                 @{}.{}\n\
                 M=D\n",
                self.name, index
            ),
            _ => panic!(),
        };
        Ok(asm_command)
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
