use crate::cli::commands::Command;
use crate::utils::*;
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
		let mut email = String::new();
		let mut password = String::new();
		
		helper::print("Email: ");
		helper::get_input(&mut email);
		
		helper::print("Password: ");
		helper::get_input(&mut password);
		
		return user_service::login(&email, &password);
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The login command follows this structure:
		
		vault login
		
		This command does not take any arguments except 'help'
		
		On command execution, you will be required to provide a valid username and password in order to gain access.
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