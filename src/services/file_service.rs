use std::fs;
use std::io::{Error, Read, Write};
use crate::utils::enums;
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
	
	let metadata_file_path = "./src/storage/metadata.json";
	
	fs::OpenOptions::new().append(true).open(&metadata_file_path).unwrap().write(file_metadata_json.as_bytes()).unwrap();
	
	//println!("{}", file_metadata_json);
	
	enums::Outcome::Success("File uploaded successfully".to_string())
}

pub fn read (file_id: &String) -> enums::Outcome<String> {
	enums::Outcome::Success("Woo".to_string())
}

pub fn list () -> enums::Outcome<String> {
	let path = "./src/storage/metadata.json";
	let file_metadata = fs::File::open(path).unwrap();
	let files: Vec<File> = serde_json::from_reader(file_metadata).unwrap();
	
	println!("{}", format!("{:15} | {:40} | {:10} | {:25}", "id", "name", "size", "created on"));
	
	for file in files {
		println!("{}", format!("{:15} | {:40} | {:10}", file.file_id, file.file_name, file.file_size/*, file.date_created*/));
	}
	
	let message = String::from("All files listed successfully");
	enums::Outcome::Success(message)
}

pub fn delete (file_id: &String) -> enums::Outcome<String> {
	
	enums::Outcome::Success("Woo".to_string())
}