use std::{
    fs,
    path::Path,
    error::Error,
    collections::HashMap,
    fmt::{self, Display},
};
use crate::storage::{
    error::{
        DatabaseError,
        CollectionError,
        DocumentError,
    },
    pb,
    pb::document::DataType,
    pb::document::data_type,
    serialize_database,
    deserialize_database,
    write_database_to_file,
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

    /// Validates document by checking its data.
    /// 
    /// Returns any errors that may occur during the process.
    pub fn validate_errors(&self) -> Result<(), DocumentError> {
        for (key, _value) in self.data.iter() {
            if key.is_empty() {
                return Err(DocumentError::EmptyFieldName);
            }
        }

        Ok(())
    }
}

/// Document data transfer object (DTO).
/// 
/// Exposes document data that clients can use.
#[derive(Debug)]
pub struct DocumentDto {
    pub id: u64,
    pub data: HashMap<String, DataType>,
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

impl Display for DocumentDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut data = Vec::new();
        for (key, value) in self.data().iter() {
            // Get data type and value
            let (data_type, field_value) = match &value.data_type {
                Some(data_type::DataType::Int32(value)) => ("Int32", value.to_string()),
                Some(data_type::DataType::Int64(value)) => ("Int64", value.to_string()),
                Some(data_type::DataType::Decimal(value)) => ("Decimal", value.to_string()),
                Some(data_type::DataType::Bool(value)) => ("Bool", value.to_string()),
                Some(data_type::DataType::Text(value)) => ("Text", format!("\"{}\"", value)),
                _ => ("Invalid document data type", "Invalid value".to_string()),
            };
    
            data.push(format!("  [{}] \"{}\": {}", data_type, key, field_value));
        }

        write!(
            f,
            "{{\n  [DocumentId] _id: {}\n{}\n}}",
            self.id(),
            data.join("\n"),
        )
    }
}

