use crate::utils::enums;

pub fn upload (file_path: &str) -> enums::Outcome {
	let mut outcome_msg;
	
	let file_id = 0;
	
	outcome_msg = String::from(format!("File has been uploaded. File ID is: {}", file_id));
	return enums::Outcome::Success(outcome_msg);
}