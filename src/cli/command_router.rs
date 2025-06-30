use crate::cli::commands;
use crate::utils::enums;

pub fn route_command (command: &str, arg: &str) -> enums::Outcome {
	match command {
		"upload" => commands::upload::upload(arg),
		"list" => commands::list::list(),
		"read" => commands::read::read(arg),
		"delete" => commands::delete::delete(arg),
		_ => enums::Outcome::Fail(String::from("Invalid command")),
	}
}