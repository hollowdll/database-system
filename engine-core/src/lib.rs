// Engine core library
// Prototype phase
// Will be organized and structured better later
// This library will be split into modules later

#![allow(unused)]

/// Configure program data.
pub struct Config {
    database_manager: DatabaseManager,
}

impl Config {
    pub fn database_manager(&self) -> &DatabaseManager {
        &self.database_manager
    }
}

impl Config {
    //TODO: this will build database manager
    // and all program data

    /// Builds a new program configuration.
    pub fn build() -> Config {
        return Config {
            database_manager: DatabaseManager::build(),
        }
    }
}

#[derive(Debug)]
pub struct Database {

}

struct DatabaseTable {

}

// This will be configured better later
/// Database manager that will
/// manage all database instances.
/// 
/// All data related to database instances
/// will be managed by this.
#[derive(Debug)]
pub struct DatabaseManager {
    connected: bool,
    database_count: u32,
    databases: Vec<Database>,
}

impl DatabaseManager {
    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn database_count(&self) -> u32 {
        self.database_count
    }

    pub fn databases(&self) -> &[Database] {
        &self.databases
    }

    /// Connect to this database manager.
    pub fn connect(&mut self) {
        self.connected = true
        // more code
    }

    // Disconnect from this database manager.
    pub fn disconnect(&mut self) {
        self.connected = false
        // more code
    }
}

impl DatabaseManager {
    /// Build a new database manager.
    /// 
    /// This is supposed to be called only once.
    /// 
    /// `Config` handles this so user doesn't need to worry about this.
    fn build() -> Self {
        Self {
            connected: false,
            database_count: 0,
            databases: Vec::new(),
        }
    }
}



#[cfg(test)]
mod tests {

}
