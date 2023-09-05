use engine::{
    Engine,
    storage::DB_FILE_EXTENSION,
};
use crate::common::ConfigSettings;

#[test]
fn find_all_collections_success() {
    let config_settings = ConfigSettings::new();
    let engine = Engine::build(&config_settings.config);
    let db_name = "test";
    let collection_name = "people";
    let file_path = config_settings.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));

    let result = engine
        .storage_api()
        .create_database_by_file_path(db_name, &file_path);
    assert!(result.success);

    let result = engine
        .storage_api()
        .create_collection(collection_name, &file_path);
    assert!(result.success);

    let result = engine
        .storage_api()
        .find_all_collections(&file_path);
    assert!(result.success);
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let collections = result.data.unwrap();
    assert_eq!(collections.len(), 1);

    config_settings.close_temp_dirs();
}

#[test]
fn find_collection_success() {
    let config_settings = ConfigSettings::new();
    let engine = Engine::build(&config_settings.config);
    let db_name = "test";
    let collection_name = "people";
    let file_path = config_settings.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));

    let result = engine
        .storage_api()
        .create_database_by_file_path(db_name, &file_path);
    assert!(result.success);

    let result = engine
        .storage_api()
        .create_collection(collection_name, &file_path);
    assert!(result.success);

    let result = engine
        .storage_api()
        .find_collection(collection_name, &file_path);
    assert!(result.success);
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let collection = result.data.unwrap();
    assert_eq!(collection.unwrap().name(), collection_name);

    config_settings.close_temp_dirs();
}
