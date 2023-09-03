use engine::config::{
    load_config,
    CONFIG_FILE_NAME,
    DB_DIR_DEFAULT_NAME,
    LOGS_DIR_DEFAULT_NAME,
};
use tempfile::tempdir;
use std::{ffi::OsStr, fs};

#[test]
fn load_config_success() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join(CONFIG_FILE_NAME);
    let config = load_config(&file_path).unwrap();

    assert!(file_path.is_file());
    assert_eq!(config.db_dir_path().file_name(), Some(OsStr::new(DB_DIR_DEFAULT_NAME)));
    assert_eq!(config.logs_dir_path().file_name(), Some(OsStr::new(LOGS_DIR_DEFAULT_NAME)));

    let content = fs::read_to_string(file_path).unwrap();
    assert!(content.contains(DB_DIR_DEFAULT_NAME));
    assert!(content.contains(LOGS_DIR_DEFAULT_NAME));

    dir.close().unwrap();
}
