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
use self::error::LogError;
use crate::Config;

pub const DB_EVENTS_LOG: &str = "events.log";
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
#[derive(Debug)]
pub struct Logger<'a> {
    config: &'a Config,
}

impl<'a> Logger<'a> {
    /// Builds a new logger.
    pub fn build(config: &'a Config) -> Self {
        Self {
            config
        }
    }
}

impl<'a> Logger<'a> {
    /// Gets logs directory path.
    pub fn logs_dir_path(&self) -> &Path {
        &self.config.logs_dir_path
    }

    /// Gets database events log file path.
    pub fn get_db_events_log_path(&self) -> PathBuf {
        self.logs_dir_path().join(DB_EVENTS_LOG)
    }

    /// Gets errors log file path.
    pub fn get_errors_log_path(&self) -> PathBuf {
        self.logs_dir_path().join(ERRORS_LOG)
    }
}

impl<'a> Logger<'a> {
    /// Logs an event to event log file.
    pub fn log_event(
        &self,
        content: &str,
    ) -> Result<(), LogError>
    {
        let log = EventLog::new(content).format();

        if let Err(e) = create_logs_dir_if_not_exists(&self.logs_dir_path()) {
            return Err(LogError::CreateDir(e.to_string()));
        }
        if let Err(e) = create_log_file_if_not_exists(
            &self.get_db_events_log_path()
        ) {
            return Err(LogError::CreateFile(e.to_string()));
        }
        if let Err(e) = write_log_file(
            &self.get_db_events_log_path(),
            &log
        ) {
            return Err(LogError::WriteFile(e.to_string()));
        }

        Ok(())
    }

    /// Logs an error to error log file.
    pub fn log_error(
        &self,
        error_type: ErrorLogType,
        content: &str,
    ) -> Result<(), LogError>
    {
        let log = ErrorLog::new(error_type, content).format();

        if let Err(e) = create_logs_dir_if_not_exists(&self.logs_dir_path()) {
            return Err(LogError::CreateDir(e.to_string()));
        }
        if let Err(e) = create_log_file_if_not_exists(
            &self.get_errors_log_path()
        ) {
            return Err(LogError::CreateFile(e.to_string()));
        }
        if let Err(e) = write_log_file(
            &self.get_errors_log_path(),
            &log
        ) {
            return Err(LogError::WriteFile(e.to_string()));
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
