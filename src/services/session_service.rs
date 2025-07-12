use std::io::{Read, Write};
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use uuid::Uuid;
use redis::{Connection, TypedCommands, HashFieldExpirationOptions, SetExpiry};

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
	
	let mut con = get_redis_connection();
	
	con.set_ex("session_id", session.id , 60);
	con.set_ex("session_owner", session.owner, 60);
	
	return true;
}

pub fn get_current_session () -> Session {
	let mut con = get_redis_connection();
	
	let session = Session {
		id: match con.get("session_id").unwrap() {
			Some(val) => val,
			None => String::from("")
		},
		owner: match con.get("session_owner").unwrap() {
			Some(val) => val,
			None => String::from("")
		}
	};
	
	return session;
}

pub fn delete_session () -> bool {
	let mut con = get_redis_connection();
	
	match con.del(&["session_id", "session_owner"]) {
		Ok(_) => return true,
		Err(_) => return false
	}
}




fn get_redis_connection () -> redis::Connection {
	let client = redis::Client::open("redis://127.0.0.1").unwrap();
	
	return client.get_connection().unwrap();
}