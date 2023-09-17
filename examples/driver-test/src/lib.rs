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
        .get_database("DriverPeopleDatabase")
        .expect("Cannot construct database");
    let db_data = database
        .get_metadata()
        .expect("Cannot get database metadata");

    println!("Database info");
    println!("-------------");
    println!("Name: {}", db_data.name());
    println!("Description: {}", db_data.description());
    println!("Size: {} B", db_data.size());
    println!("Location: {}", db_data.file_path().display());

    let people_collection = database
        .get_collection("people")
        .expect("Cannot construct collection");

    println!("\nInserting a document to collection 'people'...");
    let new_person = people_collection.insert_one(person).unwrap();
    let found_person = people_collection.find_one_by_id(new_person.id()).unwrap();
    
    println!("\nInserted document info");
    println!("----------------------");
    println!("_id: {}", new_person.id());

    for (key, value) in new_person.data {
        println!("{}: {}", key, value);
    }

    println!("\nFind inserted document by id");
    println!("----------------------------");

    if let Some(found_person) = found_person {
        println!("_id: {}", found_person.id());
    
        for (key, value) in found_person.data {
            println!("{}: {}", key, value);
        }
    } else {
        println!("Document not found");
    }

    let people = people_collection.find_all().unwrap();

    println!("\nAll documents");
    println!("-------------");
    for person in people {
        println!("_id: {}", person.id);
        for (key, value) in person.data {
            println!("{}: {}", key, value);
        }
        println!();
    }
}