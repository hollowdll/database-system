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
fn find_all_documents_success() {
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
    let result = engine
        .storage_api()
        .create_document(&file_path, collection_name, data);
    assert!(result.success);

    let document = result.data.unwrap();
    let result = engine
        .storage_api()
        .find_all_documents(&file_path, collection_name);
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let documents = result.data.unwrap();
    assert_eq!(documents.len(), 1);
    assert_eq!(documents.get(0).unwrap().id(), document.id());
    assert_eq!(documents.get(0).unwrap().data().len(), document.data().len());

    config_settings.db_dir.close().unwrap();
    config_settings.logs_dir.close().unwrap();
}

#[test]
fn find_document_by_id_success() {
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
    let result = engine
        .storage_api()
        .create_document(&file_path, collection_name, data);
    assert!(result.success);

    let created_document = result.data.unwrap();
    let result = engine
        .storage_api()
        .find_document_by_id(created_document.id(), &file_path, collection_name);
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let document = result.data.unwrap().unwrap();
    assert_eq!(document.id(), created_document.id());
    assert_eq!(document.data().len(), created_document.data().len());

    config_settings.db_dir.close().unwrap();
    config_settings.logs_dir.close().unwrap();
}
