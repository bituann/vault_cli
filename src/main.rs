use std::io;
use utils::*;

mod cli;
mod services;
mod utils;

fn main() {

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
			enums::Check::Valid => {
				let input_tokens: Vec<&str> = input.trim()
					.split(" ").collect();
				let command = input_tokens[1];
				let mut arg = "";
		
				if input_tokens.len() == 3 {
					arg = input_tokens[2];
				}
				cli::command_router::route_command(command, arg);
			}
			//if command is invalid
			enums::Check::Invalid(msg) => println!("{}", msg),
		}
    }
}
