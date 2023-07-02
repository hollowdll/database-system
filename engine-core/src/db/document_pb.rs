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


