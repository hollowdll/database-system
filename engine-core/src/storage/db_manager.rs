// Database manager Protocol Buffers

#![allow(unused)]

use std::{
    collections::HashMap,
    path::{
        Path,
        PathBuf,
    },
};
use crate::{
    logging::*,
    DocumentInputDataField,
    storage::{
        self,
        error::DatabaseOperationError,
        pb::document::DataType,
        database::*,
        collection::*,
        document::*,
        create_db_dir_if_not_exists,
        DB_FILE_EXTENSION,
    },
    config::Config,
};

/// Database manager that manages all databases and database operations.
/// 
/// Stores and retrieves data from databases.
#[derive(Debug)]
pub struct DatabaseManager<'a> {
    config: &'a Config,
}

impl<'a> DatabaseManager<'a> {
    /// Builds database manager.
    pub fn build(config: &'a Config) -> Self {
        Self {
            config
        }
    }
}

impl<'a> DatabaseManager<'a> {
    /// Gets databases directory path.
    fn db_dir_path(&self) -> &Path {
        &self.config.db_dir_path
    }

    /// Gets database file path.
    fn db_file_path(&self, db_name: &str) -> PathBuf {
        PathBuf::from(&self.db_dir_path()
            .join(format!("{}.{}", db_name, DB_FILE_EXTENSION)))
    }
}

