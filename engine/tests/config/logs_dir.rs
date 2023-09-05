use engine::{
    Engine,
    config::load_config,
};
use tempfile::tempdir;
use crate::common::ConfigSettings;
use std::fs;

#[test]
fn set_logs_dir_path_success() {
    let config_settings = ConfigSettings::new();
    let engine = Engine::build(&config_settings.config);
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
    config_settings.close_temp_dirs();
    new_dir.close().unwrap();
}

#[test]
fn set_logs_dir_path_and_load_config() {
    let config_settings = ConfigSettings::new();
    let engine = Engine::build(&config_settings.config);
    let new_dir = tempdir().unwrap();
    let config_file = fs::File::create(config_settings.config.config_file_path()).unwrap();

    let result = engine
        .config_api()
        .set_logs_dir_path(new_dir.path());
    assert!(result.success);

    let config = load_config(config_settings.config.config_file_path()).unwrap();
    assert_eq!(config.db_dir_path(), config_settings.db_dir.path());
    assert_eq!(config.logs_dir_path(), new_dir.path());
    
    drop(config_file);
    config_settings.close_temp_dirs();
    new_dir.close().unwrap();
}
