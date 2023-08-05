// Data storage API

use std::path::Path;
use crate::{
    DatabaseManager,
    Logger,
    storage::{
        error::DatabaseOperationError,
        database::DatabaseDto,
        collection::CollectionDto,
        document::DocumentDto,
    },
    DocumentInputDataField,
    logging::ErrorLogType,
};

/// Result for database operations that request databases.
/// 
/// Storage API methods return this.
pub struct DatabaseRequestResult {
    success: bool,
    message: String,
    data: DatabaseDto,
    log_error_message: Option<String>,
}

/// Result for database operations that request collections.
/// 
/// Storage API methods return this.
pub struct CollectionRequestResult {
    success: bool,
    message: String,
    data: CollectionDto,
    log_error_message: Option<String>,
}

/// Result for database operations that request documents.
/// 
/// Storage API methods return this.
pub struct DocumentRequestResult {
    success: bool,
    message: String,
    data: DocumentDto,
    log_error_message: Option<String>,
}

/// Data storage API.
/// 
/// Provides methods to do database operations.
/// 
/// Logs errors and events before forwarding results to the caller.
pub struct StorageApi<'a> {
    db_manager: DatabaseManager<'a>,
    logger: &'a Logger<'a>,
}

impl<'a> StorageApi<'a> {
    /// Builds storage API.
    pub fn build(
        db_manager: DatabaseManager<'a>,
        logger: &'a Logger<'a>,
    ) -> StorageApi<'a>
    {
        StorageApi {
            db_manager,
            logger,
        }
    }
}


