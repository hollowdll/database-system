use std::collections::HashMap;

/// Data type for document id.
/// 
/// DocumentId is an unsigned 64-bit integer.
pub struct DocumentId(pub u64);

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
}
