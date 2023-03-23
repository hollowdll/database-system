// Library for database logs
// All database events are logged to .log files

// TODO
// Create structs and methods later

//#![allow(unused)]

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    time::{
        SystemTime,
    },
};

#[derive(Debug)]
pub enum DatabaseEventSource {
    System,
    DatabaseManager,
    Database,
}

#[derive(Debug)]
pub enum DatabaseEventType {
    Test,
    Connected,
    Disconnected,
    Created,
    Deleted,
    Updated,
}

struct DatabaseEventLog {
    created: SystemTime,
    event_source: DatabaseEventSource,
    event_type: DatabaseEventType,
    content: String,
}

impl DatabaseEventLog {
    fn format(&self) -> String {
        let time = match self.created.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(time) => time,
            Err(e) => panic!("SystemTime error: {e}"),
        };

        format!(
            "[System time in nanoseconds: {:?}] [{:?}] [{:?}] - {}\n",
            time.as_nanos(),
            self.event_source,
            self.event_type,
            self.content
        )
    }
}

impl DatabaseEventLog {
    fn from(event_source: DatabaseEventSource, event_type: DatabaseEventType, content: &str) -> Self {
        Self {
            created: SystemTime::now(),
            event_source,
            event_type,
            content: String::from(content),
        }
    }

    fn create_test_log() -> Self {
        Self {
            created: SystemTime::now(),
            event_source: DatabaseEventSource::System,
            event_type: DatabaseEventType::Test,
            content: String::from("Test log 123"),
        }
    }
}



fn create_logs_dir() -> io::Result<()> {
    if !Path::new("./logs").is_dir() {
        fs::create_dir("./logs")?;
    }

    Ok(())
}

fn create_log_file(name: &str) -> io::Result<()> {
    if !Path::new(format!("./logs/{name}").as_str()).is_file() {
        let file = fs::File::create(format!("./logs/{name}"))?;
    }

    Ok(())
}

fn write_log_file(name: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(format!("./logs/{name}"))?;

    file.write(content.as_bytes())?;

    Ok(())
}

pub fn create_test_log() -> Result<(), io::Error> {
    let log_name = "testlog.log";
    let log = DatabaseEventLog::create_test_log().format();

    if let Err(e) = create_logs_dir() {
        return Err(e);
    }

    if let Err(e) = create_log_file(log_name) {
        return Err(e);
    }

    if let Err(e) = write_log_file(log_name, log.as_str()) {
        return Err(e);
    }

    Ok(())
}

pub fn log_database_event(
    event_source: DatabaseEventSource,
    event_type: DatabaseEventType,
    content: &str
) -> Result<(), io::Error>
{
    let log_name = "database-events.log";
    let log = DatabaseEventLog::from(event_source, event_type, content).format();

    if let Err(e) = create_logs_dir() {
        return Err(e);
    }

    if let Err(e) = create_log_file(log_name) {
        return Err(e);
    }

    if let Err(e) = write_log_file(log_name, log.as_str()) {
        return Err(e);
    }

    Ok(())
}