use std::io::{self, Write};

pub fn print (text: &str) {
	print!("{}", text);
	io::stdout().flush().unwrap();
}

pub fn get_input (stored_in: &mut String) {
	io::stdin().read_line(stored_in).expect("Failed to read line");
}