/* Disabled for now. Currently defined in generated protocol buffers code.
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
/// Writes the modified database to the database file.
/// 
/// Returns the created document.
pub fn create_document_to_collection(
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
            if let Err(e) = document.validate_errors() {
                return Err(Box::new(e));
            }
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
/// Writes the modified database to the database file.
pub fn replace_document_in_collection(
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
                if let Err(e) = document.validate_errors() {
                    return Err(Box::new(e));
                }
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
/// Writes the modified database to the database file.
pub fn delete_document_from_collection(
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
            if let Some(_document) = collection
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

/// Deletes all documents from a collection.
/// 
/// Writes the modified database to the database file.
/// 
/// Returns the number of deleted documents.
pub fn delete_all_documents_from_collection(
    file_path: &Path,
    collection_name: &str,
) -> Result<usize, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut database = deserialize_database(&fs::read(file_path)?)?;

    for collection in database.collections_mut() {
        if collection.name() == collection_name {
            let deleted_count = collection.documents_mut()
                .drain(..)
                .count();
            let buf = serialize_database(&database)?;

            match write_database_to_file(&buf, file_path) {
                Ok(()) => return Ok(deleted_count),
                Err(e) => return Err(e.into()),
            }
        }
    }

    Err(Box::new(CollectionError::NotFound))
}

/// Finds all documents in a collection.
/// 
/// Limit specifies the maximum number of documents to return.
/// 
/// Returns the found documents.
pub fn find_all_documents_in_collection(
    file_path: &Path,
    collection_name: &str,
    limit: Option<usize>,
) -> Result<Vec<DocumentDto>, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let database = deserialize_database(&fs::read(file_path)?)?;
    let mut documents = Vec::new();

    for collection in database.collections.into_iter() {
        if collection.name() == collection_name {
            if let Some(limit) = limit {
                for document in collection.documents.into_iter() {
                    if documents.len() >= limit {
                        return Ok(documents)
                    }
                    documents.push(DocumentDto {
                        id: document.id,
                        data: document.data,
                    });
                }
            } else {
                for document in collection.documents.into_iter() {
                    documents.push(DocumentDto {
                        id: document.id,
                        data: document.data,
                    });
                }
            }
            return Ok(documents);
        }
    }

    Err(Box::new(CollectionError::NotFound))
}

/// Finds the first documents in a collection specified by limit.
/// 
/// Returns the found documents.
pub fn find_documents_in_collection_limit(
    file_path: &Path,
    collection_name: &str,
    limit: usize,
) -> Result<Vec<DocumentDto>, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let database = deserialize_database(&fs::read(file_path)?)?;
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

/// Finds a document in a collection by document id.
/// 
/// Returns the found document.
pub fn find_document_in_collection_by_id(
    file_path: &Path,
    document_id: &u64,
    collection_name: &str,
) -> Result<Option<DocumentDto>, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let database = deserialize_database(&fs::read(file_path)?)?;

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

/// Finds documents in a collection.
/// Accepts query to find specific documents.
/// 
/// The query contains fields of key-values that the document must match.
/// 
/// Limit specifies the maximum number of documents to return.
/// 
/// Returns the found documents.
pub fn find_documents_in_collection(
    file_path: &Path,
    collection_name: &str,
    query: &HashMap<String, data_type::DataType>,
    limit: Option<usize>,
) -> Result<Vec<DocumentDto>, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let database = deserialize_database(&fs::read(file_path)?)?;
    let mut documents = Vec::new();

    for collection in database.collections.into_iter() {
        if collection.name() == collection_name {
            if query.is_empty() {
                return Ok(documents);
            }

            if let Some(limit) = limit {
                for document in collection.documents.into_iter() {
                    if documents.len() >= limit {
                        return Ok(documents)
                    }
                    let mut fields_match = 0;
                    for (document_key, document_value) in document.data.iter() {
                        if let Some(query_value) = query.get(document_key) {
                            if let Some(document_value) = &document_value.data_type {
                                if query_value == document_value {
                                    fields_match += 1;
                                }
                                if fields_match == query.len() {
                                    documents.push(DocumentDto {
                                        id: document.id,
                                        data: document.data,
                                    });
                                    break
                                }
                            }
                        }
                    }
                }
            } else {
                for document in collection.documents.into_iter() {
                    let mut fields_match = 0;
                    for (document_key, document_value) in document.data.iter() {
                        if let Some(query_value) = query.get(document_key) {
                            if let Some(document_value) = &document_value.data_type {
                                if query_value == document_value {
                                    fields_match += 1;
                                }
                                if fields_match == query.len() {
                                    documents.push(DocumentDto {
                                        id: document.id,
                                        data: document.data,
                                    });
                                    break
                                }
                            }
                        }
                    }
                }
            }

            return Ok(documents);
        }
    }

    Err(Box::new(CollectionError::NotFound))
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;
    use std::fs::File;
    use crate::storage::{
        DB_FILE_EXTENSION,
        pb::{
            Database,
            Collection,
            Document,
            document::{
                DataType,
                data_type,
            },
        },
    };

    // Inserts some data to documents that are created in tests
    fn insert_document_test_data(data: &mut HashMap<String, DataType>) {
        data.insert(
            String::from("first_name"),
            DataType {
                data_type: Some(data_type::DataType::Text(String::from("John")))
            }
        );
        data.insert(
            String::from("last_name"),
            DataType {
                data_type: Some(data_type::DataType::Text(String::from("Smith")))
            }
        );
        data.insert(
            String::from("age"),
            DataType {
                data_type: Some(data_type::DataType::Int32(30))
            }
        );
    }

    #[test]
    fn test_create_document_to_collection() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        let collection = Collection::from(collection_name);
        db.collections_mut().push(collection);
        let db_buf = serialize_database(&db).unwrap();

        let mut data = HashMap::new();
        insert_document_test_data(&mut data);
        assert!(data.len() > 0);

        let mut document = Document::new(db.collections_mut()
            .get_mut(0)
            .unwrap());
        document.data = data.clone();
        db.collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .push(document);
        let expected_db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        let created_document = create_document_to_collection(
            &file_path,
            collection_name,
            data
        ).unwrap();
        assert!(created_document.id == 1);
        assert_eq!(fs::read(&file_path).unwrap(), expected_db_buf);

        drop(file);
        dir.close().unwrap();
    }

    
    #[test]
    fn test_replace_document_in_collection() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        let collection = Collection::from(collection_name);
        db.collections_mut().push(collection);

        let mut data = HashMap::new();
        insert_document_test_data(&mut data);
        assert!(data.len() > 0);

        let mut document = Document::new(db.collections_mut()
            .get_mut(0)
            .unwrap());
        document.data = data;
        db.collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .push(document);
        let db_buf = serialize_database(&db).unwrap();

        let mut new_data = HashMap::new();
        new_data.insert(
            String::from("name"),
            DataType {
                data_type: Some(data_type::DataType::Text(String::from("John Smith")))
            }
        );
        let document = db.collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .get_mut(0)
            .unwrap();
        document.data = new_data.clone();
        let expected_db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        assert!(replace_document_in_collection(&file_path, &1, collection_name, new_data).is_ok());
        assert_eq!(fs::read(&file_path).unwrap(), expected_db_buf);

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn test_delete_document_from_collection() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        let collection = Collection::from(collection_name);
        db.collections_mut().push(collection);
        
        let mut data = HashMap::new();
        insert_document_test_data(&mut data);
        assert!(data.len() > 0);

        let mut document = Document::new(db.collections_mut()
            .get_mut(0)
            .unwrap());
        document.data = data;
        db.collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .push(document);
        let db_buf = serialize_database(&db).unwrap();

        db.collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .swap_remove(0);
        let expected_db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        assert!(delete_document_from_collection(&file_path, &1, collection_name).is_ok());
        assert_eq!(fs::read(&file_path).unwrap(), expected_db_buf);

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn test_delete_all_documents_from_collection() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        let collection = Collection::from(collection_name);
        db.collections_mut().push(collection);

        for _ in 0..3 {
            let document = Document::new(db.collections_mut()
                .get_mut(0)
                .unwrap());
            db.collections_mut()
                .get_mut(0)
                .unwrap()
                .documents_mut()
                .push(document);
        }
        let document_count = db.collections()
            .get(0)
            .unwrap()
            .documents()
            .len();
        let db_buf = serialize_database(&db).unwrap();

        db.collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .clear();
        let expected_db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        let deleted_count = delete_all_documents_from_collection(&file_path, collection_name)
            .unwrap();
        assert_eq!(deleted_count, document_count);
        assert_eq!(fs::read(&file_path).unwrap(), expected_db_buf);

        drop(file);
        dir.close().unwrap();
    }
    
    #[test]
    fn test_find_all_documents_in_collection() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        let collection = Collection::from(collection_name);
        db.collections_mut().push(collection);

        let mut data = HashMap::new();
        insert_document_test_data(&mut data);
        assert!(data.len() > 0);

        let mut document = Document::new(db.collections_mut()
            .get_mut(0)
            .unwrap());
        document.data = data;
        db.collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .push(document);
        let db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        let documents = find_all_documents_in_collection(
            &file_path,
            collection_name,
            None,
        ).unwrap();
        assert!(documents.len() == 1);

        drop(file);
        dir.close().unwrap();
    }
    
    #[test]
    fn test_find_document_in_collection_by_id() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        let collection = Collection::from(collection_name);
        db.collections_mut().push(collection);

        let mut data = HashMap::new();
        insert_document_test_data(&mut data);
        assert!(data.len() > 0);

        let mut document = Document::new(db.collections_mut()
            .get_mut(0)
            .unwrap());
        document.data = data;
        db.collections_mut()
            .get_mut(0)
            .unwrap()
            .documents_mut()
            .push(document);
        let db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        let document = find_document_in_collection_by_id(
            &file_path,
            &1,
            collection_name
        ).unwrap();
        assert!(document.is_some());
        assert_eq!(document.unwrap().id, 1);

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn test_find_documents_in_collection() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        let collection = Collection::from(collection_name);
        db.collections_mut().push(collection);
        let max_documents = 4;

        for i in 1..=max_documents {
            let mut data = HashMap::new();
            if i == 1 {
                data.insert(
                    String::from("age"),
                    DataType {
                        data_type: Some(data_type::DataType::Int32(55))
                    }
                );
            } else {
                data.insert(
                    String::from("age"),
                    DataType {
                        data_type: Some(data_type::DataType::Int32(40))
                    }
                );
            }

            let mut document = Document::new(db.collections_mut()
                .get_mut(0)
                .unwrap());
            document.data = data;

            db.collections_mut()
                .get_mut(0)
                .unwrap()
                .documents_mut()
                .push(document);
        }
        let document_count = db.collections()
            .get(0)
            .unwrap()
            .documents()
            .len();
        let expected_document_count = max_documents - 1;
        let db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();
        let mut query = HashMap::new();
        query.insert("age".to_string(), data_type::DataType::Int32(40));

        assert!(file.write_all(&db_buf).is_ok());
        let documents = find_documents_in_collection(
            &file_path,
            collection_name,
            &query,
            None,
        ).unwrap();
        assert_ne!(documents.len(), document_count);
        assert_eq!(documents.len(), expected_document_count);

        drop(file);
        dir.close().unwrap();
    }
}

