use crate::common::Config;
use driver::client::DatabaseClient;
use engine::storage::DB_FILE_EXTENSION;

#[test]
pub fn get_database_metadata_success() {
    let config = Config::new();
    let client = DatabaseClient::build(config.db_dir.path());
    let db_name = "testdb123";
    let file_path = config.db_dir.path().join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));
    let database = client.get_database(db_name).unwrap();

    assert_eq!(database.connection_string(), &file_path);
    assert!(file_path.is_file());

    let metadata = database.get_metadata().unwrap();
    assert_eq!(metadata.name(), db_name);
    assert_eq!(metadata.file_path(), &file_path);

    config.close_temp_dirs();
}