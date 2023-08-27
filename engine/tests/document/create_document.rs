use engine::{
    Logger,
    Engine,
    storage::DB_FILE_EXTENSION,
};
use crate::common::{
    ConfigSettings,
    create_document_input_data,
};

#[test]
fn create_document_success() {
    let config_settings = ConfigSettings::new();
    let logger = Logger::build(&config_settings.config);
    let engine = Engine::build(&config_settings.config, &logger);
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

    let data = create_document_input_data();
    let data_field_count = data.len();
    let result = engine
        .storage_api()
        .create_document(&file_path, collection_name, data);
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let document = result.data.unwrap();
    assert_eq!(document.id(), &1);
    assert_eq!(document.data().len(), data_field_count);

    let result = engine
        .storage_api()
        .find_document_by_id(document.id(), &file_path, collection_name);
    assert!(result.success);
    assert!(result.data.unwrap().is_some());

    config_settings.db_dir.close().unwrap();
    config_settings.logs_dir.close().unwrap();
}
