// Engine API module

use crate::{
    DatabaseManager,
    db::error::DatabaseOperationError,
    InputDataField,
};

/// Engine API that provides methods to do database operations.
/// 
/// Logs system errors and events before forwarding data to clients.
#[derive(PartialEq, Debug)]
pub struct EngineApi {
    db_manager: DatabaseManager,
}

impl EngineApi {
    /// Builds a new instance of Engine API.
    pub fn build(db_manager: DatabaseManager) -> EngineApi {
        EngineApi {
            db_manager,
        }
    }
}

impl EngineApi {
    /// Returns an immutable reference to `DatabaseManager`.
    pub fn db_manager(&self) -> &DatabaseManager {
        &self.db_manager
    }

    /// Returns a mutable reference to `DatabaseManager`.
    pub fn db_manager_mut(&mut self) -> &mut DatabaseManager {
        &mut self.db_manager
    }
}

impl EngineApi {
    /// Requests `DatabaseManager` to create a database.
    /// 
    /// Forwards results to the calling client.
    pub fn create_db(
        &self,
        db_name: &str,
    ) -> Result<String, DatabaseOperationError>
    {
        match self.db_manager().create_database(db_name) {
            Ok(result) => {
                self.db_manager().log_event(&result);
                return Ok(result)
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
    pub fn delete_db(
        &self,
        db_name: &str,
    ) -> Result<String, DatabaseOperationError>
    {
        match self.db_manager().delete_database(db_name) {
            Ok(result) => {
                self.db_manager().log_event(&result);
                return Ok(result)
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
    pub fn change_db_description(
        &self,
        db_name: &str,
        description: &str,
    ) -> Result<String, DatabaseOperationError>
    {
        match self.db_manager().change_database_description(
            db_name,
            description
        ) {
            Ok(result) => {
                self.db_manager().log_event(&result);
                return Ok(result)
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
    ) -> Result<String, DatabaseOperationError>
    {
        match self.db_manager().create_collection(
            collection_name,
            db_name
        ) {
            Ok(result) => {
                self.db_manager().log_event(&result);
                return Ok(result)
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
    ) -> Result<String, DatabaseOperationError>
    {
        match self.db_manager().delete_collection(
            collection_name,
            db_name
        ) {
            Ok(result) => {
                self.db_manager().log_event(&result);
                return Ok(result)
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
        collection_name: &str,
        db_name: &str,
        data: Vec<InputDataField>,
    ) -> Result<String, DatabaseOperationError>
    {
        match self.db_manager().create_document(
            db_name,
            collection_name,
            data
        ) {
            Ok(result) => {
                self.db_manager().log_event(&result);
                return Ok(result)
            },
            Err(err) => {
                self.db_manager().log_error(&err.0);
                return Err(err)
            }
        }
    }
}

