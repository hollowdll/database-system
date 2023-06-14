// Engine core library

// #![allow(unused)]

pub mod logging;
pub mod db;
mod db_manager;
mod input_data;
pub mod constants;

use std::path::PathBuf;
pub use serde_json;
pub use db_manager::DatabaseManager;
pub use input_data::InputDataField;

// Engine version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Configure engine data.
#[derive(PartialEq, Debug)]
pub struct Config {
    database_manager: db_manager::DatabaseManager,
    version: &'static str,
    db_dir_path: PathBuf,
    logs_dir_path: PathBuf,
}

impl Config {
    /// Returns an immutable reference to `DatabaseManager`
    pub fn database_manager(&self) -> &DatabaseManager {
        &self.database_manager
    }

    /// Returns a mutable reference to `DatabaseManager`
    pub fn database_manager_mut(&mut self) -> &mut DatabaseManager {
        &mut self.database_manager
    }

    pub fn version(&self) -> &'static str {
        &self.version
    }
}

impl Config {
    /// Builds a new engine configuration.
    /// 
    /// Call this only once.
    pub fn build() -> Config {
        Config {
            // db_dir_path is hard coded for now.
            // Will be changed later to read from config file.
            database_manager: DatabaseManager::build(
                PathBuf::from("./databases"),
                logging::get_logs_dir_path()
            ),
            version: VERSION,
            db_dir_path: PathBuf::from("./databases"),
            logs_dir_path: logging::get_logs_dir_path(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_config_build_works() {
        let config = Config {
            database_manager: DatabaseManager::build(
                PathBuf::from("./databases"),
                logging::get_logs_dir_path()
            ),
            version: VERSION,
            db_dir_path: PathBuf::from("./databases"),
            logs_dir_path: logging::get_logs_dir_path(),
        };

        assert_eq!(config, Config::build());
    }
}
