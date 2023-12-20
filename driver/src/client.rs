pub mod error;

use std::{
    io,
    path::{
        PathBuf,
        Path,
    },
    env::current_exe,
};
use engine::{
    DriverEngine,
    config::Config,
    storage::{
        error::{
            DatabaseOperationError,
            DatabaseOperationErrorKind,
        },
        DB_FILE_EXTENSION,
    },
};
use crate::{
    database::Database,
    collection::Collection
};
use self::error::{
    DatabaseClientError,
    DatabaseClientErrorKind,
    UNEXPECTED_ERROR,
};
use tempfile::tempdir;

/// Connect to databases using this. Multiple databases can be connected
/// using the same database client.
/// 
/// Database client communicates with the database engine and handles
/// transforming input and output data.
pub struct DatabaseClient {
    pub engine: DriverEngine,
    pub db_dir: PathBuf,
}

impl DatabaseClient {
    /// Builds a new database client using the database directory.
    /// 
    /// Database directory is where databases will be created to and accessed from. 
    pub fn build(db_dir: &Path) -> DatabaseClient {
        let mut config = Config::default();
        config.db_dir_path = PathBuf::from(db_dir);

        DatabaseClient {
            engine: DriverEngine::build_logger_disabled(&config),
            db_dir: PathBuf::from(db_dir),
        }
    }

    /// Gets a database using the database name.
    /// 
    /// Creates the database if it doesn't exist.
    /// Databases will be created to the database directory.
    pub fn get_database(&self, name: &str) -> Result<Database, DatabaseClientError> {
        let file_path = self.db_dir.join(&format!("{}.{}", name, DB_FILE_EXTENSION));
        
        let result = self.engine
            .storage_api()
            .find_database(name);

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::GetDatabase,
                e.message));
        }

        if result.success {
            if let Some(db) = result.data {
                if let None = db {
                    if let Err(e) = self.create_database(name) {
                        return Err(DatabaseClientError::new(
                            DatabaseClientErrorKind::GetDatabase,
                            format!("Cannot create database: {}", e.message)));
                    }
                }

                return Ok(Database::new(&self, &file_path));
            }
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::GetDatabase,
            UNEXPECTED_ERROR.to_string()));
    }
}

impl DatabaseClient {
    /// Creates a database with the given name.
    fn create_database(&self, name: &str) -> Result<(), DatabaseOperationError> {
        let result = self.engine
            .storage_api()
            .create_database_to_db_dir(name);

        if let Some(e) = result.error {
            return Err(e);
        }

        if result.success {
            return Ok(());
        }

        return Err(DatabaseOperationError::new(
            DatabaseOperationErrorKind::CreateDatabase,
            UNEXPECTED_ERROR.to_string()));
    }
}
