use std::path::{
    Path,
    PathBuf,
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
    pub connection_string: PathBuf,
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

    /// Creates and returns a collection API.
    /// 
    /// This will fail if the collection doesn't exist.
    pub fn get_collection<T>(&self, name: &str) -> Result<Collection<T>, DatabaseClientError> {
        // TODO
        // change this method so it tries to find collection. if it does not exist, try to create it

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
                if let Some(collection) = collection {
                    return Ok(Collection::new(self.client, self, collection.name()));
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
