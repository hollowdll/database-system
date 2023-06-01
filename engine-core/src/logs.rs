// Database logs
// All database events are logged to .log files

// THIS MODULE KEEPS CHANGING FREQUENTLY
// Code in this module will likely change a lot

//#![allow(unused)]

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};
use chrono::{
    DateTime,
    Local,
};

pub const DB_EVENTS_LOG: &str = "db_events.log";
const LOGS_DIR_PATH: &str = "./logs";

#[derive(Debug)]
pub enum DatabaseEventSource {
    System,
    DatabaseManager,
    Database,
    Collection,
    Document,
}

#[derive(Debug)]
pub enum DatabaseEvent {
    Test,
    Connected,
    Disconnected,
    Created,
    Deleted,
    Updated,
}

/// Database event log to write to log file
struct DatabaseEventLog {
    created: DateTime<Local>,
    event_source: DatabaseEventSource,
    event: DatabaseEvent,
    content: String,
}

impl DatabaseEventLog {
    fn format(&self) -> String {
        format!(
            "[{:?}] [{:?}] [{:?}] - {}\n",
            self.created,
            self.event_source,
            self.event,
            self.content
        )
    }
}

impl DatabaseEventLog {
    fn from(
        event_source: DatabaseEventSource,
        event: DatabaseEvent,
        content: &str,
    ) -> Self
    {
        Self {
            created: Local::now(),
            event_source,
            event,
            content: String::from(content),
        }
    }
}

/// Logger that logs all events to log files
pub struct Logger {}

impl Logger {
    /// Logs database event to log file.
    pub fn log_database_event(
        event_source: DatabaseEventSource,
        event: DatabaseEvent,
        content: &str,
        file_path: &Path,
    ) -> io::Result<()>
    {
        let log = DatabaseEventLog::from(event_source, event, content).format();

        create_logs_dir_if_not_exists()?;
        create_log_file_if_not_exists(file_path)?;
        write_log_file(file_path, &log)?;

        Ok(())
    }

    /// Gets database events log file path
    pub fn get_db_events_log_path() -> PathBuf {
        PathBuf::from(&format!("{}/{}", LOGS_DIR_PATH, DB_EVENTS_LOG))
    }
}

/// Creates logs directory if it doesn't exist.
fn create_logs_dir_if_not_exists() -> io::Result<()> {
    if !Path::new(&format!("{LOGS_DIR_PATH}")).is_dir() {
        fs::create_dir(&format!("{LOGS_DIR_PATH}"))?;
    }

    Ok(())
}

/// Creates a log file to logs directory if it doesn't exist.
fn create_log_file_if_not_exists(file_path: &Path) -> io::Result<()> {
    if !file_path.is_file() {
        fs::File::create(file_path)?;
    }

    Ok(())
}

/// Writes data to a log file by appending.
fn write_log_file(file_path: &Path, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)?;

    file.write(content.as_bytes())?;

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::{
        fs::File,
        path::Path,
    };

    #[test]
    fn test_create_logs_dir_if_not_exists() {
        assert!(create_logs_dir_if_not_exists().is_ok());
        assert!(Path::new(&format!("{LOGS_DIR_PATH}")).is_dir());
    }

    #[test]
    fn test_create_log_file_if_not_exists() {
        assert!(create_log_file_if_not_exists(
            Path::new(&format!("{}/{}", LOGS_DIR_PATH, DB_EVENTS_LOG))
        ).is_ok());
        assert!(Path::new(&format!("{}/{}", LOGS_DIR_PATH, DB_EVENTS_LOG)).is_file());
    }

    #[test]
    fn test_log_database_event() {
        let log = DatabaseEventLog::from(
            DatabaseEventSource::System,
            DatabaseEvent::Test,
            "Test log 123",
        ).format();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.log");
        let file = File::create(&file_path).unwrap();
        
        assert!(write_log_file(&file_path, &log).is_ok());

        let buf = fs::read_to_string(&file_path).unwrap();
        assert_eq!(buf, log);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }
}