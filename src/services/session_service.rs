use std::io::{Read, Write};
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Session {
	pub id: String,
	pub owner: String,
}

const session_path: &str = "./src/storage/.vault-session";

pub fn create_session (email: String) -> bool {
	let session = Session {
		id: Uuid::new_v4().to_string(),
		owner: email,
	};
	
	let session_json = serde_json::to_string(&session).unwrap();
	
	let mut session_file;
	
	match fs::File::create(session_path) {
		Ok(file) => session_file = file,
		Err(_) => return false,
	}
	
	session_file.write(session_json.as_bytes());
	
	return true;
}

pub fn get_current_session () -> Session {
	let mut session_file;
	
	match fs::File::open(session_path) {
		Ok(file) => session_file = file,
		Err(_) => return Session { 
			id: String::from(""), 
			owner: String::from(""),
		},
	}
	
	let mut session: Session;
	
	match serde_json::from_reader(session_file) {
		Ok(from) => session = from,
		Err(_) => return Session {
			id: String::from(""),
			owner: String::from(""),
		},
	}
	
	return session;
}

pub fn delete_session () -> bool {
	match fs::File::create(session_path) {
		Ok(_) => return true,
		Err(_) => return false,
	}
}