use std::fs;
use std::io::{Error, Read, Write};
use crate::utils::enums;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct File {
	file_id: String,
	file_name: String,
	file_size: String,
	date_created: String
}

pub fn upload (file_path: &String) -> enums::Outcome<String> {
	
	enums::Outcome::Success("File uploaded successfully".to_string())
}

pub fn read (file_id: &String) -> enums::Outcome<String> {
	enums::Outcome::Success("Woo".to_string())
}

pub fn list () -> enums::Outcome<String> {
	let path = Path::new(".");
	println!("{:?}", path.canonicalize().unwrap());
	let file_metadata = fs::File::open("./src/storage/metadata.json").unwrap();
	let files: Vec<File> = serde_json::from_reader(file_metadata).unwrap();
	
	println!("{}", format!("{:15} | {:40} | {:10} | {:25}", "id", "name", "size", "created on"));
	
	for file in files {
		println!("{}", format!("{:15} | {:40} | {:10} | {:25}", file.file_id, file.file_name, file.file_size, file.date_created));
	}
	
	let message = String::from("All files listed successfully");
	enums::Outcome::Success(message)
}

pub fn delete (file_id: &String) -> enums::Outcome<String> {
	
	enums::Outcome::Success("Woo".to_string())
}