// This module contains lower level code to do operations to database files.
// Contains file system access, and database file reading and writing.

#![allow(unused)]

pub mod database;
pub mod collection;
pub mod document;
pub mod data_type;

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};
use crate::constants::{
    DATABASES_DIR_PATH,
    DATABASE_FILE_EXTENSION,
};
pub use crate::db::{
    data_type::DataType,
    database::*,
    collection::*,
    document::*,
};

/// Gets database file path. Database files have JSON format.
fn database_file_path(database_name: &str) -> String {
    format!("{DATABASES_DIR_PATH}/{database_name}.{DATABASE_FILE_EXTENSION}")
}

/// Check if a database file exists in databases directory
fn database_file_exists(database_name: &str) -> bool {
    return Path::new(&database_file_path(database_name)).is_file();
}

/// Check if databases directory exists in project root
fn databases_dir_exists() -> bool {
    return Path::new(DATABASES_DIR_PATH).is_dir();
}

/// Creates databases directory in project directory
pub fn create_databases_dir() -> io::Result<()> {
    if !databases_dir_exists() {
        fs::create_dir(DATABASES_DIR_PATH)?;
    }

    Ok(())
}

/// Writes database as JSON to database file
fn write_database_json(database: &Database, file_path: &str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(&database)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)?;

    file.write(json.as_bytes())?;

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_file_path() {
        let database_name = "test_db_123";
        let file_path = format!("{DATABASES_DIR_PATH}/{database_name}.{DATABASE_FILE_EXTENSION}");

        assert_eq!(file_path, database_file_path(database_name));
    }
}
