use crate::common::{
    Config,
    create_test_document,
};
use driver::{
    client::DatabaseClient,
    document::{
        DocumentModel,
        DataType, DocumentQuery
    }
};
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
    let found_documents = collection.find_all(&None).unwrap();

    assert_eq!(found_documents.len(), 1);
    assert_eq!(found_documents.first().unwrap().id.0, created_document.id.0);
    assert_eq!(found_documents.first().unwrap().data.len(), created_document.data.len());

    config.close_temp_dirs();
}

#[test]
pub fn find_documents_success() {
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
    let max_document_count = 3;
    let expected_document_count = max_document_count - 1;
    for i in 1..=max_document_count {
        let mut document = DocumentModel::new();
        if i == 1 {
            document.data.insert("age".to_string(), DataType::Int32(40));
        } else {
            document.data.insert("age".to_string(), DataType::Int32(35));
        }

        assert!(collection.insert_one(document).is_ok());
    }

    let mut query = DocumentQuery::new();
    query.data.insert("age".to_string(), DataType::Int32(35));
    let found_documents = collection.find_many(&query, &None).unwrap();

    assert!(found_documents.len() > 0);
    assert_eq!(found_documents.len(), expected_document_count);

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