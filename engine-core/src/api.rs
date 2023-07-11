// Engine API module


use crate::{
    DatabaseManager,
    db::{
        error::DatabaseOperationError,
        database::DatabaseDto,
        collection::CollectionDto,
        document::DocumentDto,
    },
    DocumentInputDataField,
};

/// Engine API that provides methods to do database operations.
/// 
/// Logs system errors and events before forwarding data to clients.
#[derive(PartialEq, Debug)]
pub struct EngineApi<'a> {
    db_manager: DatabaseManager<'a>,
}

impl<'a> EngineApi<'a> {
    /// Builds a new instance of Engine API.
    pub fn build(db_manager: DatabaseManager) -> EngineApi {
        EngineApi {
            db_manager,
        }
    }
}

impl<'a> EngineApi<'a> {
    /// Returns an immutable reference to `DatabaseManager`.
    pub fn db_manager(&self) -> &DatabaseManager {
        &self.db_manager
    }

    /// Returns a mutable reference to `DatabaseManager`.
    pub fn db_manager_mut(&mut self) -> &'a mut DatabaseManager {
        &mut self.db_manager
    }
}

impl<'a> EngineApi<'a> {
    /// Requests `DatabaseManager` to create a database.
    /// 
    /// Forwards results to the calling client.
    pub fn create_database(
        &self,
        db_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager().create_database(db_name) {
            Ok(()) => {
                let content = format!("Created database '{}'", db_name);
                self.db_manager().log_event(&content);
                return Ok(())
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }
    
    /// Requests `DatabaseManager` to delete a database.
    /// 
    /// Forwards results to the calling client.
    pub fn delete_database(
        &self,
        db_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager().delete_database(db_name) {
            Ok(()) => {
                let content = format!("Deleted database '{}'", db_name);
                self.db_manager().log_event(&content);
                return Ok(())
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to change the description of a database.
    /// 
    /// Forwards results to the calling client.
    pub fn change_database_description(
        &self,
        db_name: &str,
        description: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager().change_database_description(
            db_name,
            description
        ) {
            Ok(()) => {
                let content = format!("Changed description of database '{}'", db_name);
                self.db_manager().log_event(&content);
                return Ok(())
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to create a new collection.
    /// 
    /// Forwards results to the calling client.
    pub fn create_collection(
        &self,
        collection_name: &str,
        db_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager().create_collection(
            collection_name,
            db_name
        ) {
            Ok(()) => {
                let content = format!(
                    "Created collection '{}' to database '{}'",
                    collection_name,
                    db_name
                );
                self.db_manager().log_event(&content);
                return Ok(())
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to delete a collection.
    /// 
    /// Forwards results to the calling client.
    pub fn delete_collection(
        &self,
        collection_name: &str,
        db_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager().delete_collection(
            collection_name,
            db_name
        ) {
            Ok(()) => {
                let content = format!(
                    "Deleted collection '{}' from database '{}'",
                    collection_name,
                    db_name
                );
                self.db_manager().log_event(&content);
                return Ok(())
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to create a new document.
    /// 
    /// Forwards results to the calling client.
    pub fn create_document(
        &self,
        db_name: &str,
        collection_name: &str,
        data: Vec<DocumentInputDataField>,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager().create_document(
            db_name,
            collection_name,
            data
        ) {
            Ok(()) => {
                let content = format!(
                    "Created document to collection '{}' in database '{}'",
                    collection_name,
                    db_name
                );
                self.db_manager().log_event(&content);
                return Ok(())
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to delete a document.
    /// 
    /// Forwards results to the calling client.
    pub fn delete_document(
        &self,
        db_name: &str,
        document_id: &u64,
        collection_name: &str,
    ) -> Result<(), DatabaseOperationError>
    {
        match self.db_manager().delete_document(
            db_name,
            document_id,
            collection_name,
        ) {
            Ok(()) => {
                let content = format!(
                    "Deleted document with ID '{}' from collection '{}' in database '{}'",
                    document_id,
                    collection_name,
                    db_name
                );
                self.db_manager().log_event(&content);
                return Ok(())
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to find all databases.
    /// 
    /// Forwards results to the calling client.
    pub fn find_all_databases(
        &self,
    ) -> Result<Vec<DatabaseDto>, DatabaseOperationError>
    {
        match self.db_manager().find_all_databases() {
            Ok(result) => {
                let content = "Fetched all databases";
                self.db_manager().log_event(content);
                
                return Ok(result)
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to find a database.
    /// 
    /// Forwards results to the calling client.
    pub fn find_database(
        &self,
        db_name: &str,
    ) -> Result<Option<DatabaseDto>, DatabaseOperationError>
    {
        match self.db_manager().find_database(db_name) {
            Ok(result) => {
                let content = format!("Fetched database '{}'", db_name);
                self.db_manager().log_event(&content);

                return Ok(result)
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to find all collections from a database.
    /// 
    /// Forwards results to the calling client.
    pub fn find_all_collections(
        &self,
        db_name: &str,
    ) -> Result<Vec<CollectionDto>, DatabaseOperationError>
    {
        match self.db_manager().find_all_collections(db_name) {
            Ok(result) => {
                let content = format!(
                    "Fetched all collections from database '{}'",
                    db_name
                );
                self.db_manager().log_event(&content);
                
                return Ok(result)
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to find a collection from a database.
    /// 
    /// Forwards results to the calling client.
    pub fn find_collection(
        &self,
        collection_name: &str,
        db_name: &str,
    ) -> Result<Option<CollectionDto>, DatabaseOperationError>
    {
        match self.db_manager().find_collection(collection_name, db_name) {
            Ok(result) => {
                let content = format!(
                    "Fetched collection '{}' from database '{}'",
                    collection_name,
                    db_name
                );
                self.db_manager().log_event(&content);
                
                return Ok(result)
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to find all documents from a collection.
    /// 
    /// Forwards results to the calling client.
    pub fn find_all_documents(
        &self,
        db_name: &str,
        collection_name: &str,
    ) -> Result<Vec<DocumentDto>, DatabaseOperationError>
    {
        match self.db_manager().find_all_documents(db_name, collection_name) {
            Ok(result) => {
                let content = format!(
                    "Fetched all documents from collection '{}' in database '{}'",
                    collection_name,
                    db_name
                );
                self.db_manager().log_event(&content);
                
                return Ok(result)
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }

    /// Requests `DatabaseManager` to find a document from a database by document id.
    /// 
    /// Forwards results to the calling client.
    pub fn find_document_by_id(
        &self,
        document_id: &u64,
        db_name: &str,
        collection_name: &str,
    ) -> Result<Option<DocumentDto>, DatabaseOperationError>
    {
        match self.db_manager().find_document_by_id(document_id, db_name, collection_name) {
            Ok(result) => {
                let content = format!(
                    "Fetched document with ID '{}' from collection '{}' in database '{}'",
                    document_id,
                    collection_name,
                    db_name
                );
                self.db_manager().log_event(&content);
                
                return Ok(result)
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }
}
