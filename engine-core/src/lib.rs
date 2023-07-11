// Engine core library

// #![allow(unused)]

pub mod logging;
pub mod db;
mod db_manager;
mod input_data;
mod api;
mod config;

use std::path::{
    PathBuf,
    Path
};
pub use api::EngineApi;
pub use logging::Logger;
pub use serde_json;
pub use db_manager::DatabaseManager;
pub use input_data::DocumentInputDataField;

const DB_DIR_PATH: &str = "./databases";
const LOGS_DIR_PATH: &str = "./logs";
// Engine version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Engine configuration.
#[derive(PartialEq, Debug)]
pub struct Config {
    db_dir_path: PathBuf,
    pub logs_dir_path: PathBuf,
}

impl Config {
    pub fn db_dir_path(&self) -> &Path {
        &self.db_dir_path
    }

    pub fn logs_dir_path(&self) -> &Path {
        &self.logs_dir_path
    }
}

impl Config {
    /// Builds a new engine configuration.
    pub fn build() -> Config {
        Config {
            db_dir_path: PathBuf::from(DB_DIR_PATH),
            logs_dir_path: PathBuf::from(LOGS_DIR_PATH),
        }
    }
}

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
    pub fn build(config: Config, logger: &'a Logger) -> Engine<'a> {
        Engine {
            api: EngineApi::build(DatabaseManager::build(
                config.db_dir_path,
                logger,
            )),
            version: VERSION,
        }
    }
}



#[cfg(test)]
mod tests {
    
}
