// Engine configuration API

use std::path::Path;
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

    /// Result message.
    pub message: String,

    /// Possible error that occurred during logging.
    pub log_error: Option<LogError>,
}

/// Engine configuration API.
/// 
/// Provides methods to change engine configurations.
/// 
/// Logs errors and events before forwarding results to the caller.
pub struct ConfigApi<'a> {
    config_manager: ConfigManager<'a>,
    logger: &'a Logger<'a>,
}

impl<'a> ConfigApi<'a> {
    /// Builds config API.
    pub fn build(
        config_manager: ConfigManager<'a>,
        logger: &'a Logger<'a>,
    ) -> ConfigApi<'a>
    {
        ConfigApi {
            config_manager,
            logger,
        }
    }
}

impl<'a> ConfigApi<'a> {
    /// Requests `ConfigManager` to set database directory path config.
    /// 
    /// Forwards the result to the caller.
    pub fn set_db_dir_path(&self, path: &Path) -> ConfigRequestResult {
        match self.config_manager.set_db_dir_path(path) {
            Ok(()) => {
                let content = format!("Changed database directory path configuration to {:?}", path);
                if let Err(e) = self.logger.log_event(&content) {
                    return ConfigRequestResult {
                        success: true,
                        message: content,
                        log_error: Some(e),
                    };
                }
                return ConfigRequestResult {
                    success: true,
                    message: content,
                    log_error: None,
                };
            },
            Err(e) => {
                let content = format!("Failed to change database directory path configuration: {}", e);
                if let Err(e) = self.logger
                    .log_error(ErrorLogType::Error, &content)
                {
                    return ConfigRequestResult {
                        success: false,
                        message: content,
                        log_error: Some(e),
                    };
                }
                return ConfigRequestResult {
                    success: false,
                    message: content,
                    log_error: None,
                };
            },
        };
    }

    /// Requests `ConfigManager` to set logs directory path config.
    /// 
    /// Forwards the result to the caller.
    pub fn set_logs_dir_path(&self, path: &Path) -> ConfigRequestResult {
        match self.config_manager.set_logs_dir_path(path) {
            Ok(()) => {
                let content = format!("Changed logs directory path configuration to {:?}", path);
                if let Err(e) = self.logger.log_event(&content) {
                    return ConfigRequestResult {
                        success: true,
                        message: content,
                        log_error: Some(e),
                    };
                }
                return ConfigRequestResult {
                    success: true,
                    message: content,
                    log_error: None,
                };
            },
            Err(e) => {
                let content = format!("Failed to change logs directory path configuration: {}", e);
                if let Err(e) = self.logger
                    .log_error(ErrorLogType::Error, &content)
                {
                    return ConfigRequestResult {
                        success: false,
                        message: content,
                        log_error: Some(e),
                    };
                }
                return ConfigRequestResult {
                    success: false,
                    message: content,
                    log_error: None,
                };
            },
        };
    }
}
