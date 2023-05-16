use crate::db::collection::DocumentCollection;
use serde::{Serialize, Deserialize};

/// Database structure for database files
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    name: String,
    pub description: String,
    pub collections: Vec<DocumentCollection>,
    pub id_count: u64,
}

impl Database {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut str {
        &mut self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn collections(&self) -> &Vec<DocumentCollection> {
        &self.collections
    }

    pub fn collections_mut(&mut self) -> &mut Vec<DocumentCollection> {
        &mut self.collections
    }

    pub fn id_count(&self) -> &u64 {
        &self.id_count
    }
}

impl From<&str> for Database {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::new(),
            collections: Vec::new(),
            id_count: 0,
        }
    }
}

impl From<(&str, &str)> for Database {
    fn from((name, description): (&str, &str)) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            collections: Vec::new(),
            id_count: 0,
        }
    }
}

/// Formatted database that can be listed in clients.
/// 
/// Size = database file size in bytes.
pub struct FormattedDatabase {
    name: String,
    description: String,
    size: u64,
}

impl FormattedDatabase {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }
}

impl FormattedDatabase {
    pub fn from(name: String, description: String, size: u64) -> Self {
        Self {
            name,
            description,
            size,
        }
    }
}
