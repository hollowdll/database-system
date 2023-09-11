pub mod error;

use std::{
    io,
    path::{
        PathBuf,
        Path,
    }, env::current_exe
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
};

/// Connect to databases using this. Multiple databases can be connected
/// using the same database client.
/// 
/// Database client communicates with the database engine and handles
/// transforming input and output data.
pub struct DatabaseClient {
    pub engine: DriverEngine
}

impl DatabaseClient {
    /// Builds a new database client.
    pub fn build() -> DatabaseClient {
        let config = Config::default();

        DatabaseClient {
            engine: DriverEngine::build_logger_disabled(&config),
        }
    }

    /// Gets a database using the database name.
    /// 
    /// Creates the database if it doesn't exist.
    /// Databases will be created to the crate root.
    pub fn get_database(&self, name: &str) -> Result<Database, DatabaseClientError> {
        let mut dir_path = match current_exe() {
            Ok(dir_path) => dir_path,
            Err(e) => return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::GetDatabase,
                e.to_string())),
        };
        dir_path.pop();
        let file_path = dir_path.join(&format!("{}.{}", name, DB_FILE_EXTENSION));

        let result = self.engine
            .storage_api()
            .find_database_by_file_path(&file_path);

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::ConnectionFailed,
                e.message));
        }

        if result.success {
            if let Some(db) = result.data {
                if let None = db {
                    if let Err(e) = self.create_database(name, &file_path) {
                        return Err(DatabaseClientError::new(
                            DatabaseClientErrorKind::CreateDatabase,
                            e.message));
                    }
                }

                return Ok(Database::new(&self, &file_path));
            }
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::Unexpected,
            "Failed to get database".to_string()));
    }
}

impl DatabaseClient {
    /// Creates a database with the given name and file path.
    fn create_database(&self, name: &str, file_path: &Path) -> Result<(), DatabaseOperationError> {
        let result = self.engine
            .storage_api()
            .create_database_by_file_path(name, file_path);

        if let Some(e) = result.error {
            return Err(e);
        }

        if result.success {
            return Ok(());
        }

        return Err(DatabaseOperationError::new(
            DatabaseOperationErrorKind::CreateDatabase,
            "Unexpected error creating database".to_string()));
    }
}
