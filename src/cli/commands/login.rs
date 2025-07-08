use crate::cli::commands::Command;
use crate::utils::enums;
use crate::services::user_service;

pub struct Login {
	help: String
}

impl Command for Login {
	fn execute (&self) -> enums::Outcome<String> {
		/*================ CHECKS =================*/
		//check if help is needed
		if self.help == "help" {
			let msg = self.help();
			return enums::Outcome::Success(msg);
		}
		
		/*================ EXECUTION =================*/
		let email = String::new();
		let password = String::new();
		
		return user_service::login(&email, &password);
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The login command follows this structure:
		
		vault login
		
		This command does not take any arguments except 'help'
		
		On command execution, you will be required to provide a valid username and password in order to gain accessc
		"#)
	}
}

impl Login {
	pub fn new(arg: &str) -> Self {
		Self {
			help: String::from(arg)
		}
	}
}