// Engine core library

// #![allow(unused)]

pub mod logging;
pub mod db;
mod db_manager;
mod input_data;
mod api;

use std::path::PathBuf;
pub use api::EngineApi;
use logging::Logger;
pub use serde_json;
pub use db_manager::DatabaseManager;
pub use input_data::DocumentInputDataField;

const DB_DIR_PATH: &str = "./databases";
const LOGS_DIR_PATH: &str = "./logs";
// Engine version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Configure engine data.
#[derive(PartialEq, Debug)]
pub struct Config {
    api: EngineApi,
    version: &'static str,
    db_dir_path: PathBuf,
    logs_dir_path: PathBuf,
}

impl Config {
    pub fn version(&self) -> &'static str {
        &self.version
    }

    pub fn api(&self) -> &EngineApi {
        &self.api
    }
}

impl Config {
    /// Builds a new engine configuration.
    /// 
    /// Call this only once.
    pub fn build() -> Config {
        Config {
            api: EngineApi::build(DatabaseManager::build(
                PathBuf::from(DB_DIR_PATH),
                Logger::build(PathBuf::from(LOGS_DIR_PATH)),
            )),
            version: VERSION,
            // db_dir_path is hard coded for now.
            // Will be changed later to read from config file.
            db_dir_path: PathBuf::from(DB_DIR_PATH),
            logs_dir_path: PathBuf::from(LOGS_DIR_PATH),
        }
    }
}



#[cfg(test)]
mod tests {
    
}
