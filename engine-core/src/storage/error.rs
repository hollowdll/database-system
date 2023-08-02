// Database related error types

use std::error::Error;
use std::fmt;

/// Error type for database errors.
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

    /// Collection has documents.
    /// 
    /// This will occur when deleting a collection if it has documents.
    HasDocuments,
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CollectionError::Exists => "Collection already exists",
                CollectionError::NotFound => "Collection was not found",
                CollectionError::HasDocuments => "Collection has documents",
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

/// Error type for document data type parse errors.
#[derive(Debug)]
pub enum ParseError {
    Unknown,
    Int32,
    Int64,
    Decimal,
    Bool,
    Text,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse data into {}",
            match self {
                ParseError::Unknown => "an unknown type. The specified type does not exist",
                ParseError::Int32 => "'Int32'",
                ParseError::Int64 => "'Int64'",
                ParseError::Decimal => "'Decimal'",
                ParseError::Bool => "'Bool'",
                ParseError::Text => "'Text'",
            }
        )
    }
}

impl Error for ParseError {}

/// Error type for database operation failures.
/// 
/// This error can occur, for example, when the systems fails
/// to create or find a database.
#[derive(Debug)]
pub struct DatabaseOperationError(pub String);

impl fmt::Display for DatabaseOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Database operation failed: {}",
            self.0
        )
    }
}

impl Error for DatabaseOperationError {}
