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
        println!("Created logs dir");
    } else {
        println!("logs dir already exists");
    }

    if let Err(e) = create_log_file() {
        eprintln!("Error: {e}");
    }

    Ok(())
}

fn create_log_file() -> io::Result<()> {
    // Create .log file to logs directory with the given name
    // if it doesn't exist

    if Path::new("./logs").is_dir() {
        let mut file = fs::File::create("./logs/testlog.log")?;
        println!("Created testlog.log");

        file.write(b"Test log 123")?;
        println!("Write data to testlog.log");
    } else {
        println!("logs dir not found");
    }

    Ok(())
}

pub fn create_test_log() {
    // Check if logs directory exists
    // Check if .log file exists
    // Create items above if false
    // Write to the file

    if let Err(e) = create_logs_dir() {
        eprintln!("Error: {e}");
    }
}

pub fn log_database_event() {

}