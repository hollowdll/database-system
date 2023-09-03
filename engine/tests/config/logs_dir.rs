use engine::{
    Logger,
    Engine,
};
use tempfile::tempdir;
use crate::common::ConfigSettings;
use std::fs;

#[test]
fn set_logs_dir_path_success() {
    let config_settings = ConfigSettings::new();
    let logger = Logger::build(&config_settings.config);
    let engine = Engine::build(&config_settings.config, &logger);
    let new_dir = tempdir().unwrap();
    let config_file = fs::File::create(config_settings.config.config_file_path()).unwrap();

    let result = engine
        .config_api()
        .set_logs_dir_path(new_dir.path());
    assert!(result.success);
    assert!(result.error.is_none());
    assert!(result.log_error.is_none());

    let content = fs::read_to_string(config_settings.config.config_file_path()).unwrap();
    assert!(content.contains(&format!("{:?}", new_dir.path())));

    drop(config_file);
    config_settings.config_dir.close().unwrap();
    config_settings.db_dir.close().unwrap();
    config_settings.logs_dir.close().unwrap();
    new_dir.close().unwrap();
}