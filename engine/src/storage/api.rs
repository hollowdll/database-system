// Data storage API

use std::path::Path;
use crate::{
    storage::db_manager::DatabaseManager,
    Logger,
    storage::{
        error::DatabaseOperationError,
        database::DatabaseDto,
        collection::CollectionDto,
        document::DocumentDto,
    },
    DocumentInputDataField,
    logging::{
        ErrorLogType,
        error::LogError,
    },
};

/// Result for storage API calls.
/// Storage API calls `DatabaseManager` methods to do database operations.
/// 
/// T = Type for returned data.
/// 
/// Storage API methods return this.
pub struct StorageRequestResult<T> {
    /// Whether result is successful.
    pub success: bool,

    /// Possible database operation error.
    pub error: Option<DatabaseOperationError>,

    /// Data that is returned.
    pub data: Option<T>,

    /// Possible error that occurred during logging.
    pub log_error: Option<LogError>,
}

/// Returns successful storage API response.
fn request_success<T>(
    data: Option<T>,
    log_result: Result<(), LogError>
) -> StorageRequestResult<T> {
    StorageRequestResult {
        success: true,
        error: None,
        data,
        log_error: match log_result {
            Ok(()) => None,
            Err(e) => Some(e),
        },
    }
}

/// Returns failed storage API response.
fn request_fail<T>(
    error: DatabaseOperationError,
    log_result: Result<(), LogError>
) -> StorageRequestResult<T> {
    StorageRequestResult {
        success: false,
        error: Some(error),
        data: None,
        log_error: match log_result {
            Ok(()) => None,
            Err(e) => Some(e),
        },
    }
}

/// Data storage API.
/// 
/// Provides methods to do database operations.
/// 
/// Logs errors and events before forwarding results to the caller.
pub struct StorageApi {
    db_manager: DatabaseManager,
    logger: Logger,
}

impl StorageApi {
    /// Builds storage API.
    pub fn build(
        db_manager: DatabaseManager,
        logger: Logger,
    ) -> StorageApi
    {
        StorageApi {
            db_manager,
            logger,
        }
    }
}


