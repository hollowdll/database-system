#![allow(unused)]

use driver::client::DatabaseClient;

struct Person {
    pub id: u64,
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub age: u16,
}

pub fn run() {
    let client = DatabaseClient::build();
    let database = client
        .get_database("DriverTestPeople")
        .expect("Cannot construct database");
    let people_collection = database
        .get_collection::<Person>("people")
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