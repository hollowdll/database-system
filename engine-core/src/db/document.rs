use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::{
    io,
    fs,
    path::Path,
    error::Error,
    fmt,
};
use crate::db::{
    error::{
        DatabaseError,
        CollectionError,
        DocumentError,
    },
    Database,
    DocumentCollection,
    write_database_json,
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
    pub id: DataType,
    pub data: HashMap<String, DataType>,
}

impl Document {
    pub fn id(&self) -> &DataType {
        &self.id
    }

    pub fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }
}

impl Document {
    pub fn from(database: &mut Database) -> Self {
        database.id_count += 1;
        Self {
            id: DataType::DocumentId(database.id_count),
            data: HashMap::new(),
        }
    }
}

/// Formatted document that can be listed in clients
#[derive(Debug)]
pub struct FormattedDocument {
    id: DataType,
    collection: String,
    data: HashMap<String, DataType>,
}

impl FormattedDocument {
    pub fn id(&self) -> &DataType {
        &self.id
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }
}

impl FormattedDocument {
    pub fn from(id: DataType, collection: String, data: HashMap<String, DataType>) -> Self {
        Self {
            id,
            collection,
            data,
        }
    }
}

/// Data type for document fields
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum DataType {
    /// 64-bit unsigned integer. Only document id can have this.
    DocumentId(u64),

    /// 32-bit signed integer for numbers.
    Int32(i32),

    /// 64-bit signed integer for numbers.
    Int64(i64),

    /// 64-bit floating point for deicmal numbers.
    Decimal(f64),

    /// Boolean type for values true and false.
    Bool(bool),

    /// UTF-8 string for dynamic texts.
    Text(String),

    // Possibly more in the future
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataType::DocumentId(value) => value.to_string(),
                DataType::Int32(value) => value.to_string(),
                DataType::Int64(value) => value.to_string(),
                DataType::Decimal(value) => value.to_string(),
                DataType::Bool(value) => value.to_string(),
                DataType::Text(value) => value.to_string(),
                _ => "DataType".to_string(),
            }
        )
    }
}



/// Creates a document to a collection
pub fn create_document(
    file_path: &Path,
    collection_name: &str, 
    data: HashMap<String, DataType>,
) -> Result<(), Box<dyn Error>>
{
    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        let mut collection_index = None;

        // Find collection index
        for (index, collection) in database
            .collections()
            .iter()
            .enumerate()
        {
            if collection.name() == collection_name {
                collection_index = Some(index);
            }
        }

        if let Some(collection_index) = collection_index {
            let mut document = Document::from(&mut database);
            document.data = data;

            if let Some(collection) = database
                .collections_mut()
                .get_mut(collection_index)
            {
                collection.documents_mut().push(document);

                match write_database_json(&database, file_path) {
                    Ok(()) => return Ok(()),
                    Err(e) => return Err(e.into()),
                }
            } else {
                return Err(Box::new(CollectionError::NotFound));
            }
        } else {
            return Err(Box::new(CollectionError::NotFound));
        }
    } else {
        return Err(Box::new(DatabaseError::NotFound));
    }
}

/// Deletes a document from a collection by document id.
/// 
/// This is a faster way to delete a document
/// if the collection is known beforehand.

/* DISABLED. NOT NEEDED RIGHT NOW.
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
*/

/// Deletes a document from database by document id.
/// 
/// Goes through all collections until id is found.
pub fn delete_document(
    file_path: &Path,
    document_id: &u64
) -> Result<(), Box<dyn Error>>
{
    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        let mut found = false;

        for collection in database.collections_mut() {
            if let Some(document) = collection
                .documents()
                .iter()
                .find(|document| document.id() == &DataType::DocumentId(*document_id))
            {
                collection
                    .documents_mut()
                    .retain(|document| document.id() != &DataType::DocumentId(*document_id));
                found = true;
            };
        }

        if found {
            match write_database_json(&database, file_path) {
                Ok(()) => return Ok(()),
                Err(e) => return Err(e.into()),
            }
        } else {
            return Err(Box::new(DocumentError::NotFound));
        }
    } else {
        return Err(Box::new(DatabaseError::NotFound));
    }
}

