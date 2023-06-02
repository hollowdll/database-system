// Engine core library

// #![allow(unused)]

pub mod logging;
pub mod db;
mod db_manager;
mod input_data;
pub mod constants;

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
            database_manager: DatabaseManager::build(),
            version: VERSION,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_config_build_works() {
        let config = Config {
            database_manager: DatabaseManager::build(),
            version: VERSION,
        };

        assert_eq!(config, Config::build());
    }
}
