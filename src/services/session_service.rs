use std::io::{Read, Write};
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Session {
	id: String,
	email: String,
}

const session_path: &str = "./src/storage/.vault-session";

pub fn create_session (email: String) -> bool {
	let session = Session {
		id: Uuid::new_v4().to_string(),
		email,
	};
	
	let session_json = serde_json::to_string(&session).unwrap();
	
	// let session_path = "./src/storage/.vault-session"
	
	let mut session_file;
	
	match fs::File::create(session_path) {
		Ok(file) => session_file = file,
		Err(_) => return false,
	}
	
	session_file.write(session_json.as_bytes());
	
	return true;
}

pub fn get_current_session () -> Session {
	let session_file = fs::File::open(session_path).unwrap();
	let session: Session = serde_json::from_reader(session_file).unwrap();
	
	return session;
}

pub fn delete_session () -> bool {
	match fs::File::create(session_path) {
		Ok(_) => return true,
		Err(_) => return false,
	}
}