use std::io::{self, Write};
use std::fs;
use crate::utils::enums;

pub fn print (text: &str) {
	print!("{}", text);
	io::stdout().flush().unwrap();
}

pub fn get_input (stored_in: &mut String) {
	io::stdin().read_line(stored_in).expect("Failed to read line");
}

pub fn check_command (command_str: &String) -> enums::Check<Vec<&str>> {
	let command_tokens: Vec<&str> = command_str.trim()
		.split(" ").collect();
		
	let vault = command_tokens[0];
	let command;
	let mut arg;
	
	//full command must start with 'vault'
	if vault != "vault" {
		let msg = String::from("Start all commands with 'vault'");
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
	
	//only defined commands are accepted
	let mut def_cmd = false;
	for cmd in vec!["upload", "list", "read", "delete", "metadata", "register", "login", "logout"] {
		if command == cmd {
			def_cmd = true;
			break;
		}
	}
	if !def_cmd {
		let msg = String::from(format!("{} is an invalid command. Accepted commands are: upload, list, read, delete, & metadata.", command));
		return enums::Check::Invalid(msg);
	}
	
	//set flag for commands with argument
	let mut cmd_with_arg = true;
	
	//these commands must have no arguments except help
	for cmd in vec!["list", "register", "login", "logout"] {
		//unset flag for commands with argument
		if command == cmd {
			cmd_with_arg = false;
		}
		if command == cmd && arg != "" && arg != "help" {
			let msg = String::from(format!("{} command takes no arguments except 'help'", command));
			return enums::Check::Invalid(msg);
		}
	}
	
	//all other commands except list must have an argument
	if cmd_with_arg && (arg == "") {
		let msg = String::from(format!("You have provided no argument for {}", command));
		return enums::Check::Invalid(msg);
	}
	
	return enums::Check::Valid(command_tokens);
}

pub fn get_file_name (path: &String) -> &str {
	path.trim().split("/").collect::<Vec<&str>>().pop().unwrap()
}