use serde::{Serialize, Deserialize};
use std::{
    io,
    fs,
    path::Path,
};
use crate::db::{
    Database,
    Document,
    database_file_path,
    write_database_json,
    create_databases_dir_if_not_exists,
};
use crate::constants::{
    DB_NOT_FOUND,
    COLLECTION_NOT_FOUND,
    DB_DIR_PATH,
    DB_FILE_EXTENSION,
};

/// Database document collection
/// that holds database documents.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentCollection {
    pub name: String,
    pub documents: Vec<Document>,
}

impl DocumentCollection {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn documents(&self) -> &Vec<Document> {
        &self.documents
    }

    pub fn documents_mut(&mut self) -> &mut Vec<Document> {
        &mut self.documents
    }
}

impl DocumentCollection {
    pub fn from(name: &str) -> Self {
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
    pub fn from(name: String) -> Self {
        Self {
            name
        }
    }
}



/// Writes a new collection to a database file
pub fn create_collection_to_database_file(
    collection_name: &str,
    file_path: &str
) -> io::Result<(bool, String)>
{
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
pub fn delete_collection_from_database_file(
    collection_name: &str,
    file_path: &str
) -> io::Result<(bool, String)>
{
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
pub fn find_all_collections_of_database(
    file_path: &str
) -> io::Result<Vec<FormattedDocumentCollection>>
{
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
pub fn find_collection(
    collection_name: &str,
    file_path: &str
) -> io::Result<bool>
{
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
