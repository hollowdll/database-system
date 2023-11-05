use crate::Logger;
use crate::config::{
    Config,
    api::ConfigApi,
    config_manager::ConfigManager,
};
use crate::storage::{
    api::StorageApi,
    db_manager::DatabaseManager,
};

/// Engine version
const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Database engine.
/// 
/// Holds all the engine APIs and metadata.
pub struct Engine {
    storage_api: StorageApi,
    config_api: ConfigApi,
    version: &'static str,
}

impl Engine {
    /// Gets an immutable reference to the storage API.
    pub fn storage_api(&self) -> &StorageApi {
        &self.storage_api
    }

    /// Gets an immutable reference to the config API.
    pub fn config_api(&self) -> &ConfigApi {
        &self.config_api
    }

    /// Gets engine version.
    pub fn version(&self) -> &'static str {
        &self.version
    }
}

impl Engine {
    /// Builds the engine.
    pub fn build(config: &Config) -> Engine {
        Engine {
            storage_api: StorageApi::build(
                DatabaseManager::build(config.db_dir_path()),
                Logger::build(config.logs_dir_path()),
            ),
            config_api: ConfigApi::build(
                ConfigManager::build(config),
                Logger::build(config.logs_dir_path()),
            ),
            version: ENGINE_VERSION,
        }
    }
}

/// Database engine for database drivers.
/// 
/// This doesn't include a config file so it is more minimal.
/// Use this If the database driver doesn't need a config file.
pub struct DriverEngine {
    storage_api: StorageApi,
    version: &'static str,
}

impl DriverEngine {
    /// Gets an immutable reference to the storage API.
    pub fn storage_api(&self) -> &StorageApi {
        &self.storage_api
    }

    /// Gets engine version.
    pub fn version(&self) -> &'static str {
        &self.version
    }
}

impl DriverEngine {
    /// Builds the engine with logger enabled.
    pub fn build(config: &Config) -> DriverEngine {
        DriverEngine {
            storage_api: StorageApi::build(
                DatabaseManager::build(config.db_dir_path()),
                Logger::build(config.logs_dir_path()),
            ),
            version: ENGINE_VERSION,
        }
    }

    /// Builds the engine with logger disabled.
    pub fn build_logger_disabled(config: &Config) -> DriverEngine {
        DriverEngine {
            storage_api: StorageApi::build(
                DatabaseManager::build(config.db_dir_path()),
                Logger::build_disabled(config.logs_dir_path()),
            ),
            version: ENGINE_VERSION,
        }
    }
}