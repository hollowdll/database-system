// Database Protocol Buffers module

use std::{
    io,
    fs,
    path::Path,
    error::Error,
};
use crate::db::{
    error::DatabaseError,
    pb,
    serialize_database,
    deserialize_database,
    write_database_to_file,
};

impl From<&str> for pb::Database {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::new(),
            collections: Vec::new(),
            id_count: 0,
        }
    }
}

impl From<(&str, &str)> for pb::Database {
    fn from((name, description): (&str, &str)) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            collections: Vec::new(),
            id_count: 0,
        }
    }
}

/// Creates a database file and writes initial data to it.
pub fn create_database_file(
    db_name: &str,
    file_path: &Path
) -> Result<(), Box<dyn Error>>
{
    if file_path.is_file() {
        return Err(Box::new(DatabaseError::Exists))
    }

    let file = fs::File::create(file_path)?;
    let database = pb::Database::from(db_name);
    let buf = serialize_database(&database)?;

    match write_database_to_file(&buf, file_path) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(e.into()),
    }
}
