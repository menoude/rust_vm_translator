
use std::convert::From;
use std::ffi::OsStr;
use std::io::BufWriter;
use std::path::PathBuf;


pub mod code_writer;
// pub mod constants;
pub mod commands;
pub mod error;
pub mod parser;
pub mod segments;

use code_writer::*;
use commands::*;
use error::TranslateError;
use parser::*;
use segments::*;

type Result<T> = std::result::Result<T, TranslateError>;

#[derive(Debug, Clone)]
pub struct FileData {
	destination_name: String,
	original_path: PathBuf,
	destination_path: PathBuf,
}

/// Get file information for a file path
fn get_file_data(original_path: PathBuf) -> Result<FileData> {
	let destination_path: PathBuf = original_path.with_extension("asm");
	let destination_name = destination_path
		.file_name()
		.ok_or_else(|| error::TranslateError::WrongFilePath(destination_path.clone()))?
		.to_os_string()
		.into_string()
		.unwrap();
	let data = FileData {
		destination_name,
		original_path,
		destination_path,
	};
	Ok(data)
}

/// Returns a vector with every FileFata objects
fn get_data(path: &str) -> Result<Vec<FileData>> {
	let to_check = PathBuf::from(path);
	let vm_extension = Some(OsStr::new("vm"));
	let mut data = Vec::new();
	if to_check.is_dir() {
		for entry in to_check.read_dir()? {
			if let Ok(entry) = entry {
				let entry_path = entry.path();
				if entry_path.extension() == vm_extension {
					data.push(get_file_data(entry_path)?);
				}
			}
		}
		if data.is_empty() {
			return Err(TranslateError::NoVmFile(to_check));
		}
	} else {
		if to_check.extension() != vm_extension {
			return Err(TranslateError::WrongFilePath(to_check));
		}
		data.push(get_file_data(PathBuf::from(path))?);
	}
	Ok(data)
}

pub fn translate(raw_path: &str) -> Result<()> {
	let files_data = get_data(raw_path)?;
	for file in &files_data {
		let content = read_content(file)?;
		let mut code_writer = CodeWriter::new(file)?;
		code_writer.write_file_name()?;
		let mut parser = Parser::new(&content)?;
		while parser.advance() {
			code_writer.set_original_line_index(parser.get_line_index());
			let command_type = parser.command_type()?;
			match command_type {
				CommandType::Arithmetic(op) => code_writer.write_arithmetic(op),
				op @ CommandType::Push | op @ CommandType::Pop => {
					code_writer.write_push_or_pop(op, parser.arg_1()?, parser.arg_2()?)?
				}
				_ => {}
			}
		}
	}
	Ok(())
}
