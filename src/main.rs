use std::io;
use utils::helper;

mod cli;
mod services;
mod utils;

fn main() {

	loop {
		let mut input = String::new();
	   
		helper::print("~$ ");
	   
		io::stdin().read_line(&mut input)
			.expect("Failed to read input");
			
		if input.trim() == "exit" {
			break;
		}
			
		let input_tokens: Vec<&str> = input.trim()
			.split(" ").collect();
			
		let prefix = input_tokens[0];
		
		if prefix != "vault" {
			println!("Start all commands with 'vault'\n");
			continue;
		}
			
		if input_tokens.len() < 2 || input_tokens.len() > 3 {
			println!("Invalid command.\n");
			continue;
		}
			
		let command = input_tokens[1];
		let arg = input_tokens[2];
		
		println!("End");
    }
}
