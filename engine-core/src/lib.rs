// Engine core library
// Prototype phase
// Will be organized and structured better later

#![allow(unused)]

pub mod logs;
mod db;
mod db_manager;

pub use db_manager::DatabaseManager;

/// Configure engine data.
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
    /// Builds a new engine configuration with needed data.
    /// 
    /// This is intended to be called only once.
    pub fn build() -> Config {
        Config {
            database_manager: DatabaseManager::build(),
        }
    }
}

/*
#[derive(Debug)]
pub struct Database {
    name: String,
    connected: bool,
}

impl Database {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    /// Connect to this database.
    fn connect(&mut self) {
        if !self.connected {
            self.connected = true;

            let log_content = format!("Connected to database: {}", self.name);
            if let Err(e) = logs::log_database_event(
                logs::DatabaseEventSource::Database,
                logs::DatabaseEventType::Connected,
                log_content.as_str()
            ) {
                eprintln!("Error: {e}");
            }
            println!("{}", log_content);
        } else {
            println!("Already connected to database {}", self.name);
        }
    }

    /// Disconnect from this database.
    fn disconnect(&mut self) {
        if self.connected {
            self.connected = false;

            let log_content = format!("Disconnected from database: {}", self.name);
            if let Err(e) = logs::log_database_event(
                logs::DatabaseEventSource::Database,
                logs::DatabaseEventType::Disconnected,
                log_content.as_str()
            ) {
                eprintln!("Error: {e}");
            }
            println!("{}", log_content);
        } else {
            println!("Already disconnected from database {}", self.name);
        }
    }
}

impl Database {
    /// Creates a new database instance
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            connected: false,
        }
    }
}
*/

/*
/// Database manager that will
/// manage all database instances.
/// 
/// All data related to database instances
/// will be managed by this.
#[derive(Debug)]
pub struct DatabaseManager {
    connected: bool,
    databases: Vec<Database>,
}

impl DatabaseManager {
    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn databases(&self) -> &[Database] {
        &self.databases
    }

    /// Connect to this database manager.
    pub fn connect(&mut self) {
        if !self.connected {
            self.connected = true;

            let log_content = "Connected to database manager";
            if let Err(e) = logs::log_database_event(
                logs::DatabaseEventSource::DatabaseManager,
                logs::DatabaseEventType::Connected,
                log_content
            ) {
                eprintln!("Error: {e}");
            }
            println!("{}", log_content);
        } else {
            println!("Already connected to database manager");
        }
    }

    /// Disconnect from this database manager.
    pub fn disconnect(&mut self) {
        if self.connected {
            self.connected = false;
            
            // Disconnect all databases
            for db in self.databases.iter_mut() {
                if db.connected {
                    db.disconnect();
                }
            }

            let log_content = "Disconnected from database manager";
            if let Err(e) = logs::log_database_event(
                logs::DatabaseEventSource::DatabaseManager,
                logs::DatabaseEventType::Disconnected,
                log_content
            ) {
                eprintln!("Error: {e}");
            }
            println!("{}", log_content);
        } else {
            println!("Already disconnected from database manager");
        }
    }

    /// Creates a new database to this database manager
    pub fn create_database(&mut self, database_name: &str) -> Result<(), &'static str> {
        if self.connected {
            if !self.database_exists(database_name) {
                let database = Database::new(database_name);
    
                println!("{:?}", database);
                
                self.databases.push(database);

                let log_content = format!("Created database: {}", database_name);
                if let Err(e) = logs::log_database_event(
                    logs::DatabaseEventSource::Database,
                    logs::DatabaseEventType::Created,
                    log_content.as_str()
                ) {
                    eprintln!("Error: {e}");
                }
                println!("{}", log_content);

                return Ok(());
            } else {
                return Err("Database with the given name already exists");
            }
        } else {
            return Err("Not connected to database manager");
        }
    }

    /// Deletes a database from this database manager
    pub fn delete_database(&mut self, database_name: &str) -> Result<(), &'static str> {
        if self.connected {
            if let Some((i, _db)) = self.find_database(database_name) {
                self.databases.remove(i);

                let log_content = format!("Deleted database: {}", database_name);
                if let Err(e) = logs::log_database_event(
                    logs::DatabaseEventSource::Database,
                    logs::DatabaseEventType::Deleted,
                    log_content.as_str()
                ) {
                    eprintln!("Error: {e}");
                }
                println!("{}", log_content);

                return Ok(());
            } else {
                return Err("Cannot find database");
            }
        } else {
            return Err("Not connected to database manager");
        }
    }

    /// Connect a database in this database manager.
    pub fn connect_database(&mut self, database_name: &str) -> Result<&Database, &'static str> {
        if self.connected {
            if let Some((_i, db)) = self.find_database_mut(database_name) {
                db.connect();

                return Ok(db);
            } else {
                return Err("Cannot find database");
            }
        } else {
            return Err("Not connected to database manager");
        }
    }

    /// Disconnect a database in this database manager.
    pub fn disconnect_database(&mut self, database_name: &str) -> Result<&Database, &'static str> {
        if self.connected {
            if let Some((_i, db)) = self.find_database_mut(database_name) {
                db.disconnect();

                return Ok(db);
            } else {
                return Err("Cannot find database");
            }
        } else {
            return Err("Not connected to database manager");
        }
    }

    /// Checks if there is a database with the given name
    fn database_exists(&self, database_name: &str) -> bool {
        for db in self.databases.iter() {
            if db.name() == database_name {
                return true;
            }
        }

        return false;
    }

    /// Tries to find a database with the given name.
    /// 
    /// Returned database is an immutable reference
    fn find_database(&self, database_name: &str) -> Option<(usize, &Database)> {
        for (i, db) in self.databases.iter().enumerate() {
            if db.name() == database_name {
                return Some((i, db));
            }
        }

        return None;
    }

    /// Tries to find a database with the given name.
    /// 
    /// Returned database is a mutable reference
    fn find_database_mut(&mut self, database_name: &str) -> Option<(usize, &mut Database)> {
        for (i, db) in self.databases.iter_mut().enumerate() {
            if db.name() == database_name {
                return Some((i, db));
            }
        }

        return None;
    }
}

impl DatabaseManager {
    // Build a new database manager.
    fn build() -> Self {
        Self {
            connected: false,
            databases: Vec::new(),
        }
    }
}
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_db_manager() {
        // not needed yet
    }

    #[test]
    fn disconnect_from_db_manager() {
        // not needed yet
    }
}
