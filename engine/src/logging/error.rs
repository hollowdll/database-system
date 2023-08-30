use std::error::Error;
use std::fmt;

/// Error that can occur when logging to log files.
#[derive(Debug)]
pub struct LogError {
    kind: LogErrorKind,
    message: String,
}

/// Kind of log error.
#[derive(Debug)]
pub enum LogErrorKind {
    /// Failed to create logs directory.
    CreateDir,

    /// Failed to create log file.
    CreateFile,

    /// Failed to write to log file.
    WriteFile,
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Logging error: {}: {}",
            match self.kind {
                LogErrorKind::CreateDir => "Failed to create logs directory",
                LogErrorKind::CreateFile => "Failed to create log file",
                LogErrorKind::WriteFile => "Failed to write to log file",
            },
            self.message
        )
    }
}

impl LogError {
    pub fn new(kind: LogErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

impl Error for LogError {}
