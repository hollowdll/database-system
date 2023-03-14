// Engine core library
// Prototype phase
// Will be organized and structured better later
// This library will be split into modules later

#![allow(unused)]

/// Configure engine data.
pub struct Config {
    database_manager: DatabaseManager,
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

#[derive(Debug)]
pub struct Database {
    name: String,
    connected: bool,
    tables: Vec<DatabaseTable>,
}

impl Database {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn tables(&self) -> &[DatabaseTable] {
        &self.tables
    }

    /// Connect to this database.
    fn connect(&mut self) {
        if !self.connected {
            self.connected = true;

            println!("Connected to database {}", self.name);
        } else {
            println!("Already connected to database {}", self.name);
        }
    }

    /// Disconnect from this database.
    fn disconnect(&mut self) {
        if self.connected {
            self.connected = false;

            println!("Disconnected from database {}", self.name);
        } else {
            println!("Already disconnected from database {}", self.name);
        }
    }
}

impl Database {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            connected: false,
            tables: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct DatabaseTable {
    name: String,
    columns: Vec<DatabaseTableColumn>,
    // Might be changed later
    id_column: Option<DatabaseTableColumn>,
}

impl DatabaseTable {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn columns(&self) -> &[DatabaseTableColumn] {
        &self.columns
    }
}

impl DatabaseTable {
    fn create_id_column() -> DatabaseTableColumn {
        DatabaseTableColumn {
            name: String::from("id"),
            data_type: DatabaseDataType::Id,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseTableColumn {
    name: String,
    data_type: DatabaseDataType,
}

#[derive(Debug)]
enum DatabaseDataType {
    // All database data types
    // Some of these are still in planning phase
    // Some types that will be added: serial(identity), text, int, decimal, bool
    Id,
}

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

            println!("Connected to database manager");
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

            println!("Disconnected from database manager");
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

                println!("Created database: {}", database_name);

                return Ok(());
            } else {
                return Err("Database with given name already exists");
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

                println!("Deleted database: {}", database_name);

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
    /// Returns the index of found database wrapped inside `Some`
    /// or `None` if no database was found.
    fn find_database(&self, database_name: &str) -> Option<(usize, &Database)> {
        for (i, db) in self.databases.iter().enumerate() {
            if db.name() == database_name {
                return Some((i, db));
            }
        }

        return None;
    }

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
