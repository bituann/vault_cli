use crate::cli::commands::Command;
use crate::utils::enums;
use crate::services::file_service;

pub struct List;

impl Command for List {
	fn execute (&self) -> enums::Outcome<String> {
		/*================ EXECUTION =================*/
		return file_service::list();
	}
	
	fn help (&self) -> String {
		String::from(r#"
		The list command follows this structure:
		
		vault list
		
		This command does not take any argumentsc
		"#)
	}
}

impl List {
	pub fn new() -> Self {
		Self
	}
}