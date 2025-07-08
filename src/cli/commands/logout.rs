use crate::cli::commands::Command;
use crate::utils::{enums, helper};
use crate::services::user_service;

pub struct Logout {
	help: String,
}

impl Command for Logout {
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
		return user_service::logout();
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The logout command follows this structure:
		
		vault logout
		
		This command logs the current user out of the system.
		"#)
	}
}

impl Logout {
	pub fn new(arg: &str) -> Self {
		Self {
			help: String::from(arg)
		}
	}
}