use std::io::{Read, Write};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::fs;
use crate::utils::enums;
use crate::services::session_service;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
	id: String,
	email: String,
	password: String,
}

pub fn register (email: String, password: String) -> enums::Outcome<String> {
	//generate user id
	let id = Uuid::new_v4().to_string();
	
	//append id to password
	let mut password = format!("{}{}", password, id);
	
	//hash password
	password = calculate_hash(&password);
	
	//create user struct
	let user = User { id, email, password };
	
	//add user to database
	match add_user(user) {
		enums::Outcome::Success(()) => {
			let msg = String::from("User successfully registered");
			return enums::Outcome::Success(msg);
		},
		enums::Outcome::Fail(msg) => {
			let msg = String::from("Unable to register user");
			return enums::Outcome::Fail(msg);
		}
	}
}

pub fn login (email: &String, password: &String) -> enums::Outcome<String> {
	//get user struct
	let mut user: User;
	
	match get_user(email) {
		enums::Outcome::Success(usr) => user = usr,
		enums::Outcome::Fail(msg) => {
			let msg = String::from("User does not exist");
			return enums::Outcome::Fail(msg);
		}
	}
	
	//append id to password
	let mut password = format!("{}{}", password, user.id);
	
	//hash password
	password = calculate_hash(&password);
	
	
	if user.password == password {
		let mut msg;
		
		if !session_service::create_session(user.email) {
			msg = String::from("Unable to login. Try again");
			return enums::Outcome::Fail(msg);
		}
		
		msg = String::from("Login successful");
		return enums::Outcome::Success(msg);
	} else {
		let msg = String::from("Incorrect password");
		return enums::Outcome::Fail(msg);
	}
}

pub fn logout () -> enums::Outcome<String> {
	if !session_service::delete_session() {
		let msg = String::from("Error logging out. Try exiting the application");
		return enums::Outcome::Fail(msg);
	}
	
	let msg = String::from("Logged out successfully");
	return enums::Outcome::Success(msg);
}

pub fn check_user (email: &String) -> bool {
	match get_user(email) {
		enums::Outcome::Success(_) => return true,
		enums::Outcome::Fail(_) => return false
	}
}

pub fn authenticate () -> bool {
	let session = session_service::get_current_session();
	
	if session.id == "" {
		return false;
	}
	
	return true;
}

pub fn whoami () -> enums::Outcome<String> {
	let session = session_service::get_current_session();
	
	let owner = String::from(session.owner);
	
	return enums::Outcome::Success(owner);
}








//private helper functions
fn calculate_hash<T: Hash> (value: &T) -> String {
	//create hasher
	let mut hasher = DefaultHasher::new();
	
	//hash the value
    value.hash(&mut hasher);
    
    //get hashed value
    let hash = hasher.finish();
    
    return hash.to_string();
}

fn get_users_as_vec () -> enums::Outcome<Vec<User>> {
	let mut users_data;
	let path = "./src/storage/userdata.json";
	
	//open json file
	match fs::File::open(path) {
		Ok(file) => users_data = file,
		Err(_) => {
			let msg = String::from("Unable to access database");
			return enums::Outcome::Fail(msg);
		}
	}
	
	//deserialize json file into vec
	match serde_json::from_reader(&users_data) {
		Ok(users) => enums::Outcome::Success(users),
		Err(_) => enums::Outcome::Success(Vec::new()),
	}
}

fn add_user (user: User) -> enums::Outcome<()> {
	let mut users: Vec<User>;
	
	match get_users_as_vec() {
		enums::Outcome::Success(a) => users = a,
		enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg),
	}
	
	users.push(user);
	return update_userdata_json(users);
}

fn get_user (email: &String) -> enums::Outcome<User> {
	let users;
	
	match get_users_as_vec() {
		enums::Outcome::Success(usrs) => users = usrs,
		enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg)
	}
	
	for user in users {
		if user.email == *email {
			return enums::Outcome::Success(user);
		}
	}
	
	let msg = String::from("User does not exist");
	return enums::Outcome::Fail(msg)
}

fn update_userdata_json (users: Vec<User>) -> enums::Outcome<()> {
	let path = "./src/storage/userdata.json";
	
	//convert vec to peekable. This allows us to look ahead to the next value before getting to it
	let mut users = users.iter().peekable();
	let mut userdata;
	
	//open json file to be overwritten
	match fs::File::create(path) {
		Ok(file) => userdata = file,
		Err(_) => {
			let msg = String::from("Cannot access database");
			return enums::Outcome::Fail(msg);
		}
	}
	
	//begin creating json string
	let mut json_string = "[".to_string();
	
	//add each file as a json entry
	while let Some(user) = users.next() {
		if users.peek().is_none() {
			json_string = format!("{}{}", json_string, serde_json::to_string(&user).unwrap());
			break;
		}
		
		json_string = format!("{}{},", json_string, serde_json::to_string(&user).unwrap());
	}
	
	//conclude json string
	json_string = format!("{}{}", json_string, "]");
	
	//write json string to json file
	userdata.write(json_string.as_bytes());
	
	return enums::Outcome::Success(());
}