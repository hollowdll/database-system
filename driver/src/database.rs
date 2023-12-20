use std::path::{
    Path,
    PathBuf,
};
use engine::storage::{
    database::DatabaseDto,
    error::{
        DatabaseOperationError,
        DatabaseOperationErrorKind,
    },
};
use crate::collection::Collection;
use crate::client::{
    error::{
        DatabaseClientError,
        DatabaseClientErrorKind,
        UNEXPECTED_ERROR,
    },
    DatabaseClient,
};

/// Database API.
/// 
/// This provides methods to work with a database.
/// 
/// The connection string is a file path to the database.
pub struct Database<'a> {
    client: &'a DatabaseClient,
    connection_string: PathBuf,
}

impl<'a> Database<'a> {
    pub fn connection_string(&self) -> &Path {
        &self.connection_string
    }
}

impl<'a> Database<'a> {
    pub fn new(client: &'a DatabaseClient, connection_string: &Path) -> Database<'a> {
        Database {
            client,
            connection_string: PathBuf::from(connection_string),
        }
    }

    /// Gets a collection from this database using the collection name.
    /// 
    /// Creates the collection if it doesn't exist.
    pub fn get_collection(&self, name: &str) -> Result<Collection, DatabaseClientError> {
        let result = self.client.engine
            .storage_api()
            .find_collection(name, self.connection_string());

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::GetCollection,
                e.message));
        }

        if result.success {
            if let Some(collection) = result.data {
                if let None = collection {
                    if let Err(e) = self.create_collection(name) {
                        return Err(DatabaseClientError::new(
                            DatabaseClientErrorKind::GetCollection,
                            format!("Cannot create collection: {}", e.message)));
                    }
                }

                return Ok(Collection::new(self.client, self, name));
            }
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::GetCollection,
            UNEXPECTED_ERROR.to_string()));
    }

    /// Gets this database's metadata.
    pub fn get_metadata(&self) -> Result<DatabaseDto, DatabaseClientError> {
        let result = self.client.engine
            .storage_api()
            .find_database_by_file_path(self.connection_string());

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::GetDatabase,
                e.message));
        }

        if result.success {
            if let Some(db) = result.data {
                if let Some(db) = db {
                    return Ok(db);
                } else {
                    return Err(DatabaseClientError::new(
                        DatabaseClientErrorKind::GetDatabase,
                        "Database not found".to_string()));
                }
            }
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::GetDatabase,
            UNEXPECTED_ERROR.to_string()));
    }
}

impl<'a> Database<'a> {
    /// Creates a collection to this database.
    fn create_collection(&self, name: &str) -> Result<(), DatabaseOperationError> {
        let result = self.client.engine
            .storage_api()
            .create_collection(name, self.connection_string());

        if let Some(e) = result.error {
            return Err(e);
        }

        if result.success {
            return Ok(());
        }

        return Err(DatabaseOperationError::new(
            DatabaseOperationErrorKind::CreateCollection,
            UNEXPECTED_ERROR.to_string()));
    }
}
