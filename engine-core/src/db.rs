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
    DB_DIR_PATH,
    TEMP_DB_DIR_PATH,
    DB_FILE_EXTENSION,
};
pub use crate::db::{
    data_type::DataType,
    database::*,
    collection::*,
    document::*,
};

/// Gets database file path.
pub fn database_file_path(database_name: &str) -> String {
    format!("{DB_DIR_PATH}/{database_name}.{DB_FILE_EXTENSION}")
}

/// Gets temporary database file path.
pub fn temp_database_file_path(database_name: &str) -> String {
    format!("{TEMP_DB_DIR_PATH}/{database_name}.{DB_FILE_EXTENSION}")
}

/// Check if a database file exists in databases directory
fn database_file_exists(database_name: &str) -> bool {
    return Path::new(&database_file_path(database_name)).is_file();
}

/// Check if databases directory exists
fn databases_dir_exists() -> bool {
    return Path::new(DB_DIR_PATH).is_dir();
}

/// Creates databases directory if it doesn't exist
pub fn create_databases_dir_if_not_exists() -> io::Result<()> {
    if !databases_dir_exists() {
        fs::create_dir(DB_DIR_PATH)?;
    }

    Ok(())
}

/// Creates temporary databases directory if it doesn't exist
pub fn create_temp_databases_dir_if_not_exists() -> io::Result<()> {
    if !Path::new(TEMP_DB_DIR_PATH).is_dir() {
        fs::create_dir(TEMP_DB_DIR_PATH)?;
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
    use std::io::{self, Write, Read, Seek, SeekFrom};
    use tempfile::tempfile;

    #[test]
    fn test_database_file_path() {
        let database_name = "test_database_file_path";
        let file_path = format!("{DB_DIR_PATH}/{database_name}.{DB_FILE_EXTENSION}");

        assert_eq!(file_path, database_file_path(database_name));
    }

    #[test]
    fn test_temp_database_file_path() {
        let database_name = "test_temp_database_file_path";
        let file_path = format!("{TEMP_DB_DIR_PATH}/{database_name}.{DB_FILE_EXTENSION}");

        assert_eq!(file_path, temp_database_file_path(database_name));
    }

    #[test]
    fn test_create_databases_dir_if_not_exists() {
        create_databases_dir_if_not_exists().unwrap();

        assert_eq!(Path::new(&format!("{DB_DIR_PATH}")).is_dir(), true);
    }

    #[test]
    fn test_create_temp_databases_dir_if_not_exists() {
        create_temp_databases_dir_if_not_exists().unwrap();

        assert_eq!(Path::new(&format!("{TEMP_DB_DIR_PATH}")).is_dir(), true);
    }

    #[test]
    fn test_write_database_json() {
        let database = Database::from("test_write_database_json");
        let json = serde_json::to_string_pretty(&database).unwrap();

        // Create tempfile securely without relying on file paths.
        let mut file = tempfile().unwrap();
        assert!(file.write(json.as_bytes()).is_ok());

        // Seek to start. This is needed to read the file.
        assert!(file.seek(SeekFrom::Start(0)).is_ok());

        let mut buf = String::new();
        assert!(file.read_to_string(&mut buf).is_ok());
        assert_eq!(buf, json);
    }
}
