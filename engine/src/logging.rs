// Logging module that handles logs.
// All events and errors are logged to .log files.

//#![allow(unused)]

pub mod error;

use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};
use chrono::{
    DateTime,
    Local,
};
use self::error::{
    LogError,
    LogErrorKind,
};

/// Events log file name.
pub const EVENTS_LOG: &str = "events.log";

/// Errors log file name.
pub const ERRORS_LOG: &str = "errors.log";

/// Event log that can be written to log files.
struct EventLog {
    created: DateTime<Local>,
    content: String,
}

impl EventLog {
    fn format(&self) -> String {
        format!(
            "[{}] {}\n",
            self.created.format("%F %X%.3f %:z"),
            self.content
        )
    }

    fn new(content: &str) -> Self {
        Self {
            created: Local::now(),
            content: String::from(content),
        }
    }
}

/// Error log that can be written to log files.
struct ErrorLog {
    created: DateTime<Local>,
    error_type: ErrorLogType,
    content: String,
}

impl ErrorLog {
    fn format(&self) -> String {
        format!(
            "[{}] [{:?}] {}\n",
            self.created.format("%F %X%.3f %:z"),
            self.error_type,
            self.content
        )
    }

    fn new(error_type: ErrorLogType, content: &str) -> Self {
        Self {
            created: Local::now(),
            error_type,
            content: String::from(content),
        }
    }
}

/// Type of error log.
/// Can be error or warning.
#[derive(Debug)]
pub enum ErrorLogType {
    Error,
    Warning,
}

/// Logger that logs events and errors to log files.
/// 
/// Logger is enabled by default. To disable it, you need to set it explicitly.
#[derive(Debug)]
pub struct Logger {
    logs_dir_path: PathBuf,
    /// True if this logger is disabled and should not log anything.
    pub disabled: bool,
}

impl Logger {
    /// Builds a new logger.
    pub fn build(logs_dir_path: &Path) -> Self {
        Self {
            logs_dir_path: PathBuf::from(logs_dir_path),
            disabled: false,
        }
    }

    /// Builds a new logger that is disabled.
    pub fn build_disabled(logs_dir_path: &Path) -> Self {
        Self {
            logs_dir_path: PathBuf::from(logs_dir_path),
            disabled: true,
        }
    }
}

impl Logger {
    /// Gets logs directory path.
    pub fn logs_dir_path(&self) -> &Path {
        &self.logs_dir_path
    }

    /// Gets events log file path.
    pub fn get_events_log_path(&self) -> PathBuf {
        self.logs_dir_path().join(EVENTS_LOG)
    }

    /// Gets errors log file path.
    pub fn get_errors_log_path(&self) -> PathBuf {
        self.logs_dir_path().join(ERRORS_LOG)
    }
}

impl Logger {
    /// Logs an event to event log file.
    /// 
    /// Writes the data to the file by appending.
    pub fn log_event(
        &self,
        content: &str,
    ) -> Result<(), LogError>
    {
        if self.disabled {
            return Ok(());
        }

        let log = EventLog::new(content).format();

        if let Err(e) = create_logs_dir_if_not_exists(&self.logs_dir_path()) {
            return Err(LogError::new(LogErrorKind::CreateDir, e.to_string()));
        }
        if let Err(e) = create_log_file_if_not_exists(
            &self.get_events_log_path()
        ) {
            return Err(LogError::new(LogErrorKind::CreateFile, e.to_string()));
        }
        if let Err(e) = write_log_file(
            &self.get_events_log_path(),
            &log
        ) {
            return Err(LogError::new(LogErrorKind::WriteFile, e.to_string()));
        }

        Ok(())
    }

    /// Logs an error to error log file.
    /// 
    /// Writes the data to the file by appending.
    pub fn log_error(
        &self,
        error_type: ErrorLogType,
        content: &str,
    ) -> Result<(), LogError>
    {
        if self.disabled {
            return Ok(());
        }

        let log = ErrorLog::new(error_type, content).format();

        if let Err(e) = create_logs_dir_if_not_exists(&self.logs_dir_path()) {
            return Err(LogError::new(LogErrorKind::CreateDir, e.to_string()));
        }
        if let Err(e) = create_log_file_if_not_exists(
            &self.get_errors_log_path()
        ) {
            return Err(LogError::new(LogErrorKind::CreateFile, e.to_string()));
        }
        if let Err(e) = write_log_file(
            &self.get_errors_log_path(),
            &log
        ) {
            return Err(LogError::new(LogErrorKind::WriteFile, e.to_string()));
        }

        Ok(())
    }
}

/// Creates logs directory if it doesn't exist.
fn create_logs_dir_if_not_exists(path: &Path) -> io::Result<()> {
    if !path.is_dir() {
        fs::create_dir(path)?;
    }

    Ok(())
}

/// Creates a log file to logs directory if it doesn't exist.
fn create_log_file_if_not_exists(file_path: &Path) -> io::Result<()> {
    if !file_path.is_file() {
        File::create(file_path)?;
    }

    Ok(())
}

/// Writes data to a log file by appending.
fn write_log_file(file_path: &Path, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
