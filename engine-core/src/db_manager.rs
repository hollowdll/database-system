// Restructured Database manager

use std::{
    io,
};
use crate::logs;
use crate::db;

struct DatabaseManager {}

impl DatabaseManager {
    pub fn create_database(&self, database_name: &str) -> Result<(), io::Error> {
        if let Err(e) = db::create_databases_dir() {
            return Err(e);
        }

        db::create_database_file(database_name);

        Ok(())
    }

    pub fn delete_database(&self) {

    }

    pub fn connect_database(&self) {

    }

    pub fn disconnect_database(&self) {

    }

    pub fn fetch_databases() {
        
    }
}

impl DatabaseManager {
    /// Build a new database manager.
    fn build() -> Self {
        Self {}
    }
}