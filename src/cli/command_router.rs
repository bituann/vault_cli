use crate::cli::commands;
use crate::utils::enums;

pub fn route_command (command: &str, arg: &str) -> enums::Outcome {
	match command {
		"upload" => commands::upload(arg),
		"list" => commands::list(),
		"read" => commands::read(arg),
		"delete" => commands::delete(arg),
		_ => enums::Outcome::Fail("Invalid command"),
	}
}