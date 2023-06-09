// This file contains database manager related code

use std::{
    io,
    collections::HashMap,
    path::Path,
    error::Error,
};
use crate::{
    logging::*,
    constants::{
        DB_EVENT_LOG_ERROR,
        DB_DIR_PATH,
    },
    InputDataField,
};
use crate::db::{
    self,
    DataType,
    FormattedDatabase,
    FormattedDocumentCollection,
    FormattedDocument,
    database_file_path,
};

/// Database manager that manages all databases
/// and database related operations
#[derive(PartialEq, Debug)]
pub struct DatabaseManager {}

impl DatabaseManager {
    /// Creates a new database 
    pub fn create_database(
        &self,
        database_name: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::create_databases_dir_if_not_exists()?;

        db::create_database_file(
            database_name,
            &database_file_path(database_name)
        )?;

        if let Err(e) = Logger::log_event(
            DatabaseEventSource::Database,
            DatabaseEvent::Created,
            &format!("Created database '{}'", database_name),
            &get_db_events_log_path() 
        ) {
            eprintln!("{}: {e}", DB_EVENT_LOG_ERROR);
        }

        Ok("Created database".to_string())
    }

    /// Deletes a database
    pub fn delete_database(
        &self,
        database_name: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::delete_database_file(
            database_name,
            &database_file_path(database_name)
        )?;

        if let Err(e) = Logger::log_event(
            DatabaseEventSource::Database,
            DatabaseEvent::Deleted,
            &format!("Deleted database '{}'", database_name),
            &get_db_events_log_path() 
        ) {
            eprintln!("{}: {e}", DB_EVENT_LOG_ERROR);
        }
        
        Ok("Deleted database".to_string())
    }

    /// Changes description of a database
    pub fn change_database_description(
        &self,
        database_name: &str,
        description: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::change_database_description(
            description,
            &database_file_path(database_name)
        )?;

        if let Err(e) = Logger::log_event(
            DatabaseEventSource::Database,
            DatabaseEvent::Updated,
            &format!("Changed description of database '{}'", database_name),
            &get_db_events_log_path() 
        ) {
            eprintln!("{}: {e}", DB_EVENT_LOG_ERROR);
        }

        Ok("Changed database description".to_string())
    }

    /// Creates a new collection to a database
    pub fn create_collection(
        &self,
        collection_name: &str,
        database_name: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::create_collection_to_database_file(
            collection_name,
            &database_file_path(database_name)
        )?;

        if let Err(e) = Logger::log_event(
            DatabaseEventSource::Collection,
            DatabaseEvent::Created,
            &format!(
                "Created collection '{}' to database '{}'",
                collection_name,
                database_name
            ),
            &get_db_events_log_path(),
        ) {
            eprintln!("{}: {e}", DB_EVENT_LOG_ERROR);
        }

        Ok("Created collection".to_string())
    }

    /// Deletes a collection from a database
    pub fn delete_collection(
        &self,
        collection_name: &str,
        database_name: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::delete_collection_from_database_file(
            collection_name,
            &database_file_path(database_name)
        )?;

        if let Err(e) = Logger::log_event(
            DatabaseEventSource::Collection,
            DatabaseEvent::Deleted,
            &format!(
                "Deleted collection '{}' from database '{}'",
                collection_name,
                database_name
            ),
            &get_db_events_log_path() 
        ) {
            eprintln!("{}: {e}", DB_EVENT_LOG_ERROR);
        }

        Ok("Deleted collection".to_string())
    }

    /// Finds all databases
    pub fn find_all_databases(&self) -> io::Result<Vec<FormattedDatabase>> {
        db::create_databases_dir_if_not_exists()?;

        match db::find_all_databases(Path::new(DB_DIR_PATH)) {
            Ok(databases) => return Ok(databases),
            Err(e) => return Err(e),
        };
    }

    /// Finds a database
    pub fn find_database(&self, database_name: &str) -> io::Result<Option<FormattedDatabase>> {
        db::create_databases_dir_if_not_exists()?;

        match db::find_database(database_name, Path::new(DB_DIR_PATH)) {
            Ok(result) => return Ok(result),
            Err(e) => return Err(e),
        }
    }

    /// Finds all collections of a database
    pub fn find_all_collections_of_database(
        &self,
        database_name: &str,
    ) -> io::Result<Vec<FormattedDocumentCollection>>
    {
        match db::find_all_collections_of_database(&database_file_path(database_name)) {
            Ok(collections) => return Ok(collections),
            Err(e) => return Err(e),
        };
    }

