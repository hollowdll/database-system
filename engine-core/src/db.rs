// Restructured version of database
// Not final

use std::{
    fs::{self, OpenOptions, DirEntry},
    io::{self, Write},
    path::Path,
    collections::HashMap
};
use serde::{Serialize, Deserialize};

// Path to databases directory in filesystem
const DATABASES_DIR_PATH: &str = "./databases";

// Database files have JSON file extension
const DATABASE_FILE_EXTENSION: &str = "json";

/// Database structure for database files
#[derive(Debug, Serialize, Deserialize)]
struct Database {
    name: String,
    description: String,
    collections: Vec<DocumentCollection>,
    id_count: u64,
}

impl Database {
    fn name(&self) -> &str {
        &self.name
    }

    fn name_mut(&mut self) -> &mut str {
        &mut self.name
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

    fn id_count(&self) -> &u64 {
        &self.id_count
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": &self.name,
            "description": &self.description,
            "collections": [],
            "id_count": &self.id_count,
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
            id_count: 0,
        }
    }
}

impl From<(&str, &str)> for Database {
    fn from((name, description): (&str, &str)) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            collections: Vec::new(),
            id_count: 0,
        }
    }
}

/// Formatted database that can be listed in clients.
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
#[derive(Debug, Serialize, Deserialize)]
struct DocumentCollection {
    name: String,
    documents: Vec<Document>,
}

impl DocumentCollection {
    fn name(&self) -> &str {
        &self.name
    }

    fn documents(&self) -> &Vec<Document> {
        &self.documents
    }

    fn documents_mut(&mut self) -> &mut Vec<Document> {
        &mut self.documents
    }
}

impl DocumentCollection {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            documents: Vec::new(),
        }
    }
}

/// Formatted document collection that can be listed in clients
pub struct FormattedDocumentCollection {
    name: String,
}

impl FormattedDocumentCollection {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FormattedDocumentCollection {
    fn from(name: String) -> Self {
        Self {
            name
        }
    }
}

/// Database document that holds
/// data in key-value pairs
#[derive(Debug, Serialize, Deserialize)]
struct Document {
    id: u64,
    data: HashMap<String, DataType>,
}

impl Document {
    fn id(&self) -> &u64 {
        &self.id
    }

    fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }
}

impl Document {
    fn from(id_count: u64) -> Self {
        Self {
            id: id_count,
            data: HashMap::new(),
        }
    }
}

/// Formatted document that can be listed in clients
pub struct FormattedDocument {
    id: u64,
    data: HashMap<String, DataType>,
}

impl FormattedDocument {
    fn from(id: u64, data: HashMap<String, DataType>) -> Self {
        Self {
            id,
            data,
        }
    }
}



/// Data type for document fields
#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
    Int32(i32),
    Int64(i64),
    Decimal(f64),
    Bool(bool),
    Text(String),
    // Possibly more in the future
}

/// Input data field which is used to create fields to documents
pub struct InputDataField {
    field: String,
    data_type: String,
    value: String,
}

impl InputDataField {
    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn data_type(&self) -> &str {
        &self.data_type
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl InputDataField {
    pub fn from(field: &str, data_type: &str, value: &str) -> Self {
        Self {
            field: field.to_string(),
            data_type: data_type.to_string(),
            value: value.to_string(),
        }
    }
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

/// Finds all database files in databases directory
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
                    let database: Database = match serde_json::from_str(contents.as_str()) {
                        Ok(database) => database,
                        Err(e) => {
                            eprintln!("Error parsing database: {e} ({:?})", entry.file_name());
                            continue
                        },
                    };

                    let formatted_database = FormattedDatabase::from(
                        String::from(database.name()),
                        String::from(database.description()),
                        entry.metadata()?.len()
                    );
                    
                    databases.push(formatted_database);
                }
            }
        }
    }

    Ok(databases)
}

/// Finds a database file in databases directory.
/// 
/// Returns `Ok(true)` if database was found.
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

/// Changes description of a database.
/// 
/// Modifies `description` field in a database file.
pub fn change_database_description(database_name: &str, description: &str) -> io::Result<bool> {
    let file_path = database_file_path(database_name);

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        database.description = String::from(description);
        let json = serde_json::to_string_pretty(&database)?;

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)?;

        file.write(json.as_bytes())?;

        return Ok(true);
    }

    Ok(false)
}

/// Writes a new collection to a database file
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
            .truncate(true)
            .open(&file_path)?;

        file.write(json.as_bytes())?;

        return Ok(true);
    }

    Ok(false)
}

/// Deletes a collection from a database file
pub fn delete_collection_from_database_file(collection_name: &str, database_name: &str) -> io::Result<bool> {
    let file_path = database_file_path(database_name);

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        let mut found = false;

        for collection in database.collections() {
            if collection.name() == collection_name {
                found = true;
                break;
            }
        }

        if found {
            database.collections_mut().retain(|x| x.name() != collection_name);

            let json = serde_json::to_string_pretty(&database)?;
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&file_path)?;

            file.write(json.as_bytes())?;

            return Ok(true);
        }
    }

    Ok(false)
}

/// Finds all collections of a database
pub fn find_all_collections_of_database(database_name: &str) -> io::Result<Vec<FormattedDocumentCollection>> {
    let file_path = database_file_path(database_name);
    let mut collections = Vec::new();

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        for collection in database.collections() {
            let formatted_collection = FormattedDocumentCollection::from(
                String::from(collection.name())
            );
            
            collections.push(formatted_collection);
        }
    }
    
    Ok(collections)
}

/// Finds a collection in a database file.
/// 
/// Returns `Ok(true)` if collection was found.
pub fn find_collection(collection_name: &str, database_name: &str) -> io::Result<bool> {
    let file_path = database_file_path(database_name);

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        for collection in database.collections() {
            if collection.name() == collection_name {
                return Ok(true)
            }
        }
    }

    Ok(false)
}

/// Creates a document to a collection
pub fn create_document_to_collection(
    database_name: &str,
    collection_name: &str, 
    data: HashMap<String, DataType>,
) -> io::Result<bool>
{
    let file_path = database_file_path(database_name);

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        let mut collection_index = None;

        // Find collection
        for (index, collection) in database.collections().iter().enumerate() {
            if collection.name() == collection_name {
                collection_index = Some(index);
            }
        }

        if let Some(collection_index) = collection_index {
            // Increment database id_count by one
            database.id_count += 1;
            let mut document = Document::from(database.id_count);
            document.data = data;

            if let Some(collection) = database.collections_mut().get_mut(collection_index) {
                collection.documents_mut().push(document);
                let json = serde_json::to_string_pretty(&database)?;

                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&file_path)?;

                file.write(json.as_bytes())?;

                return Ok(true)
            }
        }
    }

    Ok(false)
}

/// Finds all collections of a database
pub fn find_all_documents_of_collection(
    database_name: &str,
    collection_name: &str
) -> io::Result<Vec<FormattedDocument>>
{
    let file_path = database_file_path(database_name);
    let mut documents = Vec::new();

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        for collection in database.collections.into_iter() {
            if collection.name() == collection_name {
                for document in collection.documents.into_iter() {
                    let formatted_document = FormattedDocument::from(
                        document.id,
                        document.data,
                    );

                    documents.push(formatted_document)
                }
            }
        }
    }
    
    Ok(documents)
}
