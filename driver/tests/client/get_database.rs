use crate::common::Config;
use driver::client::DatabaseClient;
use engine::storage::DB_FILE_EXTENSION;

#[test]
pub fn get_database_success() {
    let config = Config::new();
    let client = DatabaseClient::build(config.db_dir.path());
    let db_name = "testdb123";
    let file_path = config.db_dir.path().join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));

    assert_eq!(file_path.is_file(), false);
    let database = client.get_database(db_name).unwrap();

    assert_eq!(database.connection_string(), &file_path);
    assert!(file_path.is_file());

    config.close_temp_dirs();
}

#[test]
pub fn get_and_find_database_success() {
    let config = Config::new();
    let client = DatabaseClient::build(config.db_dir.path());
    let db_name = "testdb123";
    let file_path = config.db_dir.path().join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));

    assert_eq!(file_path.is_file(), false);
    let database = client.get_database(db_name).unwrap();

    assert_eq!(database.connection_string(), &file_path);
    assert!(file_path.is_file());

    let result = client.engine
        .storage_api()
        .find_database_by_file_path(&file_path);

    assert!(result.error.is_none());
    assert!(result.success);

    let found_db = result.data.unwrap();
    assert!(found_db.is_some());
    assert_eq!(found_db.unwrap().file_path(), &file_path);

    config.close_temp_dirs();
}