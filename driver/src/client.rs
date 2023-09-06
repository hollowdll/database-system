pub mod error;

use std::{
    io,
    path::{
        PathBuf,
        Path,
    }
};
use engine::{DriverEngine, config::Config};
use crate::database::Database;
use self::error::{
    DatabaseClientError,
    DatabaseClientErrorKind,
};

pub struct DatabaseClient {
    connection_string: PathBuf,
    engine: DriverEngine
}

impl DatabaseClient {
    /// Builds a new database client with a connection string.
    /// 
    /// The connection string is a file path to the database file.
    pub fn build(connection_string: &Path) -> DatabaseClient {
        let config = Config::default();

        DatabaseClient {
            connection_string: PathBuf::from(connection_string),
            engine: DriverEngine::build_logger_disabled(&config),
        }
    }

    /// Gets the database from this database client.
    /// 
    /// Returns the database if the connection string is a valid path to the database file.
    pub fn get_database(&self) -> Result<Database, DatabaseClientError> {
        let result = self.engine
            .storage_api()
            .find_database_by_file_path(&self.connection_string);

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::ConnectionFailed,
                e.message));
        }

        if result.success {
            if let Some(db) = result.data {
                if let Some(db) = db {
                    return Ok(Database::new(db.name()));
                }
            }
        } else {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::Unexpected,
                "Failed to get database".to_string()));
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::NotFound,
            format!("Tried to connect to {}", self.connection_string.display())));
    }
}
