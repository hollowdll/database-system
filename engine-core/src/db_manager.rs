// Restructured Database manager

use std::{
    io,
};
use crate::logs;
use crate::db;

/// Database manager that manages all databases
/// and database related operations
#[derive(PartialEq, Debug)]
pub struct DatabaseManager {}

impl DatabaseManager {
    /// Creates a new database 
    pub fn create_database(&self, database_name: &str) -> Result<bool, io::Error> {
        if let Err(e) = db::create_databases_dir() {
            return Err(e);
        }
            
        match db::create_database_file(database_name) {
            Ok(result) => {
                if !result {
                    return Ok(false);
                }
            },
            Err(e) => return Err(e),
        }

        let log_content = format!("Created database: {}", database_name);
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Database,
            logs::DatabaseEventType::Created,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }

        Ok(true)
    }

    /// Deletes a database
    pub fn delete_database(&self, database_name: &str) -> Result<bool, io::Error> {
        match db::delete_database_file(database_name) {
            Ok(result) => {
                if !result {
                    return Ok(false);
                }
            },
            Err(e) => return Err(e),
        }

        let log_content = format!("Deleted database: {}", database_name);
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Database,
            logs::DatabaseEventType::Deleted,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }
        
        Ok(true)
    }

    pub fn connect_database(&self) {

    }

    pub fn disconnect_database(&self) {

    }

    /// Finds all databases
    pub fn find_all_databases(&self) -> Result<Vec<db::FormattedDatabase>, io::Error> {
        let databases = match db::find_all_databases() {
            Ok(databases) => databases,
            Err(e) => return Err(e),
        };

        Ok(databases)
    }

    // Tries to find a database by name
    pub fn find_database(&self, database_name: &str) -> Result<bool, io::Error> {
        match db::find_database(database_name) {
            Ok(result) => {
                if !result {
                    return Ok(false);
                }
            },
            Err(e) => return Err(e),
        }

        Ok(true)
    }
}

impl DatabaseManager {
    /// Build a new database manager.
    pub fn build() -> Self {
        Self {}
    }
}