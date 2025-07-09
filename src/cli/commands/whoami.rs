use crate::cli::commands::Command;
use crate::utils::enums;
use crate::services::user_service;

pub struct Whoami {
	help: String
}

impl Command for Whoami {
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
		return user_service::whoami();
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The whoami command follows this structure:
		
		vault whoami
		
		It shows the email of the current logged in user. This command does not take any arguments except 'help'
		"#)
	}
}

impl Whoami {
	pub fn new(arg: &str) -> Self {
		Self {
			help: String::from(arg)
		}
	}
}