// Database related error types

use std::error::Error;
use std::fmt;

// #[derive(Debug)]
// pub struct DatabaseError(pub String);

/// Error type for database errors
#[derive(Debug)]
pub enum DatabaseError {
    Exists,
    NotFound,
}

/*
impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}*/

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
pub struct CollectionError(pub String);

/// Error type for document errors.
pub struct DocumentError(pub String);
