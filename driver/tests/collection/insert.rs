use crate::common::{
    Config,
    create_test_document,
};
use driver::client::DatabaseClient;
use engine::storage::DB_FILE_EXTENSION;

#[test]
pub fn insert_document_success() {
    let config = Config::new();
    let client = DatabaseClient::build(config.db_dir.path());
    let db_name = "testdb123";
    let collection_name = "people";
    let file_path = config.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));
    let database = client.get_database(db_name).unwrap();

    assert_eq!(database.connection_string(), &file_path);
    assert!(file_path.is_file());

    let collection = database.get_collection(collection_name).unwrap();
    assert_eq!(collection.name(), collection_name);

    let document = create_test_document();
    let document_id = document.id.0;
    let field_count = document.data.len();
    let created_document = collection.insert_one(document).unwrap();

    assert_ne!(created_document.id.0, document_id);
    assert_eq!(created_document.id.0, 1);
    assert_eq!(created_document.data.len(), field_count);

    config.close_temp_dirs();
}