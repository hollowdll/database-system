use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::db::DataType;

/// Database document that holds
/// data in key-value pairs
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: u64,
    pub data: HashMap<String, DataType>,
}

impl Document {
    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }
}

impl Document {
    pub fn from(id_count: u64) -> Self {
        Self {
            id: id_count,
            data: HashMap::new(),
        }
    }
}

/// Formatted document that can be listed in clients
#[derive(Debug)]
pub struct FormattedDocument {
    id: u64,
    data: HashMap<String, DataType>,
}

impl FormattedDocument {
    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn data(&self) -> &HashMap<String, DataType> {
        &self.data
    }
}

impl FormattedDocument {
    pub fn from(id: u64, data: HashMap<String, DataType>) -> Self {
        Self {
            id,
            data,
        }
    }
}