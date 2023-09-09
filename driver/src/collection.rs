use std::marker::PhantomData;
use engine::storage::error::DatabaseOperationError;
use crate::{
    client::DatabaseClient,
    database::Database
};

/// Collection API.
/// 
/// This offers methods to work with a collection's documents.
/// 
/// T = type for this collection's document model.
pub struct Collection<'a, T> {
    client: &'a DatabaseClient,
    database: &'a Database<'a>,
    pub name: String,
    document_model: PhantomData<T>,
}

impl<'a, T> Collection<'a, T> {
    pub fn new(client: &'a DatabaseClient, database: &'a Database, name: &str) -> Collection<'a, T> {
        Collection {
            client,
            database,
            name: name.to_string(),
            document_model: PhantomData,
        }
    }

    /// Finds all documents in this collection.
    pub fn find_all() -> Result<Vec<T>, DatabaseOperationError> {
        Ok(Vec::new())
    }

    /// Finds a document by id from this collection.
    pub fn find_one_by_id(id: &u64) -> Result<Option<T>, DatabaseOperationError> {
        Ok(None)
    }

    /// Inserts a document to this collection.
    /// 
    /// Returns the id of the inserted document.
    pub fn insert_one(document: T) -> Result<u64, DatabaseOperationError> {
        Ok(0)
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
