use std::marker::PhantomData;

use engine::storage::error::DatabaseOperationError;
use crate::client::DatabaseClient;

/// Collection API.
/// 
/// This offers methods to work with a collection's documents.
/// 
/// T = type for this collection's document model.
pub struct Collection<'a, T> {
    client: &'a DatabaseClient,
    pub name: String,
    document_model: PhantomData<T>,
}

impl<'a, T> Collection<'a, T> {
    pub fn new(client: &'a DatabaseClient, name: &str) -> Collection<'a, T> {
        Collection {
            client,
            name: name.to_string(),
            document_model: PhantomData,
        }
    }

    /// Finds all documents in this collection.
    pub fn find_all() -> Result<Vec<T>, DatabaseOperationError> {
        // temporary
        Ok(Vec::new())
    }

    /// Finds a document by id from this collection.
    pub fn find_by_id(id: &u64) -> Result<Option<T>, DatabaseOperationError> {
        // temporary
        Ok(None)
    }

    // TODO
    // other methods
}
