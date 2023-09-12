use std::marker::PhantomData;
use engine::{
    storage::error::DatabaseOperationError,
    DocumentInputDataField,
};
use crate::{
    client::DatabaseClient,
    database::Database,
    document::DocumentModel,
};

/// Collection API.
/// 
/// This provides methods to work with a collection's documents.
/// 
/// T = type for this collection's document model.
pub struct Collection<'a> {
    client: &'a DatabaseClient,
    database: &'a Database<'a>,
    name: String,
}

impl<'a> Collection<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<'a> Collection<'a> {
    pub fn new(client: &'a DatabaseClient, database: &'a Database, name: &str) -> Collection<'a> {
        Collection {
            client,
            database,
            name: name.to_string(),
        }
    }

    /// Finds all documents in this collection.
    pub fn find_all() -> Result<Vec<DocumentModel>, DatabaseOperationError> {
        Ok(Vec::new())
    }

    /// Finds a document by id from this collection.
    pub fn find_one_by_id(id: &u64) -> Result<Option<DocumentModel>, DatabaseOperationError> {
        Ok(None)
    }

    /// Inserts a document to this collection.
    /// 
    /// Returns the id of the inserted document.
    pub fn insert_one(document: DocumentModel) -> Result<DocumentModel, DatabaseOperationError> {
        Ok(DocumentModel::new())
    }

    /// Replaces a document in this collection with a new one.
    /// 
    /// Only the data is replaced, id remains the same.
    pub fn replace_one_by_id(id: &u64) -> Result<(), DatabaseOperationError> {
        Ok(())
    }

    /// Deletes a document in this collection.
    pub fn delete_one_by_id(id: &u64) -> Result<(), DatabaseOperationError> {
        Ok(())
    }
}
