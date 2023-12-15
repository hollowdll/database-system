use engine::{
    Engine,
    storage::DB_FILE_EXTENSION,
};
use crate::common::{
    ConfigSettings,
    create_document_input_data,
};

#[test]
fn delete_document_success() {
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

    let data = create_document_input_data();
    let result = engine
        .storage_api()
        .create_document(&file_path, collection_name, data);
    assert!(result.success);

    let document = result.data.unwrap();
    let result = engine
        .storage_api()
        .find_document_by_id(document.id(), &file_path, collection_name);
    assert!(result.success);
    assert!(result.data.unwrap().is_some());

    let result = engine
        .storage_api()
        .delete_document(&file_path, document.id(), collection_name);
    assert!(result.success);
    assert!(result.data.is_none());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let result = engine
        .storage_api()
        .find_document_by_id(document.id(), &file_path, collection_name);
    assert!(result.success);
    assert!(result.data.unwrap().is_none());

    config_settings.close_temp_dirs();
}

#[test]
fn delete_all_documents_success() {
    let config_settings = ConfigSettings::new();
    let engine = Engine::build(&config_settings.config);
    let db_name = "test";
    let collection_name = "people";
    let file_path = config_settings.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));

    engine.storage_api()
        .create_database_by_file_path(db_name, &file_path);
    
    engine.storage_api()
        .create_collection(collection_name, &file_path);

    for _ in 0..3 {
        let data = create_document_input_data();
        engine.storage_api()
            .create_document(&file_path, collection_name, data);
    }

    let result = engine.storage_api()
        .find_all_documents(&file_path, collection_name);
    let document_count = result.data.unwrap().len();
    assert!(document_count > 0);

    let result = engine.storage_api()
        .delete_all_documents(&file_path, collection_name);
    assert!(result.success);
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());
    assert_eq!(result.data.unwrap(), document_count);

    let result = engine.storage_api()
        .find_all_documents(&file_path, collection_name);
    assert_eq!(result.data.unwrap().len(), 0);

    config_settings.close_temp_dirs();
}
