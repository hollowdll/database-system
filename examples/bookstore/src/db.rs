use driver::{
    client::{
        DatabaseClient,
        error::DatabaseClientError,
    },
    database::Database,
};
use std::env::current_exe;

/// Creates the database client.
pub fn create_db_client() -> DatabaseClient {
    let mut db_dir = current_exe().expect("Cannot get database directory");
    db_dir.pop();

    DatabaseClient::build(&db_dir)
}

/// Gets the bookstore database that this program uses.
pub fn get_bookstore_db(client: &DatabaseClient) -> Result<Database, DatabaseClientError> {
    Ok(client.get_database("bookdb")?)
}