use std::path::{
    Path,
    PathBuf,
};
use engine::storage::error::{
    DatabaseOperationError,
    DatabaseOperationErrorKind,
};
use crate::collection::Collection;
use crate::client::{
    error::{
        DatabaseClientError,
        DatabaseClientErrorKind,
    },
    DatabaseClient,
};

/// Database API.
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
    pub fn get_collection<T>(&self, name: &str) -> Result<Collection<T>, DatabaseClientError> {
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
                            DatabaseClientErrorKind::CreateCollection,
                            e.message));
                    }
                }

                return Ok(Collection::new(self.client, self, name));
            }
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::Unexpected,
            "Failed to get collection".to_string()));
    }

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
            "Unexpected error creating collection".to_string()));
    }
}
