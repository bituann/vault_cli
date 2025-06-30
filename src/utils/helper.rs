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
	
	if vault != "vault" {
		let message = String::from("Start all commands with 'vault'\n");
		return enums::Check::Invalid(message);
	}
	
	if command_tokens.len() < 2 || command_tokens.len() > 3 {
		let message = String::from("Invalid command");
		return enums::Check::Invalid(message);
	}
	
	command = command_tokens[1];
	arg = "";
	
	if command_tokens.len() == 3 {
		arg = command_tokens[2];
	}
	return enums::Check::Valid;
}