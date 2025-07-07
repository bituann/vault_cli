use std::fs;
use std::io::{Error, Read, Write};
use crate::utils::{enums, helper};
use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Serialize, Deserialize)]
struct File {
	file_id: String,
	file_name: String,
	file_size: String,
	//date_created: String
}

pub fn upload (file_path: &String) -> enums::Outcome<String> {
	let file_name = helper::get_file_name(file_path);
	
	let destination = format!("./src/storage/uploads/{}", file_name);
	
	fs::copy(&file_path, &destination).unwrap();
	
	let file_size = fs::metadata(&destination).unwrap().len();
	
	//let date_created = fs::metadata(&destination).unwrap().created().unwrap().duration_since(UNIX_EPOCH).unwrap();
	
	let file = File {
		file_id: String::from("000"),
		file_name: String::from(file_name),
		file_size: file_size.to_string(),
		//date_created: String::from("{date_created:?}"),
	};
	
	let file_metadata_json = serde_json::to_string(&file).unwrap();
	
	let metadata_path = "./src/storage/metadata.json";
	
	let mut files: Vec<File>;
	
	match json_to_vec(metadata_path) {
		enums::Outcome::Success(received) => files = received,
		enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg)
	}
	files.push(file);
	
	let outcome = update_metadata_json(files, metadata_path);
	
	match outcome {
		enums::Outcome::Success(()) => enums::Outcome::Success("File uploaded successfully.".to_string()),
		enums::Outcome::Fail(msg) => enums::Outcome::Fail("Internal Error.".to_string())
	}
}

pub fn read (file_name: &String) -> enums::Outcome<String> {
	let file_path = format!("./src/storage/uploads/{}", file_name);
	
	let content = match fs::read_to_string(&file_path) {
		Ok(content) => content,
		Err(_) => {
			let msg = String::from("Cannot read file. Check if the file name is correct");
			return enums::Outcome::Fail(msg);
		}
	};
	
	enums::Outcome::Success(content)
}

pub fn list () -> enums::Outcome<String> {
	let path = "./src/storage/metadata.json";
	let mut files: Vec<File>;
	
	match json_to_vec(path) {
		enums::Outcome::Success(received) => files = received,
		enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg)
	};
	
	let mut file_list = format!("{:15} | {:40} | {:10}\n", "id", "name", "size");
	
	for file in files {
		file_list = format!("{}{:15} | {:40} | {:10}\n", file_list, file.file_id, file.file_name, file.file_size);
	}
	
	let file_list = String::from(file_list);
	enums::Outcome::Success(file_list)
}

pub fn delete (file_name: &String) -> enums::Outcome<String> {
	let path = "./src/storage/metadata.json";
	let mut files: Vec<File>;
	
	match json_to_vec(path) {
		enums::Outcome::Success(received) => files = received,
		enums::Outcome::Fail(msg) => return enums::Outcome::Fail(msg)
	};
	
	let file_path = format!("./src/storage/uploads/{}", file_name);
	fs::remove_file(file_path);
	
	files.retain(|file| file.file_name != *file_name);
	
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










//helper functions
fn json_to_vec (path: &str) -> enums::Outcome<Vec<File>> {
	let mut file_metadata;
	
	match fs::File::open(path) {
		Ok(file) => file_metadata = file,
		Err(_) => {
			let msg = String::from("Unable to access database");
			return enums::Outcome::Fail(msg);
		}
	}
	
	match serde_json::from_reader(&file_metadata) {
		Ok(files) => enums::Outcome::Success(files),
		Err(_) => enums::Outcome::Success(Vec::new()),
	}
}

fn update_metadata_json (files: Vec<File>, path: &str) -> enums::Outcome<()> {
	let mut files = files.iter().peekable();
	
	let mut metadata;
	
	match fs::File::create(&path) {
		Ok(file) => metadata = file,
		Err(_) => {
			let msg = String::from("Cannot access database");
			return enums::Outcome::Fail(msg);
		}
	}
	
	let mut json_string = "[".to_string();
	
	while let Some(file) = files.next() {
		if files.peek().is_none() {
			json_string = format!("{}{}", json_string, serde_json::to_string(&file).unwrap());
			break;
		}
		
		json_string = format!("{}{},", json_string, serde_json::to_string(&file).unwrap());
	}
	
	json_string = format!("{}{}", json_string, "]");
	
	metadata.write(json_string.as_bytes());
	
	enums::Outcome::Success(())
}