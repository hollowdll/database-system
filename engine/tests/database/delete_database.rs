use engine::{
    config::Config,
    Logger,
    Engine,
    storage::DB_FILE_EXTENSION,
};
use tempfile::tempdir;

#[test]
fn delete_database_success() {
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
    assert!(result.data.unwrap().is_some());

    let result = engine
        .storage_api()
        .delete_database(&file_path);
    assert!(result.success);
    assert!(result.data.is_none());
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let result = engine
        .storage_api()
        .find_database_by_file_path(&file_path);
    assert!(result.success);
    assert!(result.data.unwrap().is_none());

    db_dir.close().unwrap();
    logs_dir.close().unwrap();
}
