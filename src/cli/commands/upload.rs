use crate::cli::commands::Command;
use crate::utils::enums;
use crate::services::file_service;

pub struct Upload {
	file_path: String,
}

impl Command for Upload {
	fn execute (&self) -> enums::Outcome<String> {
		//checks
		
		//execution
		return file_service::upload(&self.file_path);
	}
	
	fn help (&self) -> String {
		"Help".to_string()
	}
}

impl Upload {
	pub fn new(file_path: &str) -> Self {
		Self {
			file_path: String::from(file_path),
		}
	}
}