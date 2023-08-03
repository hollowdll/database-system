use std::{
    io,
    fs,
    path::Path,
    error::Error,
    collections::HashMap,
    fmt,
};
use crate::storage::{
    error::{
        DatabaseError,
        CollectionError,
        DocumentError,
    },
    pb,
    pb::document::DataType,
    serialize_database,
    deserialize_database,
    write_database_to_file,
    DB_FILE_EXTENSION,
};

// Implements methods for protobuf type
impl pb::Document {
    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }

    /// Creates a new document.
    /// 
    /// Increases the collection's `id_count` by 1
    /// so each document gets a unique id in the collection.
    pub fn new(collection: &mut pb::Collection) -> Self {
        collection.id_count += 1;
        Self {
            id: collection.id_count,
            data: HashMap::new(),
        }
    }
}

/// Document data transfer object (DTO).
/// 
/// Exposes document data that clients can use.
#[derive(Debug)]
pub struct DocumentDto {
    id: u64,
    data: HashMap<String, DataType>,
}

impl DocumentDto {
    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }

    /// Creates a new instance of `DocumentDto`.
    pub fn new(id: u64, data: &HashMap<String, DataType>) -> Self {
        Self {
            id,
            data: data.to_owned(),
        }
    }
}

/* Disabled for now
/// Data type for document fields
#[derive(Debug, PartialEq, Clone)]
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
*/

/// Creates a document to a collection.
/// 
/// Writes the modified database to a file.
/// 
/// Returns the created document.
pub fn create_document_to_db_file(
    file_path: &Path,
    collection_name: &str, 
    data: HashMap<String, DataType>,
) -> Result<DocumentDto, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut database = deserialize_database(&fs::read(file_path)?)?;
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
        if let Some(collection) = database
            .collections_mut()
            .get_mut(collection_index)
        {
            let mut document = pb::Document::new(collection);
            document.data = data;
            let document_dto = DocumentDto::new(
                document.id,
                &document.data
            );

            collection.documents_mut().push(document);
            let buf = serialize_database(&database)?;

            match write_database_to_file(&buf, file_path) {
                Ok(()) => return Ok(document_dto),
                Err(e) => return Err(e.into()),
            }
        }
    }

    Err(Box::new(CollectionError::NotFound))
}

/// Replaces a document's data. Keeps the document id.
/// 
/// Writes the modified database to a file.
pub fn replace_document_in_db_file(
    file_path: &Path,
    document_id: &u64,
    collection_name: &str,
    data: HashMap<String, DataType>,
) -> Result<(), Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut database = deserialize_database(&fs::read(file_path)?)?;

    for collection in database.collections_mut() {
        if collection.name() == collection_name {
            if let Some(document) = collection
                .documents_mut()
                .iter_mut()
                .find(|document| document.id() == document_id)
            {
                document.data = data;
                let buf = serialize_database(&database)?;

                match write_database_to_file(&buf, file_path) {
                    Ok(()) => return Ok(()),
                    Err(e) => return Err(e.into()),
                }
            } else {
                return Err(Box::new(DocumentError::NotFound));
            }
        }
    }

    Err(Box::new(CollectionError::NotFound))
}

/// Deletes a document from a collection by document id.
/// 
/// Writes the modified database to a file.
pub fn delete_document_from_db_file(
    file_path: &Path,
    document_id: &u64,
    collection_name: &str,
) -> Result<(), Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut database = deserialize_database(&fs::read(file_path)?)?;

    for collection in database.collections_mut() {
        if collection.name() == collection_name {
            if let Some(document) = collection
                .documents()
                .iter()
                .find(|document| document.id() == document_id)
            {
                collection
                    .documents_mut()
                    .retain(|document| document.id() != document_id);
                let buf = serialize_database(&database)?;

                match write_database_to_file(&buf, file_path) {
                    Ok(()) => return Ok(()),
                    Err(e) => return Err(e.into()),
                }
            } else {
                return Err(Box::new(DocumentError::NotFound));
            }
        }
    }

    Err(Box::new(CollectionError::NotFound))
}

/// Finds all documents from a collection.
/// 
/// Returns the found documents.
pub fn find_all_documents_from_collection(
    file_path: &Path,
    collection_name: &str
) -> Result<Vec<DocumentDto>, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut database = deserialize_database(&fs::read(file_path)?)?;
    let mut documents = Vec::new();

    for collection in database.collections.into_iter() {
        if collection.name() == collection_name {
            for document in collection.documents.into_iter() {
                let document_dto = DocumentDto {
                    id: document.id,
                    data: document.data,
                };

                documents.push(document_dto)
            }

            return Ok(documents);
        }
    }

    Err(Box::new(CollectionError::NotFound))
}

/// Finds the first documents from a collection specified by limit.
/// 
/// Returns the found documents.
pub fn find_documents_from_collection_limit(
    file_path: &Path,
    collection_name: &str,
    limit: usize,
) -> Result<Vec<DocumentDto>, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut database = deserialize_database(&fs::read(file_path)?)?;
    let mut documents = Vec::new();

    for collection in database.collections.into_iter() {
        if collection.name() == collection_name {
            for document in collection.documents.into_iter() {
                if documents.len() >= limit {
                    return Ok(documents)
                }

                let document_dto = DocumentDto {
                    id: document.id,
                    data: document.data,
                };

                documents.push(document_dto)
            }

            return Ok(documents);
        }
    }

    Err(Box::new(CollectionError::NotFound))
}

/// Finds a document from a collection by document id.
/// 
/// Returns the found document.
pub fn find_document_from_collection_by_id(
    file_path: &Path,
    document_id: &u64,
    collection_name: &str,
) -> Result<Option<DocumentDto>, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut database = deserialize_database(&fs::read(file_path)?)?;

    for collection in database.collections.into_iter() {
        if collection.name() == collection_name {
            for document in collection.documents.into_iter() {
                if document.id() == document_id {
                    let document_dto = DocumentDto {
                        id: document.id,
                        data: document.data,
                    };
    
                    return Ok(Some(document_dto));
                }
            }

            return Ok(None);
        }
    }

    Err(Box::new(CollectionError::NotFound))
}



/*
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
*/
