use serde::{Serialize, Deserialize};

/// Data type for document fields
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Int32(i32),
    Int64(i64),
    Decimal(f64),
    Bool(bool),
    Text(String),
    // Possibly more in the future
}