impl StorageApi {
    /// Requests `DatabaseManager` to create a database to database directory.
    pub fn create_database_to_db_dir(
        &self,
        db_name: &str,
    ) -> StorageRequestResult<()>
    {
        match self.db_manager.create_database_to_db_dir(db_name) {
            Ok(()) => {
                let content = format!("Created database '{}' to database directory", db_name);
                let result = self.logger.log_event(&content);

                return request_success(None, result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to create database '{}' to database directory: {}",
                    db_name,
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to create a database by file path.
    pub fn create_database_by_file_path(
        &self,
        db_name: &str,
        db_file_path: &Path,
    ) -> StorageRequestResult<()>
    {
        match self.db_manager.create_database_by_file_path(db_name, db_file_path) {
            Ok(()) => {
                let content = format!("Created database '{}'", db_file_path.display());
                let result = self.logger.log_event(&content);
                
                return request_success(None, result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to create database '{}': {}",
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }
    
    /// Requests `DatabaseManager` to delete a database.
    pub fn delete_database(
        &self,
        db_file_path: &Path,
    ) -> StorageRequestResult<()>
    {
        match self.db_manager.delete_database(db_file_path) {
            Ok(()) => {
                let content = format!("Deleted database '{}'", db_file_path.display());
                let result = self.logger.log_event(&content);
                
                return request_success(None, result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to delete database '{}': {}",
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to change the description of a database.
    pub fn change_database_description(
        &self,
        db_file_path: &Path,
        description: &str,
    ) -> StorageRequestResult<()>
    {
        match self.db_manager.change_database_description(db_file_path, description) {
            Ok(()) => {
                let content = format!("Changed description of database '{}'", db_file_path.display());
                let result = self.logger.log_event(&content);
                
                return request_success(None, result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to change description of database '{}': {}",
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to create a new collection.
    pub fn create_collection(
        &self,
        collection_name: &str,
        db_file_path: &Path,
    ) -> StorageRequestResult<()>
    {
        match self.db_manager.create_collection(collection_name, db_file_path) {
            Ok(()) => {
                let content = format!(
                    "Created collection '{}' to database '{}'",
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(None, result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to create collection '{}' to database '{}': {}",
                    collection_name,
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to delete a collection.
    pub fn delete_collection(
        &self,
        collection_name: &str,
        db_file_path: &Path,
    ) -> StorageRequestResult<()>
    {
        match self.db_manager.delete_collection(collection_name, db_file_path) {
            Ok(()) => {
                let content = format!(
                    "Deleted collection '{}' from database '{}'",
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(None, result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to delete collection '{}' from database '{}': {}",
                    collection_name,
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to create a new document to a collection.
    /// 
    /// Returns the created document.
    pub fn create_document(
        &self,
        db_file_path: &Path,
        collection_name: &str,
        data: Vec<DocumentInputDataField>,
    ) -> StorageRequestResult<DocumentDto>
    {
        match self.db_manager.create_document(db_file_path, collection_name, data) {
            Ok(created_document) => {
                let content = format!(
                    "Created document with ID '{}' to collection '{}' in database '{}'",
                    created_document.id(),
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(Some(created_document), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to create document to collection '{}' in database '{}': {}",
                    collection_name,
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to replace a document's data.
    pub fn replace_document(
        &self,
        db_file_path: &Path,
        document_id: &u64,
        collection_name: &str,
        data: Vec<DocumentInputDataField>,
    ) -> StorageRequestResult<()>
    {
        match self.db_manager
            .replace_document(db_file_path, document_id, collection_name, data)
        {
            Ok(()) => {
                let content = format!(
                    "Replaced document with ID '{}' in collection '{}' in database '{}'",
                    document_id,
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(None, result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to replace document with ID '{}' in collection '{}' in database '{}': {}",
                    document_id,
                    collection_name,
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to delete a document from a collection.
    pub fn delete_document(
        &self,
        db_file_path: &Path,
        document_id: &u64,
        collection_name: &str,
    ) -> StorageRequestResult<()>
    {
        match self.db_manager.delete_document(db_file_path, document_id, collection_name) {
            Ok(()) => {
                let content = format!(
                    "Deleted document with ID '{}' from collection '{}' in database '{}'",
                    document_id,
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(None, result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to delete document with ID '{}' from collection '{}' in database '{}': {}",
                    document_id,
                    collection_name,
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to delete all documents from a collection.
    /// 
    /// Returns the number of deleted documents.
    pub fn delete_all_documents(
        &self,
        db_file_path: &Path,
        collection_name: &str,
    ) -> StorageRequestResult<usize>
    {
        match self.db_manager.delete_all_documents(db_file_path, collection_name) {
            Ok(deleted_count) => {
                let content = format!(
                    "Deleted {} documents from collection '{}' in database '{}'",
                    deleted_count,
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(Some(deleted_count), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to delete documents from collection '{}' in database '{}': {}",
                    collection_name,
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to find all databases from database directory.
    /// 
    /// Returns the found databases.
    pub fn find_all_databases(
        &self,
    ) -> StorageRequestResult<Vec<DatabaseDto>>
    {
        match self.db_manager.find_all_databases() {
            Ok(databases) => {
                let content = "Fetched all databases from database directory".to_string();
                let result = self.logger.log_event(&content);
                
                return request_success(Some(databases), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to find all databases from database directory: {}",
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to find a database from database directory.
    /// 
    /// Returns the found database.
    pub fn find_database(
        &self,
        db_name: &str,
    ) -> StorageRequestResult<Option<DatabaseDto>>
    {
        match self.db_manager.find_database(db_name) {
            Ok(database) => {
                let content = format!("Fetched database '{}' from database directory", db_name);
                let result = self.logger.log_event(&content);
                
                return request_success(Some(database), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to find database '{}' from database directory: {}",
                    db_name,
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to find a database by file path.
    /// 
    /// Returns the found database.
    pub fn find_database_by_file_path(
        &self,
        db_file_path: &Path,
    ) -> StorageRequestResult<Option<DatabaseDto>>
    {
        match self.db_manager.find_database_by_file_path(db_file_path) {
            Ok(database) => {
                let content = format!("Fetched database '{}'", db_file_path.display());
                let result = self.logger.log_event(&content);
                
                return request_success(Some(database), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to find database '{}': {}",
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to find all collections from a database.
    /// 
    /// Returns the found collections.
    pub fn find_all_collections(
        &self,
        db_file_path: &Path,
    ) -> StorageRequestResult<Vec<CollectionDto>>
    {
        match self.db_manager.find_all_collections(db_file_path) {
            Ok(collections) => {
                let content = format!(
                    "Fetched all collections from database '{}'",
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(Some(collections), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to find collections from database '{}': {}",
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to find a collection from a database.
    /// 
    /// Returns the found collection.
    pub fn find_collection(
        &self,
        collection_name: &str,
        db_file_path: &Path,
    ) -> StorageRequestResult<Option<CollectionDto>>
    {
        match self.db_manager.find_collection(collection_name, db_file_path) {
            Ok(collection) => {
                let content = format!(
                    "Fetched collection '{}' from database '{}'",
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(Some(collection), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to find collection '{}' from database '{}': {}",
                    collection_name,
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to find all documents from a collection.
    /// 
    /// Returns the found documents.
    pub fn find_all_documents(
        &self,
        db_file_path: &Path,
        collection_name: &str,
    ) -> StorageRequestResult<Vec<DocumentDto>>
    {
        match self.db_manager.find_all_documents(db_file_path, collection_name) {
            Ok(documents) => {
                let content = format!(
                    "Fetched all documents from collection '{}' in database '{}'",
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(Some(documents), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to find documents from collection '{}' in database '{}': {}",
                    collection_name,
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to find the first documents from a collection specified by limit.
    /// 
    /// Returns the found documents.
    pub fn find_documents_limit(
        &self,
        db_file_path: &Path,
        collection_name: &str,
        limit: usize,
    ) -> StorageRequestResult<Vec<DocumentDto>>
    {
        match self.db_manager.find_documents_limit(db_file_path, collection_name, limit) {
            Ok(documents) => {
                let content = format!(
                    "Fetched {} documents from collection '{}' in database '{}'",
                    documents.len(),
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(Some(documents), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to find documents from collection '{}' in database '{}' with limit {}: {}",
                    collection_name,
                    db_file_path.display(),
                    limit,
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }

    /// Requests `DatabaseManager` to find a document from a collection by document id.
    /// 
    /// Returns the found document.
    pub fn find_document_by_id(
        &self,
        document_id: &u64,
        db_file_path: &Path,
        collection_name: &str,
    ) -> StorageRequestResult<Option<DocumentDto>>
    {
        match self.db_manager.find_document_by_id(document_id, db_file_path, collection_name) {
            Ok(document) => {
                let content = format!(
                    "Fetched document with ID '{}' from collection '{}' in database '{}'",
                    document_id,
                    collection_name,
                    db_file_path.display()
                );
                let result = self.logger.log_event(&content);
                
                return request_success(Some(document), result);
            },
            Err(err) => {
                let content = format!(
                    "Failed to find document with ID '{}' from collection '{}' in database '{}': {}",
                    document_id,
                    collection_name,
                    db_file_path.display(),
                    &err.message
                );
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);

                return request_fail(err, result);
            },
        }
    }
}
