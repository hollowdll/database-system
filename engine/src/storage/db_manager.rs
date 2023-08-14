#![allow(unused)]

use std::{
    collections::HashMap,
    path::{
        Path,
        PathBuf,
    }, hash::Hash,
};
use crate::{
    logging::*,
    DocumentInputDataField,
    storage::{
        self,
        error::{
            DatabaseOperationError,
            DatabaseOperationVerboseError,
            DatabaseOperationErrorKind,
        },
        pb::document::DataType,
        database::*,
        collection::*,
        document::*,
        create_db_dir_if_not_exists,
        DB_FILE_EXTENSION,
    },
    config::Config,
};
use super::error::DocumentError;

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
    /// Creates a new database to database directory.
    pub fn create_database_to_db_dir(
        &self,
        db_name: &str,
    ) -> Result<(), DatabaseOperationVerboseError>
    {
        if let Err(err) = create_db_dir_if_not_exists(&self.db_dir_path()) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::CreateDatabase,
                format!("Failed to create databases directory: {}", err)
            ));
        }

        if let Err(err) = create_database_file(
            db_name,
            &self.db_file_path(db_name)
        ) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::CreateDatabase,
                err.to_string()
            ));
        }

        Ok(())
    }

    /// Creates a new database by file path.
    pub fn create_database_by_file_path(
        &self,
        db_name: &str,
        db_file_path: &Path,
    ) -> Result<(), DatabaseOperationVerboseError>
    {
        if let Err(err) = create_database_file(
            db_name,
            db_file_path
        ) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::CreateDatabase,
                err.to_string()
            ));
        }

        Ok(())
    }

    /// Deletes a database.
    pub fn delete_database(
        &self,
        db_file_path: &Path,
    ) -> Result<(), DatabaseOperationVerboseError>
    {
        if let Err(err) = delete_database_file(db_file_path) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::DeleteDatabase,
                err.to_string()
            ));
        }
        
        Ok(())
    }

    /// Changes description of a database.
    pub fn change_database_description(
        &self,
        db_file_path: &Path,
        description: &str,
    ) -> Result<(), DatabaseOperationVerboseError>
    {
        if let Err(err) = change_database_description(
            description,
            db_file_path
        ) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::ModifyDatabase,
                err.to_string()
            ));
        }

        Ok(())
    }

    /// Creates a new collection to a database.
    pub fn create_collection(
        &self,
        collection_name: &str,
        db_file_path: &Path,
    ) -> Result<(), DatabaseOperationVerboseError>
    {
        if let Err(err) = create_collection_to_database(
            collection_name,
            db_file_path
        ) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::CreateCollection,
                err.to_string()
            ));
        }

        Ok(())
    }

    /// Deletes a collection from a database.
    pub fn delete_collection(
        &self,
        collection_name: &str,
        db_file_path: &Path,
    ) -> Result<(), DatabaseOperationVerboseError>
    {
        if let Err(err) = delete_collection_from_database(
            collection_name,
            db_file_path
        ) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::DeleteCollection,
                err.to_string()
            ));
        }

        Ok(())
    }

    /// Creates a new document to a collection.
    /// 
    /// Validates input data and parses it into correct document data types.
    /// 
    /// Returns the created document.
    pub fn create_document(
        &self,
        db_file_path: &Path,
        collection_name: &str,
        input_data: Vec<DocumentInputDataField>,
    ) -> Result<DocumentDto, DatabaseOperationVerboseError>
    {
        let mut document_data: HashMap<String, DataType> = HashMap::new();

        // Validate input data
        for data_field in input_data {
            // Don't allow empty field name
            if data_field.field().is_empty() {
                return Err(DatabaseOperationVerboseError::new(
                    DatabaseOperationErrorKind::CreateDocument,
                    DocumentError::EmptyFieldName.to_string()
                ));
            }

            let converted_value = match data_field.parse_to_document_data_type(
                data_field.value(),
                data_field.data_type()
            ) {
                Ok(converted_value) => converted_value,
                Err(err) => return Err(DatabaseOperationVerboseError::new(
                    DatabaseOperationErrorKind::CreateDocument,
                    format!(
                        "Data type '{}' is not valid: {}",
                        data_field.data_type(),
                        err
                    )
                )),
            };

            document_data.insert(data_field.field().to_string(), converted_value);
        }

        let created_document = match create_document_to_collection(
            db_file_path,
            collection_name,
            document_data
        ) {
            Ok(created_document) => created_document,
            Err(err) => return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::CreateDocument,
                err.to_string()
            )),
        };

        Ok(created_document)
    }

    /// Replaces a document's data. Keeps the document id.
    /// 
    /// Validates input data and parses it into correct document data types.
    pub fn replace_document(
        &self,
        db_file_path: &Path,
        document_id: &u64,
        collection_name: &str,
        input_data: Vec<DocumentInputDataField>,
    ) -> Result<(), DatabaseOperationVerboseError>
    {
        let mut document_data: HashMap<String, DataType> = HashMap::new();

        // Validate input data
        for data_field in input_data {
            // Don't allow empty field name
            if data_field.field().is_empty() {
                return Err(DatabaseOperationVerboseError::new(
                    DatabaseOperationErrorKind::ReplaceDocument,
                    DocumentError::EmptyFieldName.to_string()
                ));
            }

            let converted_value = match data_field.parse_to_document_data_type(
                data_field.value(),
                data_field.data_type()
            ) {
                Ok(converted_value) => converted_value,
                Err(err) => return Err(DatabaseOperationVerboseError::new(
                    DatabaseOperationErrorKind::ReplaceDocument,
                    format!(
                        "Data type '{}' is not valid: {}",
                        data_field.data_type(),
                        err
                    )
                )),
            };

            document_data.insert(data_field.field().to_string(), converted_value);
        }

        if let Err(err) = replace_document_in_collection(
            db_file_path,
            document_id,
            collection_name,
            document_data
        ) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::ReplaceDocument,
                err.to_string()
            ));
        }

        Ok(())
    }

    /// Deletes a document from a collection.
    pub fn delete_document(
        &self,
        db_file_path: &Path,
        document_id: &u64,
        collection_name: &str,
    ) -> Result<(), DatabaseOperationVerboseError>
    {
        if let Err(err) = delete_document_from_collection(
            db_file_path,
            document_id,
            collection_name,
        ) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::DeleteDocument,
                err.to_string()
            ));
        }

        Ok(())
    }

    /// Finds all databases from database directory.
    pub fn find_all_databases(
        &self,
    ) -> Result<Vec<DatabaseDto>, DatabaseOperationVerboseError>
    {
        if let Err(err) = create_db_dir_if_not_exists(&self.db_dir_path()) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindDatabaseMany,
                format!("Failed to create database directory: {}", err)
            ));
        }

        match find_all_databases(&self.db_dir_path()) {
            Ok(databases) => return Ok(databases),
            Err(err) => return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindDatabaseMany,
                err.to_string()
            )),
        }
    }

    /// Finds a database from database directory.
    pub fn find_database(
        &self,
        db_name: &str,
    ) -> Result<Option<DatabaseDto>, DatabaseOperationVerboseError>
    {
        if let Err(err) = create_db_dir_if_not_exists(&self.db_dir_path()) {
            return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindDatabaseOne,
                format!("Failed to create databases directory: {}", err)
            ));
        }

        match find_database(db_name, &self.db_dir_path()) {
            Ok(database) => return Ok(database),
            Err(err) => return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindDatabaseOne,
                err.to_string()
            )),
        }
    }

    /// Finds a database by file path.
    pub fn find_database_by_file_path(
        &self,
        db_file_path: &Path,
    ) -> Result<Option<DatabaseDto>, DatabaseOperationVerboseError>
    {
        match find_database_by_file_path(db_file_path) {
            Ok(db) => return Ok(db),
            Err(err) => return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindDatabaseOne,
                err.to_string()
            )),
        }
    }

    /// Finds all collections from a database.
    pub fn find_all_collections(
        &self,
        db_file_path: &Path,
    ) -> Result<Vec<CollectionDto>, DatabaseOperationVerboseError>
    {
        match find_all_collections_from_database(db_file_path) {
            Ok(collections) => return Ok(collections),
            Err(err) => return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindCollectionMany,
                err.to_string()
            )),
        }
    }

    /// Finds a collection from a database.
    pub fn find_collection(
        &self,
        collection_name: &str,
        db_file_path: &Path,
    ) -> Result<Option<CollectionDto>, DatabaseOperationVerboseError>
    {
        match find_collection_from_database(
            collection_name,
            db_file_path
        ) {
            Ok(collection) => return Ok(collection),
            Err(err) => return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindCollectionOne,
                err.to_string()
            )),
        }
    }

    /// Finds all documents from a collection.
    pub fn find_all_documents(
        &self,
        db_file_path: &Path,
        collection_name: &str,
    ) -> Result<Vec<DocumentDto>, DatabaseOperationVerboseError>
    {
        match find_all_documents_from_collection(
            db_file_path,
            collection_name
        ) {
            Ok(document) => return Ok(document),
            Err(err) => return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindDocumentMany,
                err.to_string()
            )),
        }
    }

    /// Finds the first documents from a collection specified by limit.
    pub fn find_documents_limit(
        &self,
        db_file_path: &Path,
        collection_name: &str,
        limit: usize,
    ) -> Result<Vec<DocumentDto>, DatabaseOperationVerboseError>
    {
        match find_documents_from_collection_limit(
            db_file_path,
            collection_name,
            limit
        ) {
            Ok(document) => return Ok(document),
            Err(err) => return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindDocumentMany,
                err.to_string()
            )),
        }
    }

    /// Finds a document from a collection by document id.
    pub fn find_document_by_id(
        &self,
        document_id: &u64,
        db_file_path: &Path,
        collection_name: &str,
    ) -> Result<Option<DocumentDto>, DatabaseOperationVerboseError>
    {
        match find_document_from_collection_by_id(
            db_file_path,
            document_id,
            collection_name,
        ) {
            Ok(document) => return Ok(document),
            Err(err) => return Err(DatabaseOperationVerboseError::new(
                DatabaseOperationErrorKind::FindDocumentOne,
                err.to_string()
            )),
        }
    }
}