    /// Finds a collection in a database
    pub fn find_collection(
        &self,
        collection_name: &str,
        database_name: &str,
    ) -> Result<Option<FormattedDocumentCollection>, Box<dyn Error>>
    {
        match db::find_collection(collection_name, &database_file_path(database_name)) {
            Ok(result) => return Ok(result),
            Err(e) => return Err(e),
        }
    }

    /// Creates a new document to a collection
    pub fn create_document(
        &self,
        database_name: &str,
        collection_name: &str,
        data: Vec<InputDataField>,
    ) -> io::Result<(bool, String)>
    {
        let mut document_data: HashMap<String, DataType> = HashMap::new();

        // convert input data to correct document data types
        for data_field in data {
            let converted_value = match data_field.convert_to_document_data_type(
                data_field.value(),
                data_field.data_type()
            ) {
                Some(converted_value) => converted_value,
                None => return Ok((false, String::from("Failed to create document. Data type is not valid"))),
            };

            document_data.insert(data_field.field().to_string(), converted_value);
        }

        match db::create_document_to_collection(
            &database_file_path(database_name),
            collection_name,
            document_data
        ) {
            Ok((result, message)) => {
                if !result {
                    return Ok((false, format!("Failed to create document: {message}")));
                }
            },
            Err(e) => return Err(e),
        }

        if let Err(e) = Logger::log_event(
            DatabaseEventSource::Document,
            DatabaseEvent::Created,
            &format!("Created document to collection '{}' in database '{}'", collection_name, database_name),
            &get_db_events_log_path() 
        ) {
            eprintln!("{}: {e}", DB_EVENT_LOG_ERROR);
        }

        Ok((true, "Created document".to_string()))
    }

    /// Deletes a document from a collection
    /// 
    /// This is a faster way to delete a document if the collection is known beforehand.
    
    /* DISABLED. NOT NEEDED RIGHT NOW.
    pub fn delete_document_from_collection(
        &self,
        database_name: &str,
        collection_name: &str,
        document_id: &u64,
    ) -> io::Result<(bool, String)>
    {
        match db::delete_document_from_collection(
            &database_file_path(database_name),
            collection_name,
            document_id
        ) {
            Ok((result, message)) => {
                if !result {
                    return Ok((false, format!("Failed to delete document: {message}")));
                }
            },
            Err(e) => return Err(e),
        }

        if let Err(e) = Logger::log_event(
            DatabaseEventSource::Document,
            DatabaseEvent::Deleted,
            &format!(
                "Deleted document with ID '{}' from collection '{}' in database '{}'",
                document_id, collection_name, database_name
            ),
            &get_db_events_log_path() 
        ) {
            eprintln!("{}: {e}", DB_EVENT_LOG_ERROR);
        }

        Ok((true, "Deleted document".to_string()))
    }
    */

    /// Deletes a document from database
    pub fn delete_document(
        &self,
        database_name: &str,
        document_id: &u64,
    ) -> io::Result<(bool, String)>
    {
        match db::delete_document(&database_file_path(database_name), document_id) {
            Ok((result, message)) => {
                if !result {
                    return Ok((false, format!("Failed to delete document: {message}")));
                }
            },
            Err(e) => return Err(e),
        }

        if let Err(e) = Logger::log_event(
            DatabaseEventSource::Document,
            DatabaseEvent::Deleted,
            &format!(
                "Deleted document with ID '{}' from database '{}'",
                document_id, database_name
            ),
            &get_db_events_log_path() 
        ) {
            eprintln!("{}: {e}", DB_EVENT_LOG_ERROR);
        }

        Ok((true, "Deleted document".to_string()))
    }

    /// Finds all documents of collection
    pub fn find_all_documents_of_collection(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> io::Result<Vec<FormattedDocument>>
    {
        match db::find_all_documents_of_collection(
            &database_file_path(database_name),
            collection_name
        ) {
            Ok(documents) => return Ok(documents),
            Err(e) => return Err(e),
        };
    }

    /// Finds a document from a database by its id.
    pub fn find_document_by_id(
        &self,
        document_id: &u64,
        database_name: &str,
    ) -> io::Result<(Option<FormattedDocument>, String)>
    {
        match db::find_document_by_id(document_id, &database_file_path(database_name)) {
            Ok((result, message)) => return Ok((
                result,
                message
            )),
            Err(e) => return Err(e),
        }
    }
}

impl DatabaseManager {
    /// Build a new database manager.
    pub fn build() -> Self {
        Self {}
    }
}