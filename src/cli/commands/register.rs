use crate::cli::commands::Command;
use crate::utils::*;
use crate::services::user_service;

pub struct Register {
	help: String
}

impl Command for Register {
	fn execute (&self) -> enums::Outcome<String> {
		/*================ CHECKS =================*/
		//check if help is needed
		if self.help == "help" {
			let msg = self.help();
			return enums::Outcome::Success(msg);
		}
		
		/*=============== COLLECT DETAILS ================*/
		let mut email = String::new();
		let mut password = String::new();
		
		helper::print("Email: ");
		helper::get_input(&mut email);
		
		helper::print("Password: ");
		helper::get_input(&mut password);
		
		/*================ CHECKS =================*/
		if user_service::check_user(&email) {
			let msg = String::from("An account with that email already exists");
			return enums::Outcome::Fail(msg);
		}
		
		/*================ EXECUTION =================*/
		return user_service::register(email, password);
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The list command follows this structure:
		
		vault register
		
		This command does not take any arguments except 'help'
		
		After executing the command, you will be prompted to enter a username and a password, both of which will serve as your login credentials henceforth
		"#)
	}
}

impl Register {
	pub fn new(arg: &str) -> Self {
		Self {
			help: String::from(arg)
		}
	}
}