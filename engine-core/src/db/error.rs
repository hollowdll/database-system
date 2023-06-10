// Database related error types

use std::error::Error;
use std::fmt;

/// Error type for database errors
#[derive(Debug)]
pub enum DatabaseError {
    /// Database already exists.
    Exists,

    /// Database was not found.
    NotFound,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DatabaseError::Exists => "Database already exists",
                DatabaseError::NotFound => "Database was not found",
            }
        )
    }
}

impl Error for DatabaseError {}

/// Error type for collection errors.
#[derive(Debug)]
pub enum CollectionError {
    /// Collection already exists.
    Exists,

    /// Collection was not found.
    NotFound,
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CollectionError::Exists => "Collection already exists",
                CollectionError::NotFound => "Collection was not found",
            }
        )
    }
}

impl Error for CollectionError {}

/// Error type for document errors.
#[derive(Debug)]
pub enum DocumentError {
    /// Document was not found.
    NotFound,
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DocumentError::NotFound => "Document was not found",
            }
        )
    }
}

impl Error for DocumentError {}
