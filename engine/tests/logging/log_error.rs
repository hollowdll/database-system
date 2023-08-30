use engine::{
    Logger,
    logging::{
        ERRORS_LOG,
        ErrorLogType,
    },
};
use crate::common::ConfigSettings;
use std::fs::read_to_string;

#[test]
fn log_error_success() {
    let config_settings = ConfigSettings::new();
    let logger = Logger::build(&config_settings.config);
    let file_path = config_settings.config
        .logs_dir_path()
        .join(ERRORS_LOG);
    let log_content = "This error log is a test";
    assert_eq!(file_path.is_file(), false);

    logger.log_error(ErrorLogType::Error, log_content).unwrap();
    assert!(file_path.is_file());
    assert!(read_to_string(&file_path).unwrap().contains(log_content));

    config_settings.db_dir.close().unwrap();
    config_settings.logs_dir.close().unwrap();
}
