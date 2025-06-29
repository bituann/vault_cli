use std::io::{self, Write};

pub fn print (text: &str) {
	print!("{}", text);
	io::stdout().flush().unwrap();
}i