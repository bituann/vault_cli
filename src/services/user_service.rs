use std::io;
use std::hash::{DefaultHasher, Hash, Hasher};
use crate::utils::enums;
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct User {
	id: String,
	email: String,
	password: String,
}

pub fn register (email: &String, password: &String) -> enums::Outcome<String> {
	
	//
	enums::Outcome::Success(String::from(""))
}

pub fn login (email: &String, password: &String) -> enums::Outcome<String> {
	
	enums::Outcome::Success(String::from(""))
}

fn calculate_hash<T: Hash> (value: &T) -> String {
	//create hasher
	let mut hasher = DefaultHasher::new();
	
	//hash the value
    value.hash(&mut hasher);
    
    //get hashed value
    let hash = hasher.finish();
    
    return hash.to_string();
}