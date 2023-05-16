use crate::db::document::Document;
use serde::{Serialize, Deserialize};

/// Database document collection
/// that holds database documents.
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentCollection {
    name: String,
    pub documents: Vec<Document>,
}

impl DocumentCollection {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn documents(&self) -> &Vec<Document> {
        &self.documents
    }

    pub fn documents_mut(&mut self) -> &mut Vec<Document> {
        &mut self.documents
    }
}

impl DocumentCollection {
    pub fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            documents: Vec::new(),
        }
    }
}

/// Formatted document collection that can be listed in clients
pub struct FormattedDocumentCollection {
    name: String,
}

impl FormattedDocumentCollection {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FormattedDocumentCollection {
    pub fn from(name: String) -> Self {
        Self {
            name
        }
    }
}