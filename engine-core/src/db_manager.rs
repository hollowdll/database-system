// Restructured Database manager

use std::{
    io,
    collections::HashMap,
};

use crate::logs;
use crate::db;
use crate::input_data;

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

    /// Creates a new collection to a database
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

        let log_content = format!("Created collection '{}' to database '{}'", collection_name, database_name);
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Collection,
            logs::DatabaseEventType::Created,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }

        Ok(true)
    }

    /// Deletes a collection from a database
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

    /// Creates a new document to a collection
    pub fn create_document(
        &self,
        database_name: &str,
        collection_name: &str,
        data: Vec<db::InputDataField>,
    ) -> Result<(bool, String), io::Error>
    {
        let mut document_data: HashMap<String, db::DataType> = HashMap::new();

        // convert input data to correct data type
        for data_field in data {
            let converted_value = match input_data::convert_input_data(data_field.value(), data_field.data_type()) {
                Some(converted_value) => converted_value,
                None => return Ok((false, String::from("Failed to create document. Data type is not valid"))),
            };

            document_data.insert(data_field.field().to_string(), converted_value);
        }

        match db::create_document_to_collection(database_name, collection_name, document_data) {
            Ok(result) => {
                if !result {
                    return Ok((false, String::from("Failed to create document. Database or collection might not exist")));
                }
            },
            Err(e) => return Err(e),
        }

        let log_content = format!("Created document to collection '{}' in database '{}'", collection_name, database_name);
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Document,
            logs::DatabaseEventType::Created,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }

        Ok((true, String::from("Created document")))
    }

    /// Deletes a document from a collection
    pub fn delete_document_from_collection(
        &self,
        database_name: &str,
        collection_name: &str,
        document_id: &u64,
    ) -> io::Result<(bool, String)>
    {
        match db::delete_document_from_collection(database_name, collection_name, document_id) {
            Ok(result) => {
                if !result {
                    return Ok((false, String::from("Failed to delete document. It might not exist.")));
                }
            },
            Err(e) => return Err(e),
        }

        let log_content = format!(
            "Deleted document with ID '{}' from collection '{}' in database '{}'",
            document_id, collection_name, database_name
        );
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Document,
            logs::DatabaseEventType::Deleted,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }

        Ok((true, String::from("Deleted document")))
    }

    /// Deletes a document from database
    pub fn delete_document(
        &self,
        database_name: &str,
        document_id: &u64,
    ) -> io::Result<(bool, String)>
    {
        match db::delete_document(database_name, document_id) {
            Ok(result) => {
                if !result {
                    return Ok((false, String::from("Failed to delete document. It might not exist.")));
                }
            },
            Err(e) => return Err(e),
        }

        let log_content = format!(
            "Deleted document with ID '{}' from database '{}'",
            document_id, database_name
        );
        if let Err(e) = logs::log_database_event(
            logs::DatabaseEventSource::Document,
            logs::DatabaseEventType::Deleted,
            log_content.as_str()
        ) {
            eprintln!("Error occurred while trying to log database event: {e}");
        }

        Ok((true, String::from("Deleted document")))
    }

    /// Finds all documents of collection
    pub fn find_all_documents_of_collection(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> io::Result<Vec<db::FormattedDocument>>
    {
        let documents = match db::find_all_documents_of_collection(database_name, collection_name) {
            Ok(documents) => documents,
            Err(e) => return Err(e),
        };

        Ok(documents)
    }
}

impl DatabaseManager {
    /// Build a new database manager.
    pub fn build() -> Self {
        Self {}
    }
}