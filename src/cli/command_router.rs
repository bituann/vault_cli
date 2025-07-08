use crate::cli::commands::*;
use crate::utils::enums;

pub fn route_command (command: &str, arg: &str) -> enums::Outcome<String> {
	match command {
		"upload" => upload::Upload::new(arg).execute(),
		"list" => list::List::new(arg).execute(),
		"read" => read::Read::new(arg).execute(),
		"delete" => delete::Delete::new(arg).execute(),
		"metadata" => metadata::Metadata::new(arg).execute(),
		"register" => register::Register::new(arg).execute(),
		"login" => login::Login::new(arg).execute(),
		_ => enums::Outcome::Fail(String::from("Invalid command")),
	}
}