// Restructured version of database
// Not final

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path, collections::HashMap,
};

struct Database {
    name: String,
    collections: Vec<DatabaseDocumentCollection>,
}

impl Database {
    fn new() {
        
    }

    fn new_json() {

    }
}

struct DatabaseDocumentCollection {
    name: String,
    documents: Vec<DatabaseDocument>,
}

struct DatabaseDocument {
    id: u64,
    data: HashMap<String, String>,
}

pub fn create_databases_dir() -> io::Result<()> {
    if !Path::new("./databases").is_dir() {
        fs::create_dir("./databases")?;
    }

    Ok(())
}

pub fn create_database_file(database_name: &str) -> io::Result<bool> {
    if !Path::new(format!("./databases/{database_name}.json").as_str()).is_file() {
        let mut file = fs::File::create(format!("./databases/{database_name}.json"))?;

        // write initial data


    } else {
        return Ok(false);
    }

    Ok(true)
}

pub fn database_file_exists(database_name: &str) -> bool {
    if Path::new(format!("./databases/{database_name}.json").as_str()).is_file() {
        return true;
    } else {
        return false;
    }
}