use std::error::Error;
use std::path::PathBuf;

// pub mod code_writer;
pub mod parser;

#[derive(Debug)]
pub struct FileData {
	destination_name: String,
	original_path: PathBuf,
	destination_path: PathBuf,
}
/// Get file information for a file path
fn get_file_data(original_path: PathBuf) -> FileData {
	let destination_path: PathBuf = original_path.with_extension("asm");
	FileData {
		destination_name: destination_path
			.file_name()
			.unwrap()
			.to_os_string()
			.into_string()
			.unwrap(),
		original_path,
		destination_path,
	}
}

/// Returns a vector with every FileFata objects
fn get_data(path: &str) -> Result<Vec<FileData>, Box<dyn Error>> {
	let to_check = PathBuf::from(path);
	let mut data = Vec::new();
	if to_check.is_dir() {
		for entry in to_check.read_dir()? {
			if let Ok(entry) = entry {
				data.push(get_file_data(entry.path()));
			}
		}
	} else {
		data.push(get_file_data(PathBuf::from(path)));
	}
	Ok(data)
}

pub fn translate(raw_path: String) -> Result<(), Box<dyn std::error::Error>> {
	let files_data = get_data(&raw_path)?;
	for file in files_data {
		// let code_writer = code_writer::CodeWriter::new();
		let content = parser::read_content(file)?;
		let mut parser = parser::Parser::new(&content)?;
		println!("{:?}", parser);
		while parser.has_more_commands() {
			parser.advance();
		}
	}
	Ok(())
}
