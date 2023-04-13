// Restructured version of database
// Not final

use std::{
    fs::{self, OpenOptions, DirEntry},
    io::{self, Write},
    path::Path, collections::HashMap,
};
use serde::{Serialize, Deserialize};

// Path to databases directory in filesystem
const DATABASES_DIR_PATH: &str = "./databases";

// Database files have JSON file extension
const DATABASE_FILE_EXTENSION: &str = "json";

/// Database structure for database files
#[derive(Serialize, Deserialize)]
struct Database {
    name: String,
    description: String,
    collections: Vec<DocumentCollection>,
}

impl Database {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn collections(&self) -> &Vec<DocumentCollection> {
        &self.collections
    }

    fn collections_mut(&mut self) -> &mut Vec<DocumentCollection> {
        &mut self.collections
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": &self.name,
            "description": &self.description,
            "collections": [],
        })
    }
}

impl Database {}

impl From<&str> for Database {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::new(),
            collections: Vec::new(),
        }
    }
}

impl From<(&str, &str)> for Database {
    fn from((name, description): (&str, &str)) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            collections: Vec::new(),
        }
    }
}

/// Formatted database that can be listed in database clients.
/// 
/// Size = database file size in bytes.
pub struct FormattedDatabase {
    name: String,
    description: String,
    size: u64,
}

impl FormattedDatabase {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }
}

impl FormattedDatabase {
    fn from(name: String, description: String, size: u64) -> Self {
        Self {
            name,
            description,
            size,
        }
    }
}

/// Database document collection
/// that holds database documents.
#[derive(Serialize, Deserialize)]
struct DocumentCollection {
    name: String,
    documents: Vec<Document>,
}

impl DocumentCollection {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            documents: Vec::new(),
        }
    }
}

/// Database document that holds
/// data in key-value pairs
#[derive(Serialize, Deserialize)]
struct Document {
    id: u64,
    data: HashMap<String, serde_json::Value>,
}



/// Gets database file path. Database files have JSON format.
fn database_file_path(database_name: &str) -> String {
    format!("{DATABASES_DIR_PATH}/{database_name}.{DATABASE_FILE_EXTENSION}")
}

/// Check if a database file exists in databases directory
fn database_file_exists(database_name: &str) -> bool {
    return Path::new(database_file_path(database_name).as_str()).is_file();
}

/// Check if databases directory exists in project root
fn databases_dir_exists() -> bool {
    return Path::new(DATABASES_DIR_PATH).is_dir();
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
    let file_path = database_file_path(database_name);

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
    let file_path = database_file_path(database_name);

    if Path::new(&file_path).is_file() {
        fs::remove_file(&file_path)?;
    } else {
        return Ok(false);
    }
    
    Ok(true)
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
                if file_extension == DATABASE_FILE_EXTENSION {
                    let contents = fs::read_to_string(path)?;
                    let database: Database = serde_json::from_str(contents.as_str())?;

                    let formatted_database = FormattedDatabase::from(
                        String::from(database.name()),
                        String::from(database.description()),
                        entry.metadata()?.len()
                    );
                    
                    if formatted_database.name() != "null" {
                        databases.push(formatted_database);
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
            if entry.file_name() == format!("{database_name}.{DATABASE_FILE_EXTENSION}").as_str() {
                // Check if json file contains the name
                let contents = fs::read_to_string(path)?;
                let database: Database = serde_json::from_str(contents.as_str())?;

                if database.name() == database_name {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

/// Writes a new collection to database file
pub fn create_collection_to_database_file(collection_name: &str, database_name: &str) -> io::Result<bool> {
    let file_path = database_file_path(database_name);

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        // Create new JSON and write to file
        let collection = DocumentCollection::from(collection_name);
        database.collections_mut().push(collection);
        let json = serde_json::to_string_pretty(&database)?;
        
        let mut file = OpenOptions::new()
            .write(true)
            .open(&file_path)?;

        file.write(json.as_bytes())?;

        return Ok(true);
    }

    Ok(false)
}
