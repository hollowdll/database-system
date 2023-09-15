#![allow(unused)]

use std::env::current_exe;

use driver::{
    client::DatabaseClient,
    document::{
        DocumentModel,
        DataType,
    },
};

struct Person {
    pub id: u64,
    pub first_name: &'static str,
    pub last_name: &'static str,
}

pub fn run() {
    let mut person = DocumentModel::new();
    person.data.insert("first_name".to_string(), DataType::Text("John".to_string()));
    person.data.insert("last_name".to_string(), DataType::Text("Smith".to_string()));

    let mut db_dir = current_exe().unwrap();
    db_dir.pop();

    let client = DatabaseClient::build(&db_dir);
    let database = client
        .get_database("  .  a. ....")
        .expect("Cannot construct database");
    let people_collection = database
        .get_collection("people")
        .expect("Cannot construct collection");
    let db_data = database
        .get_metadata()
        .expect("Cannot get database metadata");

    println!("Database info:");
    println!("Name: {}", db_data.name());
    println!("Description: {}", db_data.description());
    println!("Size: {} B", db_data.size());
    println!("Location: {}", db_data.file_path().display());
}