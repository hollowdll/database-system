// Library for database logs
// All database events are logged to .log files

// TODO
// Create structs and methods later

//#![allow(unused)]

use std::{
    fs,
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
    log_text: String,
}

impl DatabaseEventLog {
    fn format(&self) -> String {
        let time = match self.created.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(time) => time.as_nanos(),
            Err(e) => panic!("SystemTime error: {e}"),
        };

        format!("[{:?}] [{:?}] [{:?}] {}", self.location, time, self.event_type, self.log_text)
    }
}

impl DatabaseEventLog {
    fn from(location: DatabaseEventLocation, event_type: DatabaseEventType, log_text: &str) -> Self {
        Self {
            location,
            created: SystemTime::now(),
            event_type,
            log_text: String::from(log_text),
        }
    }

    fn create_test_log() -> Self {
        Self {
            location: DatabaseEventLocation::System,
            created: SystemTime::now(),
            event_type: DatabaseEventType::Test,
            log_text: String::from("Test log 123"),
        }
    }
}

fn create_logs_dir() -> io::Result<()> {
    // Create logs directory to the project root
    // if it doesn't exist

    if !Path::new("./logs").is_dir() {
        fs::create_dir("./logs")?;
    } else {
        println!("logs dir already exists");
    }

    Ok(())
}

fn create_log_file(name: &str, content: String) -> io::Result<()> {
    if Path::new("./logs").is_dir() {
        let mut file = fs::File::create(format!("./logs/{name}"))?;

        file.write(content.as_bytes())?;
    } else {
        println!("logs dir not found");
    }

    Ok(())
}

fn write_to_log_file() {
    
}

pub fn create_test_log() -> Result<(), io::Error> {
    let log_name = "testlog.log";
    let log = DatabaseEventLog::create_test_log();
    let log_content = log.format();
    

    if let Err(e) = create_logs_dir() {
        eprintln!("Error: {e}");

        return Err(e);
    }

    if let Err(e) = create_log_file(log_name, log_content) {
        eprintln!("Error: {e}");

        return Err(e);
    }

    Ok(())
}

pub fn log_database_event() {

}