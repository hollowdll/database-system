// Restructured Database manager

use std::{
    io,
};
use crate::logs;
use crate::db;

pub struct DatabaseManager {}

impl DatabaseManager {
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
            eprintln!("Error: {e}");
        }

        Ok(true)
    }

    pub fn delete_database(&self) {

    }

    pub fn connect_database(&self) {

    }

    pub fn disconnect_database(&self) {

    }

    pub fn find_all_databases(&self) -> Result<(), io::Error> {
        if let Err(e) = db::find_all_database_files() {
            return Err(e);
        }

        Ok(())
    }
}

impl DatabaseManager {
    /// Build a new database manager.
    pub fn build() -> Self {
        Self {}
    }
}