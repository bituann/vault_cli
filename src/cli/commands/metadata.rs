use crate::cli::commands::Command;
use crate::utils::{enums, helper};
use crate::services::*;

pub struct Metadata {
	file_name: String,
	user: String
}

impl Command for Metadata {
	fn execute (&self) -> enums::Outcome<String> {
		/*================ CHECKS =================*/
		//user authentication
		if !user_service::authenticate() {
			let msg = String::from("Please log in to use this command");
			return enums::Outcome::Fail(msg);
		}
		
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
		let mut files;
		
		match file_service::metadata(&self.file_name) {
			enums::Outcome::Success(data) => files = data,
			enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg)
		}
		
		files.retain(|file| file.user_id == self.user);
		
		let metadata = format!("{:#?}", files[0]);
		return enums::Outcome::Success(metadata);
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
		
		let session = session_service::get_current_session();
		
		Self {
			file_name: String::from(file_name),
			user: session.owner,
		}
	}
}