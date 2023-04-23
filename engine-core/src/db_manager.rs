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

        let log_content = format!("Created database '{}'", database_name);
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

        let log_content = format!("Deleted database '{}'", database_name);
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Database,
            logs::DatabaseEventType::Deleted,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }
        
        Ok(true)
    }

    /// Changes description of a database
    pub fn change_database_description(&self, database_name: &str, description: &str) -> Result<bool, io::Error> {
        match db::change_database_description(database_name, description) {
            Ok(result) => {
                if !result {
                    return Ok(false)
                }
            },
            Err(e) => return Err(e),
        }

        let log_content = format!("Changed description of database '{}'", database_name);
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Database,
            logs::DatabaseEventType::Updated,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }

        Ok(true)
    }

    /// Creates a new collection in a database
    pub fn create_collection(&self, collection_name: &str, database_name: &str) -> Result<bool, io::Error> {
        // Cancel if collection with this name already exists
        match db::find_collection(collection_name, database_name) {
            Ok(result) => {
                if result {
                    return Ok(false);
                }
            },
            Err(e) => return Err(e),
        }

        match db::create_collection_to_database_file(collection_name, database_name) {
            Ok(result) => {
                if !result {
                    return Ok(false);
                }
            },
            Err(e) => return Err(e),
        }

        let log_content = format!("Created collection '{}' in database '{}'", collection_name, database_name);
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Collection,
            logs::DatabaseEventType::Created,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }

        Ok(true)
    }

    pub fn delete_collection(&self, collection_name: &str, database_name: &str) -> Result<bool, io::Error> {
        match db::delete_collection_from_database_file(collection_name, database_name) {
            Ok(result) => {
                if !result {
                    return Ok(false);
                }
            },
            Err(e) => return Err(e),
        }

        let log_content = format!("Deleted collection '{}' in database '{}'", collection_name, database_name);
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Collection,
            logs::DatabaseEventType::Deleted,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }

        Ok(true)
    }

    /// Finds all databases
    pub fn find_all_databases(&self) -> Result<Vec<db::FormattedDatabase>, io::Error> {
        let databases = match db::find_all_databases() {
            Ok(databases) => databases,
            Err(e) => return Err(e),
        };

        Ok(databases)
    }

    /// Check if a database exists
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

    /// Finds all collections of a database
    pub fn find_all_collections_of_database(
        &self, database_name: &str
    ) -> Result<Vec<db::FormattedDocumentCollection>, io::Error>
    {
        let collections = match db::find_all_collections_of_database(database_name) {
            Ok(collections) => collections,
            Err(e) => return Err(e),
        };

        Ok(collections)
    }

    /// Check if a collection exists
    pub fn find_collection(&self, collection_name: &str, database_name: &str) -> Result<bool, io::Error> {
        match db::find_collection(collection_name, database_name) {
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