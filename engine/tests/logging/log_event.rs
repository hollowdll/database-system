use engine::{
    Logger,
    logging::EVENTS_LOG,
};
use crate::common::ConfigSettings;
use std::fs::read_to_string;

#[test]
fn log_event_success() {
    let config_settings = ConfigSettings::new();
    let logger = Logger::build(&config_settings.config.logs_dir_path());
    let file_path = config_settings.config
        .logs_dir_path()
        .join(EVENTS_LOG);
    let log_content = "This log is a test";
    assert_eq!(file_path.is_file(), false);

    logger.log_event(log_content).unwrap();
    assert!(file_path.is_file());
    assert!(read_to_string(&file_path).unwrap().contains(log_content));

    config_settings.close_temp_dirs();
}
