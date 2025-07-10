use crate::cli::commands::Command;
use crate::utils::enums;
use crate::services::*;

pub struct List {
	help: String,
	user: String
}

impl Command for List {
	fn execute (&self) -> enums::Outcome<String> {
		/*================ CHECKS =================*/
		//user authentication
		if !user_service::authenticate() {
			let msg = String::from("Please log in to use this command");
			return enums::Outcome::Fail(msg);
		}
		
		//check if help is needed
		if self.help == "help" {
			let msg = self.help();
			return enums::Outcome::Success(msg);
		}
		
		/*================ EXECUTION =================*/
		let mut files;
		
		match file_service::list() {
			enums::Outcome::Success(rec) => files = rec,
			enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg),
		}
		
		//create file list starting with header
		let mut file_list = format!("{:40} | {:15} | {:10}\n", "id", "name", "size");
	
		for file in files {
			if file.user_id == self.user {
				//add each file to file list
				file_list = format!("{}{:40} | {:15} | {:10}\n", file_list, file.file_id, file.file_name, file.file_size);
			}
		}
		
		let file_list = String::from(file_list);
		return enums::Outcome::Success(file_list);
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The list command follows this structure:
		
		vault list
		
		This command does not take any arguments except 'help'
		"#)
	}
}

impl List {
	pub fn new(arg: &str) -> Self {
		let session = session_service::get_current_session();
		Self {
			help: String::from(arg),
			user: String::from(session.owner)
		}
	}
}