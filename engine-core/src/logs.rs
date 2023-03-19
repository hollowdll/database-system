// Library for database logs
// All database events are logged to .log files

// TODO
// Create structs and methods later

//#![allow(unused)]

use std::{
    fs,
    io::{self, Write},
    path::Path,
};

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

fn create_log_file(name: &str, content: &str) -> io::Result<()> {
    if Path::new("./logs").is_dir() {
        let mut file = fs::File::create(format!("./logs/{name}"))?;

        file.write(content.as_bytes())?;
    } else {
        println!("logs dir not found");
    }

    Ok(())
}

pub fn create_test_log() -> Result<(), io::Error> {
    let log_name = "testlog.log";
    let log_content = "[Date and time] [TEST] test log 123";

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