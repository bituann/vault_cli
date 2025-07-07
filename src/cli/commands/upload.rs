use crate::cli::commands::Command;
use crate::utils::*;
use crate::services::file_service;

pub struct Upload {
	file_path: String,
}

impl Command for Upload {
	fn execute (&self) -> enums::Outcome<String> {
		let file_name = String::from(helper::get_file_name(&self.file_path));
		
		/*================ CHECKS =================*/
		//check if file exists
		if file_service::file_exists(&file_name) {
			let msg = String::from("File already exists. Select a different file or rename the file");
			return enums::Outcome::Fail(msg);
		}
		
		//exclude certain file extensions
		let excluded_extensions = vec!["exe", "rs"];
		let file_extension = file_name.split(".").collect::<Vec<&str>>().pop().unwrap();
		
		for extension in excluded_extensions {
			if extension == file_extension {
				let msg = format!("Files with the extension '{}' are not permitted. Please select another file", file_extension).to_string();
				return enums::Outcome::Fail(msg);
			}
		}
		
		/*================ EXECUTION =================*/
		return file_service::upload(&self.file_path);
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The upload command follows this structure:
		
		vault upload file_path
		
		where 'file_path' is the path to the file in your device.
		Note that some file extensions are not permitted. The forbidden extensions are: exe and rs
		"#)
	}
}

impl Upload {
	pub fn new(file_path: &str) -> Self {
		Self {
			file_path: String::from(file_path),
		}
	}
}