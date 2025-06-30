use std::io::{self, Write};
use crate::utils::enums;

pub fn print (text: &str) {
	print!("{}", text);
	io::stdout().flush().unwrap();
}

pub fn get_input (stored_in: &mut String) {
	io::stdin().read_line(stored_in).expect("Failed to read line");
}

pub fn check_command (command_str: &String) -> enums::Check {
	let command_tokens: Vec<&str> = command_str.trim()
		.split(" ").collect();
		
	let vault = command_tokens[0];
	let command;
	let mut arg;
	
	//full command must start with 'vault'
	if vault != "vault" {
		let msg = String::from("Start all commands with 'vault'\n");
		return enums::Check::Invalid(msg);
	}
	
	//entire command must consist of at least 2 parts and not nore than 3 parts
	if command_tokens.len() < 2 || command_tokens.len() > 3 {
		let msg = String::from("Invalid command");
		return enums::Check::Invalid(msg);
	}
	
	command = command_tokens[1];
	arg = "";
	
	if command_tokens.len() == 3 {
		arg = command_tokens[2];
	}
	
	//command must be all lowercase
	if !command.chars().all(|c| c.is_lowercase()) {
		let msg = String::from(format!("{} is an invalid command. Accepted commands are: upload, list, read, & delete.", command));
		return enums::Check::Invalid(msg);
	}
	
	//list command must have no arguments
	if command == "list" && arg != "" {
		let msg = String::from("list command takes no arguments");
		return enums::Check::Invalid(msg);
	}
	
	return enums::Check::Valid;
}