use crate::common::Config;
use driver::client::DatabaseClient;
use engine::storage::DB_FILE_EXTENSION;

#[test]
pub fn get_collection_success() {
    let config = Config::new();
    let client = DatabaseClient::build(config.db_dir.path());
    let db_name = "testdb123";
    let collection_name = "testcollection";
    let file_path = config.db_dir.path().join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));
    let database = client.get_database(db_name).unwrap();

    assert_eq!(database.connection_string(), &file_path);
    assert!(file_path.is_file());

    let collection = database.get_collection(collection_name).unwrap();
    assert_eq!(collection.name(), collection_name);

    let result = client.engine
        .storage_api()
        .find_collection(collection_name, &file_path);
    assert!(result.error.is_none());
    assert!(result.success);
    assert!(result.data.is_some());
    
    let collection = result.data.unwrap().unwrap();
    assert_eq!(collection.name(), collection_name);

    config.close_temp_dirs();
}