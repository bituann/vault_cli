use crate::cli::commands::*;
use crate::utils::enums;
use std::collections::HashMap;

pub fn route_command (command: &str, arg: &str) -> enums::Outcome<String> {
	match command {
		"upload" => upload::Upload::new(arg).execute(),
		"list" => list::List::new().execute(),
		"read" => read::Read::new(arg).execute(),
		"delete" => delete::Delete::new(arg).execute(),
		_ => enums::Outcome::Fail(String::from("Invalid command")),
	}
}