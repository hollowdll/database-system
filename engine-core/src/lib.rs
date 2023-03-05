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

    pub fn database_manager_mut(&mut self) -> &mut DatabaseManager {
        &mut self.database_manager
    }
}

impl Config {
    //TODO: this will build database manager
    // and all program data

    /// Builds a new program configuration.
    /// 
    /// This is intended to be called only once.
    pub fn build() -> Config {
        return Config {
            database_manager: DatabaseManager::build(),
        }
    }
}

#[derive(Debug)]
pub struct Database {
    name: String,
    tables: Vec<DatabaseTable>,
}

impl Database {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tables(&self) -> &[DatabaseTable] {
        &self.tables
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
            println!("Disconnected from database manager");
        } else {
            println!("Already disconnected from database manager");
        }
    }

    /// Creates a new database to this database manager
    pub fn create_database(&mut self, database_name: &str) {
        if self.connected {
            let database = Database {
                name: database_name.to_string(),
                tables: Vec::new(),
            };

            println!("{:?}", database);
            
            self.databases.push(database);
        } else {
            println!("Connect to database manager before attempting to create a database!");
        }
    }

    /// Deletes a database from this database manager
    pub fn delete_database(&mut self) {

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
