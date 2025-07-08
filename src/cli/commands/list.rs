use crate::cli::commands::Command;
use crate::utils::enums;
use crate::services::*;

pub struct List {
	help: String
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
		return file_service::list();
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
		Self {
			help: String::from(arg)
		}
	}
}