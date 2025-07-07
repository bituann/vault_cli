use std::env;
use crate::cli;
use crate::utils::*;

pub fn run() {
	loop {
		let mut input = String::new();
	   
		helper::print("~$ ");
		
		helper::get_input(&mut input);
			
		if input.trim() == "exit" {
			break;
		}
			
		let valid_cmd = helper::check_command(&input);
		
		match valid_cmd {
			//if command is valid
			enums::Check::Valid(input_tokens) => {
				let command = input_tokens[1];
				let mut arg = "";
		
				if input_tokens.len() == 3 {
					arg = input_tokens[2];
				}
				let result = cli::command_router::route_command(command, arg);
				
				match result {
					enums::Outcome::Success(msg) => println!("{}", msg),
					enums::Outcome::Fail(msg) => println!("{}", msg),
				}
			}
			//if command is invalid
			enums::Check::Invalid(msg) => println!("{}", msg),
		}
    }
}