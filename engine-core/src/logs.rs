// Database logs
// All database events are logged to .log files

//#![allow(unused)]

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    time::SystemTime,
};

pub const DB_EVENTS_LOG: &str = "db_events.log";
pub const TEMP_DB_EVENTS_LOG: &str = "temp_db_events.log";
const TEST_LOG: &str = "test_log.log";
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
    created: SystemTime,
    event_source: DatabaseEventSource,
    event: DatabaseEvent,
    content: String,
}

impl DatabaseEventLog {
    fn format(&self) -> String {
        let time = match self.created.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(time) => time,
            Err(e) => panic!("SystemTime error: {e}"),
        };

        format!(
            "[System time in seconds: {:?}] [{:?}] [{:?}] - {}\n",
            time.as_secs_f64(),
            self.event_source,
            self.event,
            self.content
        )
    }
}

impl DatabaseEventLog {
    fn from(event_source: DatabaseEventSource, event: DatabaseEvent, content: &str) -> Self {
        Self {
            created: SystemTime::now(),
            event_source,
            event,
            content: String::from(content),
        }
    }

    fn create_test_log() -> Self {
        Self {
            created: SystemTime::now(),
            event_source: DatabaseEventSource::System,
            event: DatabaseEvent::Test,
            content: String::from("Test log 123"),
        }
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
fn create_log_file_if_not_exists(file_name: &str) -> io::Result<()> {
    if !Path::new(&format!("{LOGS_DIR_PATH}/{file_name}")).is_file() {
        fs::File::create(format!("{LOGS_DIR_PATH}/{file_name}"))?;
    }

    Ok(())
}

/// Writes data to a log file without overwriting it.
fn write_log_file(file_name: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(&format!("{LOGS_DIR_PATH}/{file_name}"))?;

    file.write(content.as_bytes())?;

    Ok(())
}

/// Creates test log to test log file.
pub fn create_test_log() -> io::Result<()> {
    let file_name = TEST_LOG;
    let log = DatabaseEventLog::create_test_log().format();

    create_logs_dir_if_not_exists()?;
    create_log_file_if_not_exists(file_name)?;
    write_log_file(file_name, &log)?;

    Ok(())
}

/// Logs database event to log file.
pub fn log_database_event(
    event_source: DatabaseEventSource,
    event: DatabaseEvent,
    content: &str,
    file_name: &str,
) -> io::Result<()>
{
    let log = DatabaseEventLog::from(event_source, event, content).format();

    create_logs_dir_if_not_exists()?;
    create_log_file_if_not_exists(file_name)?;
    write_log_file(file_name, &log)?;

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_logs_dir_if_not_exists() {
        create_logs_dir_if_not_exists().unwrap();

        assert!(Path::new(&format!("{LOGS_DIR_PATH}")).is_dir());
    }

    #[test]
    fn test_create_log_file_if_not_exists() {
        create_log_file_if_not_exists(TEMP_DB_EVENTS_LOG).unwrap();

        assert!(Path::new(&format!("{LOGS_DIR_PATH}/{TEMP_DB_EVENTS_LOG}")).is_file());
    }

    #[test]
    fn test_log_database_event() {
        let file_name = "test_log_database_event";
        let log = DatabaseEventLog::from(
            DatabaseEventSource::System,
            DatabaseEvent::Test,
            "Test log",
        ).format();
        
        create_logs_dir_if_not_exists().unwrap();
        create_log_file_if_not_exists(file_name).unwrap();
        write_log_file(file_name, &log).unwrap();

        let contents = fs::read_to_string(&format!("{LOGS_DIR_PATH}/{file_name}")).unwrap();
        let mut log = log.chars();
        log.next_back();
        let log = log.as_str();

        assert_eq!(contents.lines().last(), Some(log));

        fs::remove_file(&format!("{LOGS_DIR_PATH}/{file_name}")).unwrap();
    }
}