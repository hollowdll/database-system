// This module contains lower level code to do operations to database files.
// Contains file system access, and database file reading and writing.

#![allow(unused)]

pub mod database;
pub mod collection;
pub mod document;
pub mod error;

/// This module contains Protocol Buffers types.
pub mod pb {
    // Include generated code from compiled .proto files.
    include!(concat!(env!("OUT_DIR"), "/pb.rs"));
}

use std::{
    fs::{self, OpenOptions},
    io::{self, Write, Cursor},
    path::{
        Path,
        PathBuf,
    },
};
use self::error::DatabaseError;
use prost::{
    Message,
    EncodeError,
    DecodeError,
};

/// Database file extension.
pub const DB_FILE_EXTENSION: &str = "db";

/// Creates databases directory if it doesn't exist
pub fn create_db_dir_if_not_exists(path: &Path) -> io::Result<()> {
    if !path.is_dir() {
        fs::create_dir(path)?;
    }

    Ok(())
}

/// Serializes database to a buffer.
/// The buffer can be used to write the database to a file.
fn serialize_database(database: &pb::Database) -> Result<Vec<u8>, EncodeError> {
    let mut buf = Vec::new();
    buf.reserve(database.encoded_len());
    database.encode(&mut buf)?;

    Ok(buf)
}

/// Deserializes database from a buffer.
fn deserialize_database(buf: &[u8]) -> Result<pb::Database, DecodeError> {
    pb::Database::decode(&mut Cursor::new(buf))
}

/// Writes database buffer to a file.
fn write_database_to_file(buf: &[u8], file_path: &Path) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(buf)?;

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
}
