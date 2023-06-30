// This file contains database manager related code

use std::{
    collections::HashMap,
    path::{
        Path,
        PathBuf,
    },
};
use crate::{
    logging::*,
    InputDataField,
    db::{
        self,
        DataType,
        FormattedDatabase,
        FormattedDocumentCollection,
        FormattedDocument,
        error::DatabaseOperationError,
        DB_FILE_EXTENSION,
    },
};

/// Database manager that manages all databases
/// and database related operations
#[derive(PartialEq, Debug)]
pub struct DatabaseManager {
    /// Directory path where databases will be created.
    db_dir_path: PathBuf,

    /// Directory path where logs will be created.
    logs_dir_path: PathBuf,
}

impl DatabaseManager {
    /// Build a new database manager.
    pub fn build(db_dir_path: PathBuf, logs_dir_path: PathBuf) -> Self {
        Self {
            db_dir_path,
            logs_dir_path,
        }
    }
}

impl DatabaseManager {
    /// Gets databases directory path.
    fn db_dir_path(&self) -> &Path {
        &self.db_dir_path
    }

    /// Gets logs directory path.
    pub fn logs_dir_path(&self) -> &Path {
        &self.logs_dir_path
    }

    /// Gets database file path.
    fn db_file_path(&self, db_name: &str) -> PathBuf {
        PathBuf::from(&self.db_dir_path()
            .join(format!("{}.{}", db_name, DB_FILE_EXTENSION)))
    }

    /// Attempts to log events with configured logs directory to log file.
    pub fn log_event(&self, content: &str) {
        if let Err(e) = Logger::log_event(
            content,
            &self.logs_dir_path(),
            &self.logs_dir_path().join(DB_EVENTS_LOG)
        ) {
            eprintln!("[Error] {}", e);
        }
    }

    /// Attempts to log errors with configured logs directory to log file.
    pub fn log_error(&self, content: &str) {
        if let Err(e) = Logger::log_error(
            ErrorLogType::Error,
            content,
            &self.logs_dir_path(),
            &self.logs_dir_path().join(ERRORS_LOG)
        ) {
            eprintln!("[Error] {}", e);
        }
    }
}

