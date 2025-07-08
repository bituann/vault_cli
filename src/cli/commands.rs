pub mod delete;
pub mod list;
pub mod metadata;
pub mod read;
pub mod upload;
pub mod register;
pub mod login;

use crate::utils::enums;

pub trait Command {
	fn execute (&self) -> enums::Outcome<String>;
	fn help (&self) -> String;
}