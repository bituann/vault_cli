use crate::utils::enums;

pub fn read (file_id: &str) -> enums::Outcome {
	let mut outcome_msg;
	
	outcome_msg = String::from(format!("File with ID {} has been read", file_id));
	return enums::Outcome::Success(outcome_msg);
}