// Engine core library

// #![allow(unused)]

pub mod logging;
pub mod db;
mod db_manager;
mod input_data;
mod api;
pub mod config;

pub use api::EngineApi;
pub use logging::Logger;
pub use serde_json;
pub use db_manager::DatabaseManager;
pub use input_data::DocumentInputDataField;
use config::Config;

// Engine version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Engine structure.
pub struct Engine<'a> {
    api: EngineApi<'a>,
    version: &'static str,
}

impl<'a> Engine<'a> {
    pub fn api(&self) -> &EngineApi {
        &self.api
    }

    pub fn version(&self) -> &'static str {
        &self.version
    }
}

impl<'a> Engine<'a> {
    /// Builds a new engine structure.
    pub fn build(config: &'a Config, logger: &'a Logger) -> Engine<'a> {
        Engine {
            api: EngineApi::build(DatabaseManager::build(
                config.db_dir_path(),
                logger,
            )),
            version: VERSION,
        }
    }
}



#[cfg(test)]
mod tests {
    
}
