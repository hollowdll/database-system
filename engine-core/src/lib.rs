// Engine core library
// Prototype phase
// Will be organized and structured better later

#![allow(unused)]

pub mod logs;
mod db;
mod db_manager;
mod input_data;

pub use db_manager::DatabaseManager;
pub use db::DataType;
pub use db::InputDataField;
pub use serde_json;

/// Configure engine data.
#[derive(PartialEq, Debug)]
pub struct Config {
    database_manager: db_manager::DatabaseManager,
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
}

impl Config {
    /// Builds a new engine configuration.
    /// 
    /// Call this only once.
    pub fn build() -> Config {
        Config {
            database_manager: DatabaseManager::build(),
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
        };

        assert_eq!(config, Config::build());
    }
}
