use serde::{Serialize, Deserialize};
use std::{
    io,
    fs,
    path::Path,
    error::Error,
};
use crate::db::{
    error::{
        DatabaseError,
        CollectionError,
    },
    Database,
    Document,
    write_database_json,
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

#[derive(Debug, PartialEq)]
/// Formatted document collection that can be listed in clients.
pub struct FormattedDocumentCollection {
    name: String,
}

impl FormattedDocumentCollection {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FormattedDocumentCollection {
    pub fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}



/// Writes a new collection to a database file.
pub fn create_collection_to_database_file(
    collection_name: &str,
    file_path: &Path,
) -> Result<(), Box<dyn Error>>
{
    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        // Error if collection already exists
        for collection in database.collections() {
            if collection.name() == collection_name {
                return Err(Box::new(CollectionError::Exists));
            }
        }

        let collection = DocumentCollection::from(collection_name);
        database.collections_mut().push(collection);
        
        match write_database_json(&database, file_path) {
            Ok(()) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    } else {
        return Err(Box::new(DatabaseError::NotFound));
    }
}

/// Deletes a collection from a database file.
pub fn delete_collection_from_database_file(
    collection_name: &str,
    file_path: &Path
) -> Result<(), Box<dyn Error>>
{
    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        let mut found = false;

        for collection in database.collections() {
            if collection.name() == collection_name {
                found = true;
                break;
            }
        }

        if found {
            database
                .collections_mut()
                .retain(|collection| collection.name() != collection_name);

            match write_database_json(&database, file_path) {
                Ok(()) => return Ok(()),
                Err(e) => return Err(e.into()),
            }
        } else {
            return Err(Box::new(CollectionError::NotFound));
        }
    } else {
        return Err(Box::new(DatabaseError::NotFound));
    }
}

/// Finds all collections of a database.
pub fn find_all_collections_of_database(
    file_path: &Path
) -> io::Result<Vec<FormattedDocumentCollection>>
{
    let mut collections = Vec::new();

    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        for collection in database.collections() {
            let formatted_collection = FormattedDocumentCollection::from(
                collection.name()
            );
            
            collections.push(formatted_collection);
        }
    }
    
    Ok(collections)
}

/// Finds a collection in a database file.
pub fn find_collection(
    collection_name: &str,
    file_path: &Path
) -> Result<Option<FormattedDocumentCollection>, Box<dyn Error>>
{
    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;

        for collection in database.collections() {
            if collection.name() == collection_name {
                let formatted_collection = FormattedDocumentCollection::from(
                    collection.name()
                );

                return Ok(Some(formatted_collection));
            }
        }

        return Ok(None);
    } else {
        return Err(Box::new(DatabaseError::NotFound));
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write, Read};
    use tempfile::tempdir;
    use std::fs::File;

    #[test]
    fn test_create_collection_to_database_file() {
        let mut database = Database::from("test");
        let collection_name = "test_collection";
        let json = serde_json::to_string_pretty(&database).unwrap();

        database.collections_mut().push(DocumentCollection::from(collection_name));
        let expected_json = serde_json::to_string_pretty(&database).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write(json.as_bytes()).is_ok());
        assert!(create_collection_to_database_file(
            collection_name,
            file_path.as_path()
        ).is_ok());

        let buf = fs::read_to_string(&file_path).unwrap();
        assert_eq!(buf, expected_json);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }

    #[test]
    fn test_delete_collection_from_database_file() {
        let mut database = Database::from("test");
        let collection_name = "test_collection";
        let expected_json = serde_json::to_string_pretty(&database).unwrap();

        database.collections_mut().push(DocumentCollection::from(collection_name));
        let json = serde_json::to_string_pretty(&database).unwrap();
    
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write(json.as_bytes()).is_ok());
        assert!(delete_collection_from_database_file(
            collection_name,
            file_path.as_path()
        ).is_ok());

        let buf = fs::read_to_string(&file_path).unwrap();
        assert_eq!(buf, expected_json);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }
    
    #[test]
    fn test_find_all_collections_of_database() {
        let mut database = Database::from("test");
        let collection_name = "test_collection";
        database.collections_mut().push(DocumentCollection::from(collection_name));
        let json = serde_json::to_string_pretty(&database).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();
        assert!(file.write(json.as_bytes()).is_ok());

        let collections = find_all_collections_of_database(&file_path).unwrap();
        assert_eq!(
            collections.get(0),
            Some(&FormattedDocumentCollection::from(collection_name))
        );
        assert!(collections.len() == 1);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }
    
    #[test]
    fn test_find_collection() {
        let mut database = Database::from("test");
        let collection_name = "test_collection";
        database.collections_mut().push(DocumentCollection::from(collection_name));
        let json = serde_json::to_string_pretty(&database).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();
        assert!(file.write(json.as_bytes()).is_ok());

        let collection = find_collection(collection_name, &file_path).unwrap();
        assert!(collection.is_some());
        assert_eq!(collection.unwrap().name(), collection_name);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }
    
}

