use std::{
    io,
    path::{
        PathBuf,
        Path,
    }
};
use engine::{DriverEngine, config::Config};

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
    /// Returns the database if the connection string is valid.
    pub fn get_database(&self) -> io::Result<Database> {
        let result = self.engine
            .storage_api()
            .find_database_by_file_path(&self.connection_string);

        if result.success {
            if let Some(db) = result.data {
                if let Some(db) = db {
                    return Ok(Database::new(db.name()));
                }
            }
        } else {
            if let Some(e) = result.error {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    e.message));
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Unexpected error"));
            }
        }

        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Database was not found with this connection string"));
    }
}

pub struct Database {
    pub name: String,
}

impl Database {
    pub fn new(name: &str) -> Database {
        Database {
            name: name.to_string()
        }
    }
}
