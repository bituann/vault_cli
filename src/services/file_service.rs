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
	let file_name = file_path.trim().split("/").collect::<Vec<&str>>().pop().unwrap();
	
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
	
	let mut files: Vec<File> = json_to_vec(metadata_path);
	files.push(file);
	
	let outcome = update_metadata_json(files, metadata_path);
	
	match outcome {
		enums::Outcome::Success(()) => enums::Outcome::Success("File uploaded successfully.".to_string()),
		enums::Outcome::Fail(msg) => enums::Outcome::Fail("Internal Error.".to_string())
	}
}

pub fn read (file_name: &String) -> enums::Outcome<String> {
	let f_name = helper::get_file_name(file_name);
	let mut file = fs::File::open(format!("./src/storage/uploads/{}", f_name)).unwrap();
	let mut content = String::new();
	
	file.read_to_string(&mut content).unwrap();
	
	println!("{}", content);
	
	enums::Outcome::Success("Woo".to_string())
}

pub fn list () -> enums::Outcome<String> {
	let path = "./src/storage/metadata.json";
	let files: Vec<File> = json_to_vec(path);
	
	println!("{}", format!("{:15} | {:40} | {:10}", "id", "name", "size"));
	
	for file in files {
		println!("{}", format!("{:15} | {:40} | {:10}", file.file_id, file.file_name, file.file_size));
	}
	
	let message = String::from("All files listed successfully");
	enums::Outcome::Success(message)
}

pub fn delete (file_name: &String) -> enums::Outcome<String> {
	let f_name = helper::get_file_name(file_name);
	let path = "./src/storage/metadata.json";
	let mut files: Vec<File> = json_to_vec(path);
	
	fs::remove_file(file_name);
	
	files.retain(|file| file.file_name != f_name);
	
	update_metadata_json(files, path);
	
	enums::Outcome::Success("Woo".to_string())
}










//helper functions
fn json_to_vec (path: &str) -> Vec<File> {
	let file_metadata = fs::File::open(path).unwrap();
	match serde_json::from_reader(&file_metadata) {
		Ok(files) => files,
		Err(_) => Vec::new(),
	}
}

fn update_metadata_json (files: Vec<File>, path: &str) -> enums::Outcome<()> {
	let mut files = files.iter().peekable();
	
	let mut metadata = fs::File::create(&path).unwrap();
	
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