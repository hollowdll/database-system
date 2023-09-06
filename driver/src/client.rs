pub mod error;

use std::{
    io,
    path::{
        PathBuf,
        Path,
    }
};
use engine::{DriverEngine, config::Config};
use crate::{database::Database, collection::Collection};
use self::error::{
    DatabaseClientError,
    DatabaseClientErrorKind,
};

/// Database client communicates with the database engine.
/// 
/// Connect to databases with connection strings from this client.
pub struct DatabaseClient {
    engine: DriverEngine
}

impl DatabaseClient {
    /// Builds a new database client.
    pub fn build(config: &Config) -> DatabaseClient {
        DatabaseClient {
            engine: DriverEngine::build_logger_disabled(config),
        }
    }

    /// Creates and returns a database API.
    /// 
    /// This will fail if the connection string is not a valid path to the database file.
    pub fn get_database(&self, connection_string: &Path) -> Result<Database, DatabaseClientError> {
        let result = self.engine
            .storage_api()
            .find_database_by_file_path(connection_string);

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::ConnectionFailed,
                e.message));
        }

        if result.success {
            if let Some(db) = result.data {
                if let Some(db) = db {
                    return Ok(Database::new(connection_string));
                }
            }
        } else {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::Unexpected,
                "Failed to get database".to_string()));
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::DatabaseNotFound,
            format!("Tried to connect to '{}'", connection_string.display())));
    }

    /// Creates and returns a collection API.
    /// 
    /// This will fail if the collection doesn't exist.
    pub fn get_collection(&self, db: &Database, name: &str) -> Result<Collection, DatabaseClientError> {
        let result = self.engine
            .storage_api()
            .find_collection(name, db.connection_string());

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::GetCollection,
                e.message));
        }

        if result.success {
            if let Some(collection) = result.data {
                if let Some(collection) = collection {
                    return Ok(Collection::new(collection.name()));
                }
            }
        } else {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::Unexpected,
                "Failed to get collection".to_string()));
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::CollectionNotFound,
            format!("Tried to get collection with name '{}'", name)));
    }
}
