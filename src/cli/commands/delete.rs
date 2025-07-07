use crate::cli::commands::Command;
use crate::utils::{enums, helper};
use crate::services::file_service;

pub struct Delete {
	file_name: String,
}

impl Command for Delete {
	fn execute (&self) -> enums::Outcome<String> {
		//checks
		
		//execution
		return file_service::delete(&self.file_name);
	}
	
	fn help (&self) -> String {
		"Help".to_string()
	}
}

impl Delete {
	pub fn new(file_path: &str) -> Self {
		let file_path = String::from(file_path);
		let file_name = helper::get_file_name(&file_path);
		
		Self {
			file_name: String::from(file_name),
		}
	}
}