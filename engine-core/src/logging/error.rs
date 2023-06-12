// Errors types for logs

use std::error::Error;
use std::fmt;

/// Error that can occur when logging to log files.
#[derive(Debug)]
pub enum LogError {
    /// Failed to create logs directory.
    CreateDir(String),

    /// Failed to create log file.
    CreateFile(String),

    /// Failed to write to log file.
    WriteFile(String),
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Error] Logging error: {}",
            match self {
                LogError::CreateDir(message) => format!("Failed to create logs directory: {}", message),
                LogError::CreateFile(message) => format!("Failed to create log file: {}", message),
                LogError::WriteFile(message) => format!("Failed to write to log file: {}", message),
            }
        )
    }
}

impl Error for LogError {}
