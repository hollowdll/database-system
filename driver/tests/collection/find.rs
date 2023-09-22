use crate::common::{
    Config,
    create_test_document,
};
use driver::client::DatabaseClient;
use engine::storage::DB_FILE_EXTENSION;

#[test]
pub fn find_all_documents_success() {
    let config = Config::new();
    let client = DatabaseClient::build(config.db_dir.path());
    let db_name = "testdb123";
    let collection_name = "collection1";
    let file_path = config.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));
    let database = client.get_database(db_name).unwrap();

    assert_eq!(database.connection_string(), &file_path);
    assert!(file_path.is_file());

    let collection = database.get_collection(collection_name).unwrap();
    let document = create_test_document();
    let created_document = collection.insert_one(document).unwrap();
    let found_documents = collection.find_all().unwrap();

    assert_eq!(found_documents.len(), 1);
    assert_eq!(found_documents.first().unwrap().id.0, created_document.id.0);
    assert_eq!(found_documents.first().unwrap().data.len(), created_document.data.len());

    config.close_temp_dirs();
}

#[test]
pub fn find_one_document_by_id_success() {
    let config = Config::new();
    let client = DatabaseClient::build(config.db_dir.path());
    let db_name = "testdb123";
    let collection_name = "collection1";
    let file_path = config.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));
    let database = client.get_database(db_name).unwrap();

    assert_eq!(database.connection_string(), &file_path);
    assert!(file_path.is_file());

    let collection = database.get_collection(collection_name).unwrap();
    let document = create_test_document();
    let created_document = collection.insert_one(document).unwrap();
    let found_document = collection.find_one_by_id(created_document.id()).unwrap();

    let found_document = found_document.unwrap();
    assert_eq!(found_document.id.0, created_document.id.0);
    assert_eq!(found_document.data.len(), created_document.data.len());

    config.close_temp_dirs();
}