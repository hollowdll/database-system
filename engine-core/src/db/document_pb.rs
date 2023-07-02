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
    id: DataType,
    collection: String,
    data: HashMap<String, DataType>,
}

impl DocumentDto {
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

impl From<(DataType, String, HashMap<String, DataType>)> for DocumentDto {
    fn from(
        (id, collection, data):
        (DataType, String, HashMap<String, DataType>)
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