impl<'a> StorageApi<'a> {
    /// Requests `DatabaseManager` to create a database to database directory.
    /// 
    /// Forwards the result to the caller.
    pub fn create_database_to_db_dir(
        &self,
        db_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager.create_database_to_db_dir(db_name) {
            Ok(()) => {
                let content = format!("Created database '{}' to database directory", db_name);
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(());
            },
            Err(e) => {
                let content = format!("Failed to create database to database directory: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to create a database by file path.
    /// 
    /// Forwards the result to the caller.
    pub fn create_database_by_file_path(
        &self,
        db_name: &str,
        db_file_path: &Path,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager.create_database_by_file_path(db_name, db_file_path) {
            Ok(()) => {
                let content = format!("Created database '{}'", db_file_path.display());
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(());
            },
            Err(e) => {
                let content = format!("Failed to create database: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }
    
    /// Requests `DatabaseManager` to delete a database.
    /// 
    /// Forwards the result to the caller.
    pub fn delete_database(
        &self,
        db_file_path: &Path,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager.delete_database(db_file_path) {
            Ok(()) => {
                let content = format!("Deleted database '{}'", db_file_path.display());
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(());
            },
            Err(e) => {
                let content = format!("Failed to delete database: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to change the description of a database.
    /// 
    /// Forwards the result to the caller.
    pub fn change_database_description(
        &self,
        db_file_path: &Path,
        description: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager.change_database_description(db_file_path, description) {
            Ok(()) => {
                let content = format!("Changed description of database '{}'", db_file_path.display());
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(());
            },
            Err(e) => {
                let content = format!("Failed to change database description: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to create a new collection.
    /// 
    /// Forwards the result to the caller.
    pub fn create_collection(
        &self,
        collection_name: &str,
        db_file_path: &Path,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager.create_collection(collection_name, db_file_path) {
            Ok(()) => {
                let content = format!(
                    "Created collection '{}' to database '{}'",
                    collection_name,
                    db_file_path.display()
                );
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(());
            },
            Err(e) => {
                let content = format!("Failed to create collection: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to delete a collection.
    /// 
    /// Forwards the result to the caller.
    pub fn delete_collection(
        &self,
        collection_name: &str,
        db_file_path: &Path,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager.delete_collection(collection_name, db_file_path) {
            Ok(()) => {
                let content = format!(
                    "Deleted collection '{}' from database '{}'",
                    collection_name,
                    db_file_path.display()
                );
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(());
            },
            Err(e) => {
                let content = format!("Failed to delete collection: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to create a new document to a collection.
    /// 
    /// Forwards the result to the caller.
    pub fn create_document(
        &self,
        db_file_path: &Path,
        collection_name: &str,
        data: Vec<DocumentInputDataField>,
    ) -> Result<DocumentDto, DatabaseOperationError>
    {
        match self.db_manager.create_document(db_file_path, collection_name, data) {
            Ok(created_document) => {
                let content = format!(
                    "Created document with ID '{}' to collection '{}' in database '{}'",
                    created_document.id(),
                    collection_name,
                    db_file_path.display()
                );
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(created_document);
            },
            Err(e) => {
                let content = format!("Failed to create document: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to replace a document's data.
    /// 
    /// Forwards the result to the caller.
    pub fn replace_document(
        &self,
        db_file_path: &Path,
        document_id: &u64,
        collection_name: &str,
        data: Vec<DocumentInputDataField>,
    ) -> Result<(), DatabaseOperationError>
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
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(());
            },
            Err(e) => {
                let content = format!("Failed to replace document: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to delete a document.
    /// 
    /// Forwards the result to the caller.
    pub fn delete_document(
        &self,
        db_file_path: &Path,
        document_id: &u64,
        collection_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager.delete_document(db_file_path, document_id, collection_name) {
            Ok(()) => {
                let content = format!(
                    "Deleted document with ID '{}' from collection '{}' in database '{}'",
                    document_id,
                    collection_name,
                    db_file_path.display()
                );
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(());
            },
            Err(e) => {
                let content = format!("Failed to delete document: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to find all databases from database directory.
    /// 
    /// Forwards the result to the caller.
    pub fn find_all_databases(
        &self,
    ) -> Result<Vec<DatabaseDto>, DatabaseOperationError>
    {
        match self.db_manager.find_all_databases() {
            Ok(databases) => {
                let content = "Fetched all databases from database directory";
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(databases);
            },
            Err(e) => {
                let content = format!("Failed to fetch all databases from database directory: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to find a database from database directory.
    /// 
    /// Forwards the result to the caller.
    pub fn find_database(
        &self,
        db_name: &str,
    ) -> Result<Option<DatabaseDto>, DatabaseOperationError>
    {
        match self.db_manager.find_database(db_name) {
            Ok(database) => {
                let content = format!("Fetched database '{}' from database directory", db_name);
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(database);
            },
            Err(e) => {
                let content = format!("Failed to fetch database from database directory: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to find a database by file path.
    /// 
    /// Forwards the result to the caller.
    pub fn find_database_by_file_path(
        &self,
        db_file_path: &Path,
    ) -> Result<Option<DatabaseDto>, DatabaseOperationError>
    {
        match self.db_manager.find_database_by_file_path(db_file_path) {
            Ok(database) => {
                let content = format!("Fetched database '{}'", db_file_path.display());
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(database);
            },
            Err(e) => {
                let content = format!("Failed to fetch database: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to find all collections from a database.
    /// 
    /// Forwards the result to the caller.
    pub fn find_all_collections(
        &self,
        db_file_path: &Path,
    ) -> Result<Vec<CollectionDto>, DatabaseOperationError>
    {
        match self.db_manager.find_all_collections(db_file_path) {
            Ok(collections) => {
                let content = format!(
                    "Fetched all collections from database '{}'",
                    db_file_path.display()
                );
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(collections);
            },
            Err(e) => {
                let content = format!("Failed to fetch all collections: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to find a collection from a database.
    /// 
    /// Forwards the result to the caller.
    pub fn find_collection(
        &self,
        collection_name: &str,
        db_file_path: &Path,
    ) -> Result<Option<CollectionDto>, DatabaseOperationError>
    {
        match self.db_manager.find_collection(collection_name, db_file_path) {
            Ok(collection) => {
                let content = format!(
                    "Fetched collection '{}' from database '{}'",
                    collection_name,
                    db_file_path.display()
                );
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(collection);
            },
            Err(e) => {
                let content = format!("Failed to fetch collection: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to find all documents from a collection.
    /// 
    /// Forwards the result to the caller.
    pub fn find_all_documents(
        &self,
        db_file_path: &Path,
        collection_name: &str,
    ) -> Result<Vec<DocumentDto>, DatabaseOperationError>
    {
        match self.db_manager.find_all_documents(db_file_path, collection_name) {
            Ok(documents) => {
                let content = format!(
                    "Fetched all documents from collection '{}' in database '{}'",
                    collection_name,
                    db_file_path.display()
                );
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(documents);
            },
            Err(e) => {
                let content = format!("Failed to fetch all documents: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to find the first documents from a collection specified by limit.
    /// 
    /// Forwards the result to the caller.
    pub fn find_documents_limit(
        &self,
        db_file_path: &Path,
        collection_name: &str,
        limit: usize,
    ) -> Result<Vec<DocumentDto>, DatabaseOperationError>
    {
        match self.db_manager.find_documents_limit(db_file_path, collection_name, limit) {
            Ok(documents) => {
                let content = format!(
                    "Fetched {} documents from collection '{}' in database '{}'",
                    documents.len(),
                    collection_name,
                    db_file_path.display()
                );
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(documents);
            },
            Err(e) => {
                let content = format!("Failed to fetch documents: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }

    /// Requests `DatabaseManager` to find a document from a collection by document id.
    /// 
    /// Forwards the result to the caller.
    pub fn find_document_by_id(
        &self,
        document_id: &u64,
        db_file_path: &Path,
        collection_name: &str,
    ) -> Result<Option<DocumentDto>, DatabaseOperationError>
    {
        match self.db_manager.find_document_by_id(document_id, db_file_path, collection_name) {
            Ok(document) => {
                let content = format!(
                    "Fetched document with ID '{}' from collection '{}' in database '{}'",
                    document_id,
                    collection_name,
                    db_file_path.display()
                );
                if let Err(e) = self.logger.log_event(&content) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Ok(document);
            },
            Err(e) => {
                let content = format!("Failed to fetch document: {}", e);
                if let Err(e) = self.logger.log_error(
                    ErrorLogType::Error,
                    &content,
                ) {
                    return Err(DatabaseOperationError(e.to_string()));
                }
                return Err(e);
            },
        }
    }
}
