use crate::common::{
    Config,
    create_test_document,
};
use driver::{
    client::DatabaseClient,
    document::{
        DataType,
        DocumentModel
    },
};
use engine::storage::DB_FILE_EXTENSION;

#[test]
pub fn replace_one_document_by_id_success() {
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
    let found_document = collection.find_one_by_id(created_document.id()).unwrap().unwrap();
    let original_data = found_document.data.clone();
    assert_eq!(found_document.id.0, created_document.id.0);

    let mut new_document = DocumentModel::new();
    new_document.data.insert("email".to_string(), DataType::Text("example@example.com".to_string()));
    collection.replace_one_by_id(found_document.id(), new_document).unwrap();
    
    let found_document = collection.find_one_by_id(found_document.id()).unwrap().unwrap();
    assert_eq!(found_document.id.0, created_document.id.0);
    assert_eq!(found_document.data.len(), 1);
    assert_ne!(found_document.data.len(), original_data.len());
    assert!(found_document.data.get("email").is_some());

    config.close_temp_dirs();
}