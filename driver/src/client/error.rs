use std::error::Error;
use std::fmt;

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

    /// Database not found with the connection string.
    NotFound,

    /// Failed to connect to database.
    ConnectionFailed,
}

impl fmt::Display for DatabaseClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            match self.kind {
                DatabaseClientErrorKind::Unexpected => "Unexpected error",
                DatabaseClientErrorKind::NotFound => "No database found with this connection string",
                DatabaseClientErrorKind::ConnectionFailed => "Failed to connect to database",
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