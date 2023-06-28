// This module contains lower level code to do operations to database files.
// Contains file system access, and database file reading and writing.

#![allow(unused)]

pub mod database;
pub mod collection;
pub mod document;
pub mod error;

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{
        Path,
        PathBuf,
    },
};
use crate::constants::{
    DB_DIR_PATH,
    DB_FILE_EXTENSION,
};
pub use crate::db::{
    database::*,
    collection::*,
    document::*,
};
use self::error::DatabaseError;
use prost::Message;

/// This module contains Protocol Buffers types.
pub mod pb {
    // Include generated Rust code from compiled .proto files.
    include!(concat!(env!("OUT_DIR"), "/pb.rs"));
}

/// Creates databases directory if it doesn't exist
pub fn create_db_dir_if_not_exists(path: &Path) -> io::Result<()> {
    if !path.is_dir() {
        fs::create_dir(path)?;
    }

    Ok(())
}

/// Writes database as JSON to database file
fn write_database_json(database: &Database, file_path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(&database)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    file.write(json.as_bytes())?;

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{
            self,
            Write,
            Read,
            Seek,
            SeekFrom
        },
        fs::File,
    };
    use tempfile::tempdir;

    #[test]
    fn test_create_db_dir_if_not_exists() {
        let base_dir = tempdir().unwrap();
        let new_dir = base_dir.path().join("test");

        assert!(create_db_dir_if_not_exists(new_dir.as_path()).is_ok());
        assert!(new_dir.is_dir());

        base_dir.close().expect("Failed to clean up tempdir before dropping.");
    }

    #[test]
    fn test_write_database_json() {
        let database = Database::from("test");
        let json = serde_json::to_string_pretty(&database).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let file = File::create(&file_path).unwrap();

        assert!(write_database_json(&database, &file_path).is_ok());
        let buf = fs::read_to_string(&file_path).unwrap();
        assert_eq!(buf, json);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }
}
