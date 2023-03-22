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
enum DatabaseEventType {
    Test,
    Connected,
}

#[derive(Debug)]
enum DatabaseEventLocation {
    System,
    DatabaseManager,
    Database,
    DatabaseTable,
}

struct DatabaseEventLog {
    location: DatabaseEventLocation,
    created: SystemTime,
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
            "[{:?}] [System time in nanoseconds: {:?}] [{:?}] {}\n",
            self.location,
            time.as_nanos(),
            self.event_type,
            self.content
        )
    }
}

impl DatabaseEventLog {
    fn from(location: DatabaseEventLocation, event_type: DatabaseEventType, content: &str) -> Self {
        Self {
            location,
            created: SystemTime::now(),
            event_type,
            content: String::from(content),
        }
    }

    fn create_test_log() -> Self {
        Self {
            location: DatabaseEventLocation::System,
            created: SystemTime::now(),
            event_type: DatabaseEventType::Test,
            content: String::from("Test log 123"),
        }
    }
}



fn create_logs_dir() -> io::Result<()> {
    // Create logs directory to the project root
    // if it doesn't exist

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

fn write_test_log(content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open("./logs/testlog.log")?;

    file.write(content.as_bytes())?;

    Ok(())
}

fn write_database_events_log(content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open("./logs/database-events.log")?;

    file.write(content.as_bytes())?;

    Ok(())
}

pub fn create_test_log() -> Result<(), io::Error> {
    let log_name = "testlog.log";
    let log = DatabaseEventLog::create_test_log().format();
    

    if let Err(e) = create_logs_dir() {
        eprintln!("Error: {e}");

        return Err(e);
    }

    if let Err(e) = create_log_file(log_name) {
        eprintln!("Error: {e}");

        return Err(e);
    }

    if let Err(e) = write_test_log(log.as_str()) {
        eprintln!("Error: {e}");

        return Err(e);
    }

    Ok(())
}

pub fn log_database_event() {

}