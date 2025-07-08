use std::io::{Read, Write};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::fs;
use crate::utils::enums;
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
	
	//hash password
	let password = calculate_hash(&password);
	
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
	let session_token;
	//hash password
	let password = calculate_hash(password);
	
	//get user struct
	let mut user: User;
	
	match get_user(email) {
		enums::Outcome::Success(usr) => user = usr,
		enums::Outcome::Fail(msg) => {
			let msg = String::from("User does not exist");
			return enums::Outcome::Fail(msg);
		}
	}
	
	if user.password == password {
		session_token = Uuid::new_v4().to_string();
		fs::File::create("./src/storage/.vault-session").unwrap().write(session_token.as_bytes());
		return enums::Outcome::Success(session_token);
	}
	
	let msg = String::from("Unable to login");
	return enums::Outcome::Fail(msg);
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