use std::error::Error;
use std::fmt;

pub const UNEXPECTED_ERROR: &str = "Unexpected error";
pub const DATA_NOT_RECEIVED: &str = "Data expected but not received";

/// Error type for database client errors.
#[derive(Debug)]
pub struct DatabaseClientError {
    pub kind: DatabaseClientErrorKind,
    pub message: String,
}

/// Kind of database client error.
#[derive(Debug)]
pub enum DatabaseClientErrorKind {
    /// Unexpected error.
    Unexpected,
    /// Database not found.
    DatabaseNotFound,
    /// Collection not found.
    CollectionNotFound,
    /// Failed to connect to database.
    ConnectionFailed,
    /// Failed to get database.
    GetDatabase,
    /// Failed to get collection.
    GetCollection,
    /// Failed to create database.
    CreateDatabase,
    /// Failed to create collection.
    CreateCollection,
    /// Failed to insert document.
    InsertOneDocument,
    /// Failed to replace document.
    ReplaceOneDocument,
    /// Failed to delete document.
    DeleteOneDocument,
    /// Failed to delete documents.
    DeleteManyDocuments,
    /// Failed to find all documents.
    FindAllDocuments,
    /// Failed to find document.
    FindOneDocument,
    /// Failed to find documents.
    FindManyDocuments,
}

impl fmt::Display for DatabaseClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            match self.kind {
                DatabaseClientErrorKind::Unexpected => "Unexpected error",
                DatabaseClientErrorKind::DatabaseNotFound => "Database was not found",
                DatabaseClientErrorKind::CollectionNotFound => "Collection was not found",
                DatabaseClientErrorKind::ConnectionFailed => "Failed to connect to database",
                DatabaseClientErrorKind::GetDatabase => "Failed to get database",
                DatabaseClientErrorKind::GetCollection => "Failed to get collection",
                DatabaseClientErrorKind::CreateDatabase => "Failed to create database",
                DatabaseClientErrorKind::CreateCollection => "Failed to create collection",
                DatabaseClientErrorKind::InsertOneDocument => "Failed to insert document",
                DatabaseClientErrorKind::ReplaceOneDocument => "Failed to replace document",
                DatabaseClientErrorKind::DeleteOneDocument => "Failed to delete document",
                DatabaseClientErrorKind::DeleteManyDocuments => "Failed to delete documents",
                DatabaseClientErrorKind::FindAllDocuments => "Failed to find all documents",
                DatabaseClientErrorKind::FindOneDocument => "Failed to find document",
                DatabaseClientErrorKind::FindManyDocuments => "Failed to find documents",
            },
            self.message,
        )
    }
}

impl DatabaseClientError {
    pub fn new(kind: DatabaseClientErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

impl Error for DatabaseClientError {}