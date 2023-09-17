use std::{collections::HashMap, fmt};

/// Data type for document id.
/// 
/// DocumentId is an unsigned 64-bit integer.
pub struct DocumentId(pub u64);

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Data type for document data fields.
pub enum DataType {
    /// 64-bit signed integer.
    Int64(i64),
    /// 32-bit signed integer.
    Int32(i32),
    /// 64-bit floating point number.
    Decimal(f64),
    /// Boolean that can be true or false.
    Bool(bool),
    /// UTF-8 string for dynamic text.
    Text(String),
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataType::Int64(value) => value.to_string(),
                DataType::Int32(value) => value.to_string(),
                DataType::Decimal(value) => value.to_string(),
                DataType::Bool(value) => value.to_string(),
                DataType::Text(value) => value.to_string(),
            }
        )
    }
}

/// Model for database documents.
/// 
/// Use this to create documents that can be saved to databases.
pub struct DocumentModel {
    pub id: DocumentId,
    pub data: HashMap<String, DataType>
}

impl DocumentModel {
    /// Creates a new document model.
    pub fn new() -> DocumentModel {
        DocumentModel {
            id: DocumentId(0),
            data: HashMap::new(),
        }
    }

    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }
}
