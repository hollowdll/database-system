// Document Protocol Buffers module

use std::{
    io,
    fs,
    path::Path,
    error::Error,
    collections::HashMap,
    fmt,
};
use crate::db::{
    error::{
        DatabaseError,
        CollectionError,
        DocumentError,
    },
    pb,
    serialize_database,
    deserialize_database,
    write_database_to_file,
    DB_FILE_EXTENSION,
};

impl pb::Document {
    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn data(&self) -> &HashMap<String, pb::document::DataType> {
        &self.data
    }
}

impl pb::Document {
    /// Creates a new document.
    /// 
    /// Increments the database's `id_count` by 1 so each document gets a unique id.
    pub fn new(database: &mut pb::Database) -> Self {
        database.id_count += 1;
        Self {
            id: database.id_count,
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
    collection: String,
    data: HashMap<String, pb::document::DataType>,
}

impl DocumentDto {
    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn data(&self) -> &HashMap<String, pb::document::DataType> {
        &self.data
    }
}

impl From<(u64, String, HashMap<String, pb::document::DataType>)> for DocumentDto {
    fn from(
        (id, collection, data):
        (u64, String, HashMap<String, pb::document::DataType>)
    ) -> Self
    {
        Self {
            id,
            collection,
            data,
        }
    }
}

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

/// Creates a document to a collection.
/// 
/// Writes the modified database to a file.
pub fn create_document_to_db_file(
    file_path: &Path,
    collection_name: &str, 
    data: HashMap<String, pb::document::DataType>,
) -> Result<(), Box<dyn Error>>
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
        let mut document = pb::Document::new(&mut database);
        document.data = data;

        if let Some(collection) = database
            .collections_mut()
            .get_mut(collection_index)
        {
            collection.documents_mut().push(document);
            let buf = serialize_database(&database)?;

            match write_database_to_file(&buf, file_path) {
                Ok(()) => return Ok(()),
                Err(e) => return Err(e.into()),
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
                let document_dto = DocumentDto::from((
                    document.id,
                    collection.name.to_string(),
                    document.data,
                ));

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
                    let document_dto = DocumentDto::from((
                        document.id,
                        collection.name,
                        document.data,
                    ));
    
                    return Ok(Some(document_dto));
                }
            }

            return Ok(None);
        }
    }

    Err(Box::new(CollectionError::NotFound))
}
