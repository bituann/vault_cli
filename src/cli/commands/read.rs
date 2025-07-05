use crate::cli::commands::Command;
use crate::utils::enums;
use crate::services::file_service;

pub struct Read {
	file_id: String,
}

impl Command for Read {
	fn execute (&self) -> enums::Outcome<String> {
		//checks
		
		//execution
		return file_service::read(&self.file_id);
	}
	
	fn help (&self) -> String {
		"Help".to_string()
	}
}

impl Read {
	pub fn new(file_id: &str) -> Self {
		Self {
			file_id: String::from(file_id),
		}
	}
}