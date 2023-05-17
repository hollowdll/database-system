// This file contains lower level code to do operations to database files.
// Contains file system access, and database file reading and writing.

#![allow(unused)]

pub mod database;
pub mod collection;
pub mod document;
pub mod data_type;

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    collections::HashMap
};
use serde::{Serialize, Deserialize};
use crate::constants::{
    DB_NOT_FOUND,
    COLLECTION_NOT_FOUND,
    DOCUMENT_NOT_FOUND,
    DATABASES_DIR_PATH,
    DATABASE_FILE_EXTENSION,
};
pub use crate::db::{
    data_type::DataType,
    collection::DocumentCollection,
    collection::FormattedDocumentCollection,
    database::Database,
    database::FormattedDatabase,
    document::Document,
    document::FormattedDocument,
};

/// Gets database file path. Database files have JSON format.
fn database_file_path(database_name: &str) -> String {
    format!("{DATABASES_DIR_PATH}/{database_name}.{DATABASE_FILE_EXTENSION}")
}

/// Check if a database file exists in databases directory
fn database_file_exists(database_name: &str) -> bool {
    return Path::new(&database_file_path(database_name)).is_file();
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

/// Writes database as JSON to database file
fn write_database_json(database: &Database, file_path: &str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(&database)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)?;

    file.write(json.as_bytes())?;

    Ok(())
}

/// Creates a database file in databases directory
/// with initial data
pub fn create_database_file(database_name: &str) -> io::Result<(bool, String)> {
    let file_path = database_file_path(database_name);
    let mut message = "";

    if !Path::new(&file_path).is_file() {
        let file = fs::File::create(&file_path)?;
        let database = Database::from(database_name);
        
        match write_database_json(&database, &file_path) {
            Ok(()) => return Ok((true, message.to_string())),
            Err(e) => return Err(e),
        }
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((false, message.to_string()))
}

/// Deletes a database file in databases directory
pub fn delete_database_file(database_name: &str) -> io::Result<(bool, String)> {
    let file_path = database_file_path(database_name);
    let mut message = "";

    if Path::new(&file_path).is_file() {
        fs::remove_file(&file_path)?;

        return Ok((true, message.to_string()));
    } else {
        message = DB_NOT_FOUND;
    }
    
    Ok((false, message.to_string()))
}

/// Finds all database files in databases directory
pub fn find_all_databases() -> io::Result<Vec<FormattedDatabase>> {
    create_databases_dir()?;

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
pub fn find_database(database_name: &str) -> io::Result<bool> {
    create_databases_dir()?;

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
pub fn change_database_description(database_name: &str, description: &str) -> io::Result<(bool, String)> {
    let file_path = database_file_path(database_name);
    let mut message = "";

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        database.description = String::from(description);
        
        match write_database_json(&database, &file_path) {
            Ok(()) => return Ok((true, message.to_string())),
            Err(e) => return Err(e),
        }
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((false, message.to_string()))
}

/// Writes a new collection to a database file
pub fn create_collection_to_database_file(collection_name: &str, database_name: &str) -> io::Result<(bool, String)> {
    let file_path = database_file_path(database_name);
    let mut message = "";

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        // Create new JSON and write to file
        let collection = DocumentCollection::from(collection_name);
        database.collections_mut().push(collection);
        
        match write_database_json(&database, &file_path) {
            Ok(()) => return Ok((true, message.to_string())),
            Err(e) => return Err(e),
        }
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((false, message.to_string()))
}

/// Deletes a collection from a database file
pub fn delete_collection_from_database_file(collection_name: &str, database_name: &str) -> io::Result<(bool, String)> {
    let file_path = database_file_path(database_name);
    let mut message = "";

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
            database.collections_mut().retain(|collection| collection.name() != collection_name);

            match write_database_json(&database, &file_path) {
                Ok(()) => return Ok((true, message.to_string())),
                Err(e) => return Err(e),
            }
        } else {
            message = COLLECTION_NOT_FOUND;
        }
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((false, message.to_string()))
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
) -> io::Result<(bool, String)>
{
    let file_path = database_file_path(database_name);
    let mut message = "";

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

                match write_database_json(&database, &file_path) {
                    Ok(()) => return Ok((true, message.to_string())),
                    Err(e) => return Err(e),
                }
            } else {
                message = COLLECTION_NOT_FOUND;
            }
        } else {
            message = COLLECTION_NOT_FOUND;
        }
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((false, message.to_string()))
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

/// Deletes a document from a collection by document id.
pub fn delete_document_from_collection(
    database_name: &str,
    collection_name: &str,
    document_id: &u64,
) -> io::Result<(bool, String)>
{
    let file_path = database_file_path(database_name);
    let mut message = "";

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        for collection in database.collections_mut() {
            if collection.name() == collection_name {
                if let Some(document) = collection.documents().iter().find(|document| document.id() == document_id) {
                    collection.documents_mut().retain(|document| document.id() != document_id);

                    match write_database_json(&database, &file_path) {
                        Ok(()) => return Ok((true, message.to_string())),
                        Err(e) => return Err(e),
                    }
                } else {
                    return Ok((false, DOCUMENT_NOT_FOUND.to_string()));
                }
            }
        }

        message = COLLECTION_NOT_FOUND;
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((false, message.to_string()))
}

/// Deletes a document from database by document id.
/// 
/// Goes through all collections until id is found.
pub fn delete_document(
    database_name: &str,
    document_id: &u64
) -> io::Result<(bool, String)>
{
    let file_path = database_file_path(database_name);
    let mut message = "";

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        let mut found = false;

        for collection in database.collections_mut() {
            if let Some(document) = collection.documents().iter().find(|document| document.id() == document_id) {
                collection.documents_mut().retain(|document| document.id() != document_id);
                found = true;
            };
        }

        if found {
            match write_database_json(&database, &file_path) {
                Ok(()) => return Ok((true, message.to_string())),
                Err(e) => return Err(e),
            }
        } else {
            message = DOCUMENT_NOT_FOUND;
        }
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((false, message.to_string()))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_file_path() {
        let database_name = "test_db_123";
        let file_path = format!("{DATABASES_DIR_PATH}/{database_name}.{DATABASE_FILE_EXTENSION}");

        assert_eq!(file_path, database_file_path(database_name));
    }
}