/// Finds all documents from a collection
pub fn find_all_documents_from_collection(
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
                        collection.name.to_string(),
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
/// Returns the document if it was found.
pub fn find_document_by_id(
    document_id: &u64,
    file_path: &Path,
) -> Result<Option<FormattedDocument>, Box<dyn Error>>
{
    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(&contents)?;

        for collection in database.collections.into_iter() {
            for document in collection.documents.into_iter() {
                if document.id() == &DataType::DocumentId(*document_id) {
                    let formatted_document = FormattedDocument::from(
                        document.id,
                        collection.name,
                        document.data,
                    );

                    return Ok(Some(formatted_document));
                }
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

    fn insert_document_test_data(data: &mut HashMap<String, DataType>) {
        data.insert(
            String::from("first_name"),
            DataType::Text(String::from("John"))
        );
        data.insert(
            String::from("last_name"),
            DataType::Text(String::from("Smith"))
        );
        data.insert(
            String::from("age"),
            DataType::Int32(30)
        );
    }

    #[test]
    fn test_create_document_to_collection() {
        let mut database = Database::from("test");
        let collection_name = "test_collection";
        database.collections_mut().push(DocumentCollection::from(collection_name));
        let json = serde_json::to_string_pretty(&database).unwrap();

        let mut data = HashMap::new();
        insert_document_test_data(&mut data);
        assert!(data.len() > 0);

        let mut document = Document::from(&mut database);
        document.data = data.clone();
        database
            .collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .push(document);
        let expected_json = serde_json::to_string_pretty(&database).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write(json.as_bytes()).is_ok());
        assert!(create_document(
            &file_path,
            collection_name,
            data
        ).is_ok());

        let buf = fs::read_to_string(&file_path).unwrap();
        assert_eq!(buf, expected_json);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }

    /*
    #[test]
    fn test_delete_document_from_collection() {
        assert!(false);
    }
    */

    #[test]
    fn test_delete_document() {
        let mut database = Database::from("test");
        let collection_name = "test_collection";
        database.collections_mut().push(DocumentCollection::from(collection_name));
        
        let mut data = HashMap::new();
        insert_document_test_data(&mut data);
        assert!(data.len() > 0);

        let mut document = Document::from(&mut database);
        document.data = data.clone();
        database
            .collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .push(document);
        let json = serde_json::to_string_pretty(&database).unwrap();

        database
            .collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .remove(0);
        let expected_json = serde_json::to_string_pretty(&database).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write(json.as_bytes()).is_ok());
        assert!(delete_document(&file_path, &1).is_ok());

        let buf = fs::read_to_string(&file_path).unwrap();
        assert_eq!(buf, expected_json);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }

    #[test]
    fn test_find_all_documents_of_collection() {
        let mut database = Database::from("test");
        let collection_name = "test_collection";
        database.collections_mut().push(DocumentCollection::from(collection_name));
        let json = serde_json::to_string_pretty(&database).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();
        assert!(file.write(json.as_bytes()).is_ok());

        let documents = find_all_documents_from_collection(
            &file_path,
            collection_name
        ).unwrap();
        assert!(documents.len() == 0);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }

    #[test]
    fn test_find_document_by_id() {
        let mut database = Database::from("test");
        let collection_name = "test_collection";
        database.collections_mut().push(DocumentCollection::from(collection_name));
        let mut document = Document::from(&mut database);

        assert_eq!(document.id(), &DataType::DocumentId(1));
        database
            .collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .push(document);
        let json = serde_json::to_string_pretty(&database).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();
        assert!(file.write(json.as_bytes()).is_ok());

        let document = find_document_by_id(&1, &file_path).unwrap();
        assert!(document.is_some());
        assert_eq!(document.unwrap().id(), &DataType::DocumentId(1));

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }
}
