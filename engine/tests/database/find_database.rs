use engine::{
    config::Config,
    Logger,
    Engine,
    storage::DB_FILE_EXTENSION,
};
use tempfile::tempdir;

#[test]
fn find_all_databases_success() {
    let db_dir = tempdir().unwrap();
    let logs_dir = tempdir().unwrap();
    let config = Config::new(
        db_dir.path(),
        logs_dir.path()
    );
    let logger = Logger::build(&config);
    let engine = Engine::build(&config, &logger);
    let db_name = "test";
    let file_path = db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));

    let result = engine
        .storage_api()
        .create_database_by_file_path(db_name, &file_path);
    assert!(result.success);

    let result = engine
        .storage_api()
        .find_all_databases();
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let databases = result.data.unwrap();
    let first_db = databases.get(0).unwrap();
    assert_eq!(databases.len(), 1);
    assert_eq!(first_db.name(), db_name);

    db_dir.close().unwrap();
    logs_dir.close().unwrap();
}

#[test]
fn find_database_success() {
    let db_dir = tempdir().unwrap();
    let logs_dir = tempdir().unwrap();
    let config = Config::new(
        db_dir.path(),
        logs_dir.path()
    );
    let logger = Logger::build(&config);
    let engine = Engine::build(&config, &logger);
    let db_name = "test";
    let file_path = db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));

    let result = engine
        .storage_api()
        .create_database_by_file_path(db_name, &file_path);
    assert!(result.success);

    let result = engine
        .storage_api()
        .find_database(db_name);
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let db = result.data.unwrap();
    assert!(db.is_some());
    assert_eq!(db.unwrap().name(), db_name);

    db_dir.close().unwrap();
    logs_dir.close().unwrap();
}

#[test]
fn find_database_by_file_path_success() {
    let db_dir = tempdir().unwrap();
    let logs_dir = tempdir().unwrap();
    let config = Config::new(
        db_dir.path(),
        logs_dir.path()
    );
    let logger = Logger::build(&config);
    let engine = Engine::build(&config, &logger);
    let db_name = "test";
    let file_path = db_dir
        .path()
        .join(&format!("{}.{}", db_name, DB_FILE_EXTENSION));

    let result = engine
        .storage_api()
        .create_database_by_file_path(db_name, &file_path);
    assert!(result.success);

    let result = engine
        .storage_api()
        .find_database_by_file_path(&file_path);
    assert!(result.success);
    assert!(result.data.is_some());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let db = result.data.unwrap();
    assert!(db.is_some());
    assert_eq!(db.unwrap().name(), db_name);

    db_dir.close().unwrap();
    logs_dir.close().unwrap();
}
