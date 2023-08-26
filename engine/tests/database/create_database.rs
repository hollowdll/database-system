use engine::{
    Logger,
    Engine,
    storage::DB_FILE_EXTENSION,
};
use crate::common::ConfigSettings;

#[test]
fn create_database_to_db_dir_success() {
    let config_settings = ConfigSettings::new();
    let logger = Logger::build(&config_settings.config);
    let engine = Engine::build(&config_settings.config, &logger);
    let db_name = "test";

    let result = engine
        .storage_api()
        .create_database_to_db_dir(db_name);
    assert!(result.success);
    assert!(result.data.is_none());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let result = engine
        .storage_api()
        .find_database(db_name);
    assert!(result.success);
    assert!(result.data.is_some());

    let db = result.data.unwrap();
    assert!(db.is_some());
    assert_eq!(db.unwrap().name(), db_name);

    config_settings.db_dir.close().unwrap();
    config_settings.logs_dir.close().unwrap();
}

#[test]
fn create_database_by_file_path_success() {
    let config_settings = ConfigSettings::new();
    let logger = Logger::build(&config_settings.config);
    let engine = Engine::build(&config_settings.config, &logger);
    let db_name = "test";
    let file_path = config_settings.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));

    let result = engine
        .storage_api()
        .create_database_by_file_path(db_name, &file_path);
    assert!(result.success);
    assert!(result.data.is_none());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let result = engine
        .storage_api()
        .find_database_by_file_path(&file_path);
    assert!(result.success);
    assert!(result.data.is_some());

    let db = result.data.unwrap();
    assert!(db.is_some());
    assert_eq!(db.unwrap().name(), db_name);

    config_settings.db_dir.close().unwrap();
    config_settings.logs_dir.close().unwrap();
}
