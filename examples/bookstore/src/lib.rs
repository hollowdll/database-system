pub mod book;

use driver::{
    client::{
        DatabaseClient,
        error::DatabaseClientError,
    },
    database::Database,
    document::DocumentModel,
};
use std::env::current_exe;

pub fn create_db_client() -> DatabaseClient {
    let mut db_dir = current_exe().expect("Cannot get database directory");
    db_dir.pop();

    DatabaseClient::build(&db_dir)
}

pub fn get_bookstore_db(client: &DatabaseClient) -> Result<Database, DatabaseClientError> {
    Ok(client.get_database("bookdb")?)
}

/// Displays a document listing its data.
pub fn display_document(document: &DocumentModel) {
    println!("_id: {}", document.id());

    for (key, value) in &document.data {
        println!("{}: {}", key, value);
    }
}

/// Displays a list of documents and their data.
pub fn display_document_list(documents: &Vec<DocumentModel>) {
    for document in documents {
        println!("{{");
        println!("  _id: {}", document.id);

        for (key, value) in &document.data {
            println!("  {}: {}", key, value);
        }

        println!("}}");
    }
}