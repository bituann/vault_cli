use std::fs;
use std::io::{Read, Write};
use crate::utils::{enums, helper};
use serde::{Serialize, Deserialize};
use serde_json::Result;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
	pub file_id: String,
	pub user_id: String,
	pub file_name: String,
	pub file_size: String,
	pub file_path: String,
	//date_created: String
}

pub fn upload (file_path: &String, user_id: &String) -> enums::Outcome<String> {
	let file_name = helper::get_file_name(file_path);
	
	let new_file_name = create_file_name(&file_name.to_string(), user_id);
	
	let destination = format!("./src/storage/uploads/{}", new_file_name);
	
	//copy file to uploads folder (destination)
	match fs::copy(&file_path, &destination) {
		Ok(_) => (),
		Err(_) => {
			let msg = String::from("Unable to upload. Check if file path is correct");
			return enums::Outcome::Fail(msg)
		}
	};
	
	let file_size = match fs::metadata(&destination) {
		Ok(file) => file.len(),
		Err(msg) => 0
	};
	
	//let date_created = fs::metadata(&destination).unwrap().created().unwrap().duration_since(UNIX_EPOCH).unwrap();
	
	let file = File {
		file_id: Uuid::new_v4().to_string(),
		user_id: user_id.to_string(),
		file_name: String::from(file_name),
		file_size: file_size.to_string(),
		file_path: String::from(destination)
		//date_created: String::from("{date_created:?}"),
	};
	
	//convert file to json format
	let file_metadata_json = serde_json::to_string(&file).unwrap();
	
	let metadata_path = "./src/storage/metadata.json";
	
	let mut files: Vec<File>;
	
	//read all json entries into 'files'
	match json_to_vec(metadata_path) {
		enums::Outcome::Success(received) => files = received,
		enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg)
	}
	
	//add current file to list of saved files
	files.push(file);
	
	//update json file
	let outcome = update_metadata_json(files, metadata_path);
	
	match outcome {
		enums::Outcome::Success(()) => return enums::Outcome::Success("File uploaded successfully.".to_string()),
		enums::Outcome::Fail(msg) => return enums::Outcome::Fail("Internal Error.".to_string())
	}
}

pub fn read (file_name: &String) -> enums::Outcome<String> {
	//create path from file name
	let file_path = format!("./src/storage/uploads/{}", file_name);
	
	//get file content as string
	let content = match fs::read_to_string(&file_path) {
		Ok(content) => content,
		Err(_) => {
			let msg = String::from("Cannot read file. Check if the file name is correct");
			return enums::Outcome::Fail(msg);
		}
	};
	
	return enums::Outcome::Success(content);
}

pub fn list () -> enums::Outcome<Vec<File>> {
	let path = "./src/storage/metadata.json";
	
	//read json entries into 'files'
	match json_to_vec(path) {
		enums::Outcome::Success(files) => return enums::Outcome::Success(files),
		enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg)
	}
}

pub fn delete (file_name: &String, user_id: &String) -> enums::Outcome<String> {
	let path = "./src/storage/metadata.json";
	let mut files: Vec<File>;
	
	//read json entries into 'files'
	match json_to_vec(path) {
		enums::Outcome::Success(received) => files = received,
		enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg)
	}
	
	let new_file_name = create_file_name(&file_name.to_string(), user_id);
	
	let file_path = format!("./src/storage/uploads/{}", new_file_name);
	
	//remove file from file system
	match fs::remove_file(file_path) {
		Ok(_) => (),
		Err(_) => {
			let msg = String::from("Unable to delete file. Check if file name is correct");
			return enums::Outcome::Fail(msg);
		}
	};
	
	//update 'files' to reflect removed file
	files.retain(|file| file.file_name != *file_name);
	
	//update json file
	match update_metadata_json(files, path) {
		enums::Outcome::Success(()) => {
			let msg = String::from("File deleted successfully");
			return enums::Outcome::Success(msg);
		}
		enums::Outcome::Fail(msg) => {
			let msg = String::from("Unable to delete file");
			return enums::Outcome::Fail(msg);
		}
	}
}

pub fn metadata (file_name: &String) -> enums::Outcome<Vec<File>> {
	let metadata_path = "./src/storage/metadata.json";
	let mut files: Vec<File> = vec![];
	
	//read json file to 'files'
	match json_to_vec(metadata_path) {
		enums::Outcome::Success(received) => {
			for file in received {
				if file.file_name == *file_name {
					files.push(file);
				}
			}
		}
		enums::Outcome::Fail(msg) => {
			let msg = String::from("Cannot read metadata. Ensure the file name is correct. You can use the list command to see the list of files");
			return enums::Outcome::Fail(msg);
		}
	}
	
	if files.len() > 0 {
		return enums::Outcome::Success(files);
	}
	
	//if execution gets to this point, it has failed
	let msg = String::from("Cannot read metadata. Ensure the file name is correct. You can use the list command to see the list of files");
	return enums::Outcome::Fail(msg);
}

pub fn file_exists (file_name: &String) -> bool {
	let metadata_path = "./src/storage/metadata.json";
	
	let mut files: Vec<File>;
	
	//read json entries
	match json_to_vec(metadata_path) {
		enums::Outcome::Success(received) => files = received,
		enums::Outcome::Fail(_) => return false
	}
	
	//check if file exists
	for file in files {
		if file.file_name == *file_name {
			return true;
		}
	}
	
	return false;
}









//private helper functions
//these helper functions were created here to confine interaction with the file system to this module
fn json_to_vec (path: &str) -> enums::Outcome<Vec<File>> {
	let mut file_metadata;
	
	//open json file
	match fs::File::open(path) {
		Ok(file) => file_metadata = file,
		Err(_) => {
			let msg = String::from("Unable to access database");
			return enums::Outcome::Fail(msg);
		}
	}
	
	//deserialize json file into vec
	match serde_json::from_reader(&file_metadata) {
		Ok(files) => enums::Outcome::Success(files),
		Err(_) => enums::Outcome::Success(Vec::new()),
	}
}

fn update_metadata_json (files: Vec<File>, path: &str) -> enums::Outcome<()> {
	//convert vec to peekable. This allows us to look ahead to the next value before getting to it
	let mut files = files.iter().peekable();
	
	let mut metadata;
	
	//open json file to be overwritten
	match fs::File::create(&path) {
		Ok(file) => metadata = file,
		Err(_) => {
			let msg = String::from("Cannot access database");
			return enums::Outcome::Fail(msg);
		}
	}
	
	//begin creating json string
	let mut json_string = "[".to_string();
	
	//add each file as a json entry
	while let Some(file) = files.next() {
		if files.peek().is_none() {
			json_string = format!("{}{}", json_string, serde_json::to_string(&file).unwrap());
			break;
		}
		
		json_string = format!("{}{},", json_string, serde_json::to_string(&file).unwrap());
	}
	
	//conclude json string
	json_string = format!("{}{}", json_string, "]");
	
	//write json string to json file
	metadata.write(json_string.as_bytes());
	
	return enums::Outcome::Success(());
}

fn create_file_name (file_name: &String, id: &String) -> String {
	let mut file_name_tokens: Vec<&str> = file_name.split(".").collect();
	let file_extension = file_name_tokens.pop().unwrap();
	
	let mut new_file_name = String::new();
	
	for word in file_name_tokens {
		new_file_name = format!("{}{}", new_file_name, word);
	}
	
	new_file_name = format!("{}##{}.{}", new_file_name, id, file_extension);
	
	return new_file_name;
}