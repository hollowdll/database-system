use std::path::{
    Path,
    PathBuf,
};
use crate::collection::Collection;
use crate::client::DatabaseClient;

/// Database API.
/// 
/// The connection string is a file path to the database.
pub struct Database {
    pub connection_string: PathBuf,
}

impl Database {
    pub fn connection_string(&self) -> &Path {
        &self.connection_string
    }
}

impl Database {
    pub fn new(connection_string: &Path) -> Database {
        Database {
            connection_string: PathBuf::from(connection_string),
        }
    }
}
