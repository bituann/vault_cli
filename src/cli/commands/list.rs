use crate::utils::enums;

pub fn list () -> enums::Outcome {
	let mut outcome_msg;
	
	outcome_msg = String::from("Files listed successfully");
	return enums::Outcome::Success(outcome_msg);
}