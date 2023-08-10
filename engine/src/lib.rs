// Entry point to engine library.

// #![allow(unused)]

pub mod logging;
pub mod storage;
mod input_data;
pub mod config;

pub use logging::Logger;
pub use serde_json;
pub use input_data::DocumentInputDataField;
use config::{
    Config,
    api::ConfigApi,
    config_manager::ConfigManager,
};
use storage::{
    api::StorageApi,
    db_manager::DatabaseManager,
};

// Engine version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Engine structure.
/// 
/// Holds all the engine APIs and metadata.
pub struct Engine<'a> {
    storage_api: StorageApi<'a>,
    config_api: ConfigApi<'a>,
    version: &'static str,
}

impl<'a> Engine<'a> {
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

impl<'a> Engine<'a> {
    /// Builds engine structure.
    pub fn build(config: &'a Config, logger: &'a Logger) -> Engine<'a> {
        Engine {
            storage_api: StorageApi::build(
                DatabaseManager::build(config),
                logger,
            ),
            config_api: ConfigApi::build(
                ConfigManager::build(config),
                logger,
            ),
            version: VERSION,
        }
    }
}



#[cfg(test)]
mod tests {
    
}
