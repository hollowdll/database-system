// Restructured version of database
// Not final

use std::{
    fs::{self, OpenOptions, DirEntry},
    io::{self, Write},
    path::Path, collections::HashMap,
};

/// Database structure for database files
struct Database {
    name: String,
    collections: Vec<DatabaseDocumentCollection>,
}

impl Database {
    fn name(&self) -> &str {
        &self.name
    }

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

struct ListableDatabase {
    name: String,
    size: u64,
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



/// Creates databases directory in project directory
pub fn create_databases_dir() -> io::Result<()> {
    if !databases_dir_exists() {
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
fn database_file_exists(database_name: &str) -> bool {
    return Path::new(format!("./databases/{database_name}.json").as_str()).is_file();
}

/// Check if databases directory exists in project root
fn databases_dir_exists() -> bool {
    return Path::new("./databases").is_dir();
}

// Find all database files in databases directory
pub fn find_all_database_files() -> io::Result<Vec<DirEntry>> {
    let dir_path = format!("./databases");

    create_databases_dir();

    let mut database_files = Vec::new();

    // Read all files
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_extension) = path.extension() {
                if file_extension == "json" {
                    println!("file is json");
                    println!("File name: {:?}", entry.file_name());
                    println!("File size: {:?} bytes", entry.metadata()?.len());

                    let contents = fs::read_to_string(path)?;

                    let value: serde_json::Value = serde_json::from_str(contents.as_str())?;
                    
                    let database = Database::from_json(value);
                    if database.name() == entry.file_name() {
                        database_files.push(entry);
                    }
                }
            }
        }
    }

    Ok(database_files)
}



