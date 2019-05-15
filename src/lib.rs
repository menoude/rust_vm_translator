use std::path::Path;

// pub mod code_writer;
pub mod parser;

fn get_file_path(file_path: &str) -> Path {
	let resolved = Path::new(file_path);
	println!("{}", resolved);
	resolved
}

pub fn translate(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
	// let original_path = get_file_path(&file_path);
	// println!("{}", original_path);
	get_file_path(&file_path);
	Ok(())
}