impl DatabaseManager {
    /// Creates a new database 
    pub fn create_database(
        &self,
        database_name: &str,
    ) -> Result<String, DatabaseOperationError>
    {
        if let Err(err) = db::create_db_dir_if_not_exists(&self.db_dir_path()) {
            return Err(DatabaseOperationError(format!(
                "Failed to create database '{}': {}",
                database_name,
                err
            )));
        }

        if let Err(err) = db::create_database_file(
            database_name,
            &self.db_file_path(database_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to create database '{}': {}",
                database_name,
                err
            )));
        }

        Ok(format!("Created database '{}'", database_name))
    }

    /// Deletes a database
    pub fn delete_database(
        &self,
        database_name: &str,
    ) -> Result<String, DatabaseOperationError>
    {
        if let Err(err) = db::delete_database_file(
            database_name,
            &self.db_file_path(database_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to delete database '{}': {}",
                database_name,
                err
            )));
        }
        
        Ok(format!("Deleted database '{}'", database_name))
    }

    /// Changes description of a database
    pub fn change_database_description(
        &self,
        database_name: &str,
        description: &str,
    ) -> Result<String, DatabaseOperationError>
    {
        if let Err(err) = db::change_database_description(
            description,
            &self.db_file_path(database_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to change description of database '{}': {}",
                database_name,
                err
            )));
        }

        Ok(format!("Changed description of database '{}'", database_name))
    }

    /// Creates a new collection to a database
    pub fn create_collection(
        &self,
        collection_name: &str,
        database_name: &str,
    ) -> Result<String, DatabaseOperationError>
    {
        if let Err(err) = db::create_collection_to_db_file(
            collection_name,
            &self.db_file_path(database_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to create collection '{}' to database '{}': {}",
                collection_name,
                database_name,
                err
            )));
        }

        Ok(format!(
            "Created collection '{}' to database '{}'",
            collection_name,
            database_name
        ))
    }

    /// Deletes a collection from a database
    pub fn delete_collection(
        &self,
        collection_name: &str,
        database_name: &str,
    ) -> Result<String, DatabaseOperationError>
    {
        if let Err(err) = db::delete_collection_from_db_file(
            collection_name,
            &self.db_file_path(database_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to delete collection '{}' from database '{}': {}",
                collection_name,
                database_name,
                err
            )));
        }

        Ok(format!(
            "Deleted collection '{}' from database '{}'",
            collection_name,
            database_name
        ))
    }

    /// Creates a new document to a collection
    pub fn create_document(
        &self,
        database_name: &str,
        collection_name: &str,
        data: Vec<InputDataField>,
    ) -> Result<String, DatabaseOperationError>
    {
        let mut document_data: HashMap<String, DataType> = HashMap::new();

        // convert input data to correct document data types
        for data_field in data {
            let converted_value = match data_field.convert_to_document_data_type(
                data_field.value(),
                data_field.data_type()
            ) {
                Ok(converted_value) => converted_value,
                Err(err) => return Err(DatabaseOperationError(format!(
                    "Data type '{}' is not valid: {}",
                    data_field.data_type(),
                    err
                ))),
            };

            document_data.insert(data_field.field().to_string(), converted_value);
        }

        if let Err(err) = db::create_document(
            &self.db_file_path(database_name),
            collection_name,
            document_data
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to create document to collection '{}' in database '{}': {}",
                collection_name,
                database_name,
                err
            )));
        }

        Ok(format!(
            "Created document to collection '{}' in database '{}'",
            collection_name,
            database_name
        ))
    }

    /// Deletes a document from database
    pub fn delete_document(
        &self,
        database_name: &str,
        document_id: &u64,
    ) -> Result<String, DatabaseOperationError>
    {
        if let Err(err) = db::delete_document(
            &self.db_file_path(database_name),
            document_id
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to delete document with ID '{}' from database '{}': {}",
                document_id,
                database_name,
                err
            )));
        }

        Ok(format!(
            "Deleted document with ID '{}' from database '{}'",
            document_id,
            database_name
        ))
    }

    /// Finds all databases.
    pub fn find_all_databases(
        &self,
    ) -> Result<Vec<FormattedDatabase>, DatabaseOperationError>
    {
        if let Err(err) = db::create_db_dir_if_not_exists(&self.db_dir_path()) {
            return Err(DatabaseOperationError(format!(
                "Failed to find all databases: {}",
                err
            )));
        }

        match db::find_all_databases(&self.db_dir_path()) {
            Ok(databases) => return Ok(databases),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find all databases: {}",
                err
            ))),
        }
    }

    /// Finds a database.
    pub fn find_database(
        &self,
        database_name: &str,
    ) -> Result<Option<FormattedDatabase>, DatabaseOperationError>
    {
        if let Err(err) = db::create_db_dir_if_not_exists(&self.db_dir_path()) {
            return Err(DatabaseOperationError(format!(
                "Failed to find all databases: {}",
                err
            )));
        }

        match db::find_database(database_name, &self.db_dir_path()) {
            Ok(database) => return Ok(database),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find all databases: {}",
                err
            ))),
        }
    }

    /// Finds all collections from a database.
    pub fn find_all_collections(
        &self,
        database_name: &str,
    ) -> Result<Vec<FormattedDocumentCollection>, DatabaseOperationError>
    {
        match db::find_all_collections_from_db_file(
            &self.db_file_path(database_name)
        ) {
            Ok(collections) => return Ok(collections),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find all collections of database '{}': {}",
                database_name,
                err
            ))),
        }
    }

    /// Finds a collection from a database.
    pub fn find_collection(
        &self,
        collection_name: &str,
        database_name: &str,
    ) -> Result<Option<FormattedDocumentCollection>, DatabaseOperationError>
    {
        match db::find_collection_from_db_file(
            collection_name,
            &self.db_file_path(database_name)
        ) {
            Ok(collection) => return Ok(collection),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find collection '{}' from database '{}': {}",
                collection_name,
                database_name,
                err
            ))),
        }
    }

    /// Finds all documents from collection.
    pub fn find_all_documents(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Result<Vec<FormattedDocument>, DatabaseOperationError>
    {
        match db::find_all_documents_from_collection(
            &self.db_file_path(database_name),
            collection_name
        ) {
            Ok(document) => return Ok(document),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find all documents from collection '{}' in database '{}': {}",
                collection_name,
                database_name,
                err
            ))),
        }
    }

    /// Finds a document from a database by its id.
    pub fn find_document_by_id(
        &self,
        document_id: &u64,
        database_name: &str,
    ) -> Result<Option<FormattedDocument>, DatabaseOperationError>
    {
        match db::find_document_by_id(
            document_id,
            &self.db_file_path(database_name)
        ) {
            Ok(document) => return Ok(document),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find document with ID '{}' from database '{}': {}",
                document_id,
                database_name,
                err
            ))),
        }
    }
}
