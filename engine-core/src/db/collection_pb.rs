// Collections Protocol Buffers module

use std::{
    io,
    fs,
    path::Path,
    error::Error,
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

impl pb::Collection {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn documents(&self) -> &Vec<pb::Document> {
        &self.documents
    }

    pub fn documents_mut(&mut self) -> &mut Vec<pb::Document> {
        &mut self.documents
    }
}

impl From<&str> for pb::Collection {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            documents: Vec::new(),
        }
    }
}

/// Checks if a collection exists in a database.
fn collection_exists(db: &pb::Database, collection_name: &str) -> bool {
    for collection in db.collections() {
        return collection.name() == collection_name
    }

    false
}

/// Creates a new collection to a database.
/// 
/// Writes the modified database to a file.
pub fn create_collection_to_db_file(
    collection_name: &str,
    file_path: &Path,
) -> Result<(), Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }

    let mut database = deserialize_database(&fs::read(file_path)?)?;
    if collection_exists(&database, collection_name) {
        return Err(Box::new(CollectionError::Exists));
    }

    let collection = pb::Collection::from(collection_name);
    database.collections_mut().push(collection);
    let buf = serialize_database(&database)?;

    match write_database_to_file(&buf, file_path) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(e.into()),
    }
}