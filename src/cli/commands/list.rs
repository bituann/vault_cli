use crate::cli::commands::Command;
use crate::utils::enums;
use crate::services::file_service;

pub struct List;

impl Command for List {
	fn execute (&self) -> enums::Outcome<String> {
		//checks
		
		//execution
		return file_service::list();
		// return enums::Outcome::Success("hey".to_string());
	}
	
	fn help (&self) -> String {
		"Help".to_string()
	}
}

impl List {
	pub fn new() -> Self {
		Self
	}
}