impl<'a> DatabaseManager<'a> {
    /// Creates a new database.
    pub fn create_database(
        &self,
        db_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        if let Err(err) = create_db_dir_if_not_exists(&self.db_dir_path()) {
            return Err(DatabaseOperationError(
                format!("Failed to create databases directory: {}", err)
            ));
        }

        if let Err(err) = create_database_file(
            db_name,
            &self.db_file_path(db_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to create database '{}': {}",
                db_name,
                err
            )));
        }

        Ok(())
    }

    /// Deletes a database.
    pub fn delete_database(
        &self,
        db_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        if let Err(err) = delete_database_file(
            &self.db_file_path(db_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to delete database '{}': {}",
                db_name,
                err
            )));
        }
        
        Ok(())
    }

    /// Changes description of a database.
    pub fn change_database_description(
        &self,
        db_name: &str,
        description: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        if let Err(err) = change_database_description(
            description,
            &self.db_file_path(db_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to change description of database '{}': {}",
                db_name,
                err
            )));
        }

        Ok(())
    }

    /// Creates a new collection to a database.
    pub fn create_collection(
        &self,
        collection_name: &str,
        db_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        if let Err(err) = create_collection_to_db_file(
            collection_name,
            &self.db_file_path(db_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to create collection '{}' to database '{}': {}",
                collection_name,
                db_name,
                err
            )));
        }

        Ok(())
    }

    /// Deletes a collection from a database.
    pub fn delete_collection(
        &self,
        collection_name: &str,
        db_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        if let Err(err) = delete_collection_from_db_file(
            collection_name,
            &self.db_file_path(db_name)
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to delete collection '{}' from database '{}': {}",
                collection_name,
                db_name,
                err
            )));
        }

        Ok(())
    }

    /// Creates a new document to a collection
    pub fn create_document(
        &self,
        db_name: &str,
        collection_name: &str,
        input_data: Vec<DocumentInputDataField>,
    ) -> Result<(), DatabaseOperationError>
    {
        let mut document_data: HashMap<String, DataType> = HashMap::new();

        // convert input data to correct document data types
        for data_field in input_data {
            let converted_value = match data_field.parse_to_document_data_type(
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

        if let Err(err) = create_document_to_db_file(
            &self.db_file_path(db_name),
            collection_name,
            document_data
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to create document to collection '{}' in database '{}': {}",
                collection_name,
                db_name,
                err
            )));
        }

        Ok(())
    }

    /// Deletes a document from a collection.
    pub fn delete_document(
        &self,
        db_name: &str,
        document_id: &u64,
        collection_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        if let Err(err) = delete_document_from_db_file(
            &self.db_file_path(db_name),
            document_id,
            collection_name,
        ) {
            return Err(DatabaseOperationError(format!(
                "Failed to delete document with ID '{}' from collection '{}' in database '{}': {}",
                document_id,
                collection_name,
                db_name,
                err
            )));
        }

        Ok(())
    }

    /// Finds all databases.
    pub fn find_all_databases(
        &self,
    ) -> Result<Vec<DatabaseDto>, DatabaseOperationError>
    {
        if let Err(err) = create_db_dir_if_not_exists(&self.db_dir_path()) {
            return Err(DatabaseOperationError(
                format!("Failed to create databases directory: {}", err)
            ));
        }

        match find_all_databases(&self.db_dir_path()) {
            Ok(databases) => return Ok(databases),
            Err(err) => return Err(DatabaseOperationError(
                format!("Failed to find all databases: {}", err)
            )),
        }
    }

    /// Finds a database.
    pub fn find_database(
        &self,
        db_name: &str,
    ) -> Result<Option<DatabaseDto>, DatabaseOperationError>
    {
        if let Err(err) = storage::create_db_dir_if_not_exists(&self.db_dir_path()) {
            return Err(DatabaseOperationError(
                format!("Failed to create databases directory: {}", err)
            ));
        }

        match find_database(db_name, &self.db_dir_path()) {
            Ok(database) => return Ok(database),
            Err(err) => return Err(DatabaseOperationError(
                format!("Failed to find database '{}': {}", db_name, err)
            )),
        }
    }

    /// Finds all collections from a database.
    pub fn find_all_collections(
        &self,
        db_name: &str,
    ) -> Result<Vec<CollectionDto>, DatabaseOperationError>
    {
        match find_all_collections_from_db_file(
            &self.db_file_path(db_name)
        ) {
            Ok(collections) => return Ok(collections),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find all collections from database '{}': {}",
                db_name,
                err
            ))),
        }
    }

    /// Finds a collection from a database.
    pub fn find_collection(
        &self,
        collection_name: &str,
        db_name: &str,
    ) -> Result<Option<CollectionDto>, DatabaseOperationError>
    {
        match find_collection_from_db_file(
            collection_name,
            &self.db_file_path(db_name)
        ) {
            Ok(collection) => return Ok(collection),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find collection '{}' from database '{}': {}",
                collection_name,
                db_name,
                err
            ))),
        }
    }

    /// Finds all documents from a collection.
    pub fn find_all_documents(
        &self,
        db_name: &str,
        collection_name: &str,
    ) -> Result<Vec<DocumentDto>, DatabaseOperationError>
    {
        match find_all_documents_from_collection(
            &self.db_file_path(db_name),
            collection_name
        ) {
            Ok(document) => return Ok(document),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find all documents from collection '{}' in database '{}': {}",
                collection_name,
                db_name,
                err
            ))),
        }
    }

    /// Finds the first documents from a collection specified by limit.
    pub fn find_documents_limit(
        &self,
        db_name: &str,
        collection_name: &str,
        limit: usize,
    ) -> Result<Vec<DocumentDto>, DatabaseOperationError>
    {
        match find_documents_from_collection_limit(
            &self.db_file_path(db_name),
            collection_name,
            limit
        ) {
            Ok(document) => return Ok(document),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find documents from collection '{}' in database '{}' with limit {}: {}",
                collection_name,
                db_name,
                limit,
                err
            ))),
        }
    }

    /// Finds a document from a collection by document id.
    pub fn find_document_by_id(
        &self,
        document_id: &u64,
        db_name: &str,
        collection_name: &str,
    ) -> Result<Option<DocumentDto>, DatabaseOperationError>
    {
        match find_document_from_collection_by_id(
            &self.db_file_path(db_name),
            document_id,
            collection_name,
        ) {
            Ok(document) => return Ok(document),
            Err(err) => return Err(DatabaseOperationError(format!(
                "Failed to find document with ID '{}' from collection '{}' in database '{}': {}",
                document_id,
                collection_name,
                db_name,
                err
            ))),
        }
    }
}
