pub mod delete;
pub mod list;
pub mod metadata;
pub mod read;
pub mod upload;
pub mod register;
pub mod login;
pub mod logout;
pub mod whoami;

use crate::utils::enums;

pub trait Command {
	fn execute (&self) -> enums::Outcome<String>;
	fn help (&self) -> String;
}