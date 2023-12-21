use engine::{
    Engine,
    storage::DB_FILE_EXTENSION,
    DocumentInputDataField,
};
use crate::common::{
    ConfigSettings,
    create_document_input_data,
};

#[test]
fn find_all_documents_success() {
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
        .find_all_documents(&file_path, collection_name, None);
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let documents = result.data.unwrap();
    assert_eq!(documents.len(), 1);
    assert_eq!(documents.get(0).unwrap().id(), document.id());
    assert_eq!(documents.get(0).unwrap().data().len(), document.data().len());

    config_settings.close_temp_dirs();
}

#[test]
fn find_all_documents_with_limit_success() {
    let config_settings = ConfigSettings::new();
    let engine = Engine::build(&config_settings.config);
    let db_name = "test";
    let collection_name = "people";
    let file_path = config_settings.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));
    let limit_under = 3;
    let document_count = limit_under + 1;
    let limit_over = document_count + 1;

    engine.storage_api()
        .create_database_by_file_path(db_name, &file_path);
    
    engine.storage_api()
        .create_collection(collection_name, &file_path);

    for _ in 1..=document_count {
        let data = create_document_input_data();
        engine.storage_api()
            .create_document(&file_path, collection_name, data);
    }

    let result = engine
        .storage_api()
        .find_all_documents(&file_path, collection_name, Some(limit_under));
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());
    assert_eq!(result.data.unwrap().len(), limit_under);

    let result = engine
        .storage_api()
        .find_all_documents(&file_path, collection_name, Some(limit_over));
    assert_eq!(result.data.unwrap().len(), document_count);

    config_settings.close_temp_dirs();
}

#[test]
fn find_document_by_id_success() {
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

    config_settings.close_temp_dirs();
}

#[test]
fn find_documents_success() {
    let config_settings = ConfigSettings::new();
    let engine = Engine::build(&config_settings.config);
    let db_name = "test";
    let collection_name = "people";
    let file_path = config_settings.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));
    let max_documents = 3;
    let expected_document_count = max_documents - 1;

    engine.storage_api()
        .create_database_by_file_path(db_name, &file_path);
    
    engine.storage_api()
        .create_collection(collection_name, &file_path);

    for i in 1..=max_documents {
        let mut data = Vec::new();
        if i == 1 {
            data.push(DocumentInputDataField::new("age", "Int32", "30"));
        } else {
            data.push(DocumentInputDataField::new("age", "Int32", "35"));
        }

        engine.storage_api()
            .create_document(&file_path, collection_name, data);
    }

    let mut query = Vec::new();
    query.push(DocumentInputDataField::new("age", "Int32", "35"));
    
    let result = engine
        .storage_api()
        .find_documents(&file_path, collection_name, &query, None);
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let documents = result.data.unwrap();
    assert!(documents.len() > 0);
    assert_eq!(documents.len(), expected_document_count);

    config_settings.close_temp_dirs();
}

#[test]
fn find_documents_with_limit_success() {
    let config_settings = ConfigSettings::new();
    let engine = Engine::build(&config_settings.config);
    let db_name = "test";
    let collection_name = "people";
    let file_path = config_settings.db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));
    let max_documents = 3;
    let limit_under = max_documents - 1;
    let limit_over = max_documents + 1;

    engine.storage_api()
        .create_database_by_file_path(db_name, &file_path);
    
    engine.storage_api()
        .create_collection(collection_name, &file_path);

    for _ in 1..=max_documents {
        let mut data = Vec::new();
        data.push(DocumentInputDataField::new("age", "Int32", "35"));

        engine.storage_api()
            .create_document(&file_path, collection_name, data);
    }

    let mut query = Vec::new();
    query.push(DocumentInputDataField::new("age", "Int32", "35"));
    
    let result = engine
        .storage_api()
        .find_documents(&file_path, collection_name, &query, Some(limit_under));
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());
    assert_eq!(result.data.unwrap().len(), limit_under);

    let result = engine
        .storage_api()
        .find_documents(&file_path, collection_name, &query, Some(limit_over));
    assert_eq!(result.data.unwrap().len(), max_documents);

    config_settings.close_temp_dirs();
}
