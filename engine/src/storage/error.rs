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

    /// Database has empty name which is not allowed.
    EmptyName,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DatabaseError::Exists => "Database already exists",
                DatabaseError::NotFound => "Database was not found",
                DatabaseError::EmptyName => "Database cannot have empty name",
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

    /// Collection has empty name which is not allowed.
    EmptyName,
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
                CollectionError::EmptyName => "Collection cannot have empty name",
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

    /// Document has an empty field name.
    EmptyFieldName,
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DocumentError::NotFound => "Document was not found",
                DocumentError::EmptyFieldName => "Document has an empty field name",
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
                ParseError::Unknown => "a type that does not exist",
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

/// Error type for database operation failures.
/// 
/// This is a verbose version of `DatabaseOperationError`
/// with separated error kind and message.
#[derive(Debug)]
pub struct DatabaseOperationVerboseError {
    /// Database operation that failed.
    pub kind: DatabaseOperationErrorKind,

    /// Message telling the cause of error.
    pub message: String,
}

#[derive(Debug)]
pub enum DatabaseOperationErrorKind {
    /// Failed to create database.
    CreateDatabase,

    /// Failed to delete database.
    DeleteDatabase,

    /// Failed to modify database.
    ModifyDatabase,

    /// Failed to find one database.
    FindDatabaseOne,

    /// Failed to find many databases.
    FindDatabaseMany,

    /// Failed to create collection.
    CreateCollection,

    /// Failed to delete collection.
    DeleteCollection,

    /// Failed to find one collection.
    FindCollectionOne,

    /// Failed to find many collections.
    FindCollectionMany,

    /// Failed to create document.
    CreateDocument,

    /// Failed to delete document.
    DeleteDocument,

    /// Failed to replace document.
    ReplaceDocument,

    /// Failed to find one document.
    FindDocumentOne,

    /// Failed to find many documents.
    FindDocumentMany,
}

impl fmt::Display for DatabaseOperationVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            match &self.kind {
                DatabaseOperationErrorKind::CreateDatabase => "Failed to create database",
                DatabaseOperationErrorKind::DeleteDatabase => "Failed to delete database",
                DatabaseOperationErrorKind::ModifyDatabase => "Failed to modify database",
                DatabaseOperationErrorKind::FindDatabaseOne => "Failed to find database",
                DatabaseOperationErrorKind::FindDatabaseMany => "Failed to find databases",
                DatabaseOperationErrorKind::CreateCollection => "Failed to create collection",
                DatabaseOperationErrorKind::DeleteCollection => "Failed to delete collection",
                DatabaseOperationErrorKind::FindCollectionOne => "Failed to find collection",
                DatabaseOperationErrorKind::FindCollectionMany => "Failed to find collections",
                DatabaseOperationErrorKind::CreateDocument => "Failed to create document",
                DatabaseOperationErrorKind::DeleteDocument => "Failed to delete document",
                DatabaseOperationErrorKind::ReplaceDocument => "Failed to replace document",
                DatabaseOperationErrorKind::FindDocumentOne => "Failed to find document",
                DatabaseOperationErrorKind::FindDocumentMany => "Failed to find documents",
            },
            &self.message
        )
    }
}

impl DatabaseOperationVerboseError {
    /// Creates a new `DatabaseOperationVerboseError`.
    pub fn new(kind: DatabaseOperationErrorKind, message: String) -> Self {
        Self {
            kind,
            message
        }
    }
}

impl Error for DatabaseOperationVerboseError {}
