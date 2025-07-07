use crate::cli::commands::Command;
use crate::utils::{enums, helper};
use crate::services::file_service;

pub struct Metadata {
	file_name: String,
}

impl Command for Metadata {
	fn execute (&self) -> enums::Outcome<String> {
		/*================ CHECKS =================*/
		//check if help is needed
		if self.file_name == "help" {
			let msg = self.help();
			return enums::Outcome::Success(msg);
		}
		
		//check if file exists
		if !file_service::file_exists(&self.file_name) {
			let msg = String::from("File does not exist. Check if the file name is correct, or use the list command to check available files");
			return enums::Outcome::Fail(msg);
		}
		
		/*================ EXECUTION =================*/
		return file_service::metadata(&self.file_name);
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The metadata command follows this structure:
		
		vault metadata file_name
		
		where 'file_name' is the name of the file when the file was uploaded.
		"#)
	}
}

impl Metadata {
	pub fn new(file_path: &str) -> Self {
		let file_path = String::from(file_path);
		let file_name = helper::get_file_name(&file_path);
		
		Self {
			file_name: String::from(file_name),
		}
	}
}