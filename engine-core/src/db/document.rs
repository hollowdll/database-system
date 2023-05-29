use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::db::DataType;
use std::{
    io,
    fs,
    path::Path,
};
use crate::db::{
    Database,
    DocumentCollection,
    database_file_path,
    write_database_json,
    create_databases_dir_if_not_exists,
};
use crate::constants::{
    DB_NOT_FOUND,
    COLLECTION_NOT_FOUND,
    DOCUMENT_NOT_FOUND,
    DB_DIR_PATH,
    DB_FILE_EXTENSION,
};

/// Database document that holds
/// data in key-value pairs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub id: u64,
    pub data: HashMap<String, DataType>,
}

impl Document {
    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }
}

impl Document {
    pub fn from(id_count: u64) -> Self {
        Self {
            id: id_count,
            data: HashMap::new(),
        }
    }
}

/// Formatted document that can be listed in clients
#[derive(Debug)]
pub struct FormattedDocument {
    id: u64,
    data: HashMap<String, DataType>,
}

impl FormattedDocument {
    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }
}

impl FormattedDocument {
    pub fn from(id: u64, data: HashMap<String, DataType>) -> Self {
        Self {
            id,
            data,
        }
    }
}



/// Creates a document to a collection
pub fn create_document_to_collection(
    file_path: &Path,
    collection_name: &str, 
    data: HashMap<String, DataType>,
) -> io::Result<(bool, String)>
{
    let mut message = "";

    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
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

                match write_database_json(&database, file_path) {
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

/// Deletes a document from a collection by document id.
/// 
/// This is a faster way to delete a document
/// if the collection is known beforehand.
pub fn delete_document_from_collection(
    file_path: &Path,
    collection_name: &str,
    document_id: &u64,
) -> io::Result<(bool, String)>
{
    let mut message = "";

    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        for collection in database.collections_mut() {
            if collection.name() == collection_name {
                if let Some(document) = collection.documents().iter().find(|document| document.id() == document_id) {
                    collection.documents_mut().retain(|document| document.id() != document_id);

                    match write_database_json(&database, file_path) {
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
    file_path: &Path,
    document_id: &u64
) -> io::Result<(bool, String)>
{
    let mut message = "";

    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        let mut found = false;

        for collection in database.collections_mut() {
            if let Some(document) = collection.documents().iter().find(|document| document.id() == document_id) {
                collection.documents_mut().retain(|document| document.id() != document_id);
                found = true;
            };
        }

        if found {
            match write_database_json(&database, file_path) {
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

/// Finds all documents of a collection
pub fn find_all_documents_of_collection(
    file_path: &Path,
    collection_name: &str
) -> io::Result<Vec<FormattedDocument>>
{
    let mut documents = Vec::new();

    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
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

/// Finds a document from a database by its id.
/// 
/// Returns the document if it was found along with a message
/// and the collection the document belongs to.
pub fn find_document_by_id(
    document_id: &u64,
    file_path: &Path,
) -> io::Result<(Option<FormattedDocument>, String, String)>
{
    let mut found_document = None;
    let mut message = "";
    let mut collection = "";

    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(&contents)?;

        for collection in database.collections.into_iter() {
            for document in collection.documents.into_iter() {
                if document.id() == document_id {
                    let formatted_document = FormattedDocument::from(
                        document.id,
                        document.data,
                    );

                    return Ok((
                        Some(formatted_document),
                        message.to_string(),
                        collection.name,
                    ))
                }
            }
        }

        message = DOCUMENT_NOT_FOUND;
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((found_document, message.to_string(), collection.to_string()))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_document_by_id() {

        
    }
}
