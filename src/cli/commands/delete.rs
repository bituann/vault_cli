use crate::cli::commands::Command;
use crate::utils::{enums, helper};
use crate::services::*;

pub struct Delete {
	file_name: String,
	user: String,
}

impl Command for Delete {
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
		return file_service::delete(&self.file_name, &self.user);
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The upload command follows this structure:
		
		vault delete file_name
		
		where 'file_name' is the name of the file you uploaded.
		"#)
	}
}

impl Delete {
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