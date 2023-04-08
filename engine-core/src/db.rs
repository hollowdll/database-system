// Restructured version of database
// Not final

use std::{
    fs::{self, OpenOptions, DirEntry},
    io::{self, Write},
    path::Path, collections::HashMap,
};

// Path to databases directory in filesystem
const DATABASES_DIR_PATH: &str = "./databases";

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

/// Formatted database that can be listed in database clients
pub struct FormattedDatabase {
    name: String,
    size: u64,
}

impl FormattedDatabase {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }
}

impl FormattedDatabase {
    fn from(name: String, size: u64) -> Self {
        Self {
            name,
            size,
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

/// Creates databases directory in project directory
pub fn create_databases_dir() -> io::Result<()> {
    if !databases_dir_exists() {
        fs::create_dir(DATABASES_DIR_PATH)?;
    }

    Ok(())
}

/// Creates a database file in databases directory
/// with initial data
pub fn create_database_file(database_name: &str) -> io::Result<bool> {
    let file_path = format!("{DATABASES_DIR_PATH}/{database_name}.json");

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

/// Deletes a database file in databases directory
pub fn delete_database_file(database_name: &str) -> io::Result<bool> {
    let file_path = format!("{DATABASES_DIR_PATH}/{database_name}.json");

    if Path::new(&file_path).is_file() {
        fs::remove_file(&file_path)?;
    } else {
        return Ok(false);
    }
    
    Ok(true)
}

/// Check if a database file exists in databases directory
fn database_file_exists(database_name: &str) -> bool {
    return Path::new(format!("{DATABASES_DIR_PATH}/{database_name}.json").as_str()).is_file();
}

/// Check if databases directory exists in project root
fn databases_dir_exists() -> bool {
    return Path::new(DATABASES_DIR_PATH).is_dir();
}

// Finds all database files in databases directory
pub fn find_all_databases() -> io::Result<Vec<FormattedDatabase>> {
    create_databases_dir();

    let mut databases = Vec::new();

    for entry in fs::read_dir(DATABASES_DIR_PATH)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(file_extension) = path.extension() {
                if file_extension == "json" {
                    let contents = fs::read_to_string(path)?;
                    let json_value: serde_json::Value = serde_json::from_str(contents.as_str())?;

                    let database = FormattedDatabase::from(
                        json_value["name"].to_string(),
                        entry.metadata()?.len()
                    );
                    
                    if database.name() != "null" {
                        databases.push(database);
                    }
                }
            }
        }
    }

    Ok(databases)
}

/// Finds a database file in databases directory
pub fn find_database(database_name: &str) -> io::Result<bool> {
    create_databases_dir();

    for entry in fs::read_dir(DATABASES_DIR_PATH)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if entry.file_name() == format!("{database_name}.json").as_str() {
                // Check if json file contains the name
                let contents = fs::read_to_string(path)?;
                let json_value: serde_json::Value = serde_json::from_str(contents.as_str())?;

                if json_value["name"] == database_name {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

