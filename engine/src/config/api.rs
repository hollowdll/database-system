// Engine configuration API

use std::{
    path::Path,
    io,
};
use crate::{
    Logger,
    logging::{
        ErrorLogType,
        error::LogError,
    },
};
use super::config_manager::ConfigManager;

/// Result for calls that request configs.
/// 
/// Config API methods return this.
pub struct ConfigRequestResult {
    /// Whether result is successful.
    pub success: bool,

    /// Possible error.
    pub error: Option<io::Error>,

    /// Possible error that occurred during logging.
    pub log_error: Option<LogError>,
}

/// Returns successful config API response.
fn request_success(log_result: Result<(), LogError>) -> ConfigRequestResult {
    ConfigRequestResult {
        success: true,
        error: None,
        log_error: match log_result {
            Ok(()) => None,
            Err(e) => Some(e),
        },
    }
}

/// Returns failed config API response.
fn request_fail(
    error: io::Error,
    log_result: Result<(), LogError>
) -> ConfigRequestResult {
    ConfigRequestResult {
        success: false,
        error: Some(error),
        log_error: match log_result {
            Ok(()) => None,
            Err(e) => Some(e),
        },
    }
}

/// Engine configuration API.
/// 
/// Provides methods to change engine configurations.
/// 
/// Logs errors and events before forwarding results to the caller.
pub struct ConfigApi {
    config_manager: ConfigManager,
    logger: Logger,
}

impl ConfigApi {
    /// Builds config API.
    pub fn build(
        config_manager: ConfigManager,
        logger: Logger,
    ) -> ConfigApi
    {
        ConfigApi {
            config_manager,
            logger,
        }
    }
}

impl ConfigApi {
    /// Requests `ConfigManager` to set database directory path config.
    pub fn set_db_dir_path(&self, path: &Path) -> ConfigRequestResult {
        match self.config_manager.set_db_dir_path(path) {
            Ok(()) => {
                let content = format!("Changed database directory path configuration to '{}'", path.display());
                let result = self.logger.log_event(&content);

                return request_success(result);
            },
            Err(err) => {
                let content = format!("Failed to change database directory path configuration: {}", err);
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);
                
                return request_fail(err, result);
            },
        };
    }

    /// Requests `ConfigManager` to set logs directory path config.
    pub fn set_logs_dir_path(&self, path: &Path) -> ConfigRequestResult {
        match self.config_manager.set_logs_dir_path(path) {
            Ok(()) => {
                let content = format!("Changed logs directory path configuration to '{}'", path.display());
                let result = self.logger.log_event(&content);

                return request_success(result);
            },
            Err(err) => {
                let content = format!("Failed to change logs directory path configuration: {}", err);
                let result = self.logger
                    .log_error(ErrorLogType::Error, &content);
                
                return request_fail(err, result);
            },
        };
    }
}
