// Restructured version of database
// Not final

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path, collections::HashMap,
};

/// Database structure for database files
struct Database {
    name: String,
    collections: Vec<DatabaseDocumentCollection>,
}

impl Database {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "collections": [],
        })
    }
}

impl Database {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            collections: Vec::new(),
        }
    }

    fn from_json(json: serde_json::Value) -> Self {
        Self {
            name: json["name"].to_string(),
            collections: Vec::new(),
        }
    }
}

/// Database document collection
/// that holds database documents.
struct DatabaseDocumentCollection {
    name: String,
    documents: Vec<DatabaseDocument>,
}

/// Database document that holds
/// data in key-value pairs
struct DatabaseDocument {
    id: u64,
    data: HashMap<String, String>,
}

/// Creates databases directory in root directory
pub fn create_databases_dir() -> io::Result<()> {
    if !Path::new("./databases").is_dir() {
        fs::create_dir("./databases")?;
    }

    Ok(())
}

/// Creates a database file in databases directory
/// with initial data
pub fn create_database_file(database_name: &str) -> io::Result<bool> {
    let file_path = format!("./databases/{database_name}.json");

    if !Path::new(&file_path).is_file() {
        let mut file = fs::File::create(&file_path)?;

        // write initial data
        let database = Database::from(database_name);
        let json = database.to_json();

        let mut file = OpenOptions::new()
            .write(true)
            .open(&file_path)?;

        file.write(serde_json::to_string_pretty(&json)?.as_bytes())?;

    } else {
        return Ok(false);
    }

    Ok(true)
}

/// Check if a database file exists in databases directory
pub fn database_file_exists(database_name: &str) -> bool {
    if Path::new(format!("./databases/{database_name}.json").as_str()).is_file() {
        return true;
    } else {
        return false;
    }
}