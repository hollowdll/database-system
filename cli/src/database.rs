use std::path::Path;
use crate::{
    Cli,
    ask_user_input,
    ask_action_confirm,
    CONFIRM_OPTION_YES,
    ConnectedDatabase,
    db_not_connected,
};

impl<'a> Cli<'a> {
    /// Resets connected database to `None` if it doesn't exist anymore.
    pub fn refresh_connected_db(&mut self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return,
        };

        match &self.engine
            .storage_api()
            .find_database_by_file_path(connected_db.file_path())
        {
            Ok(db) => {
                if db.is_none() {
                    let _ = &self.connected_db.take();
                }
            },
            Err(e) => eprintln!("[Error] Failed to find connected database: {e}"),
        }
    }

    /// Checks if connected database exists.
    pub fn database_exists(
        &self,
        connected_db: &ConnectedDatabase,
    ) -> bool
    {
        match &self.engine
            .storage_api()
            .find_database_by_file_path(connected_db.file_path())
        {
            Ok(result) => {
                if result.is_none() {
                    println!("Cannot find connected database");
                    return false;
                }
            },
            Err(e) => {
                eprintln!("[Error] Failed to find connected database: {e}");
                return false;
            },
        }

        return true;
    }

    /// Show menu to connect to a database by its name.
    /// 
    /// Tries to find the database from the database directory.
    pub fn connect_db_by_name(&mut self) {
        let db_name = match ask_user_input("Database name: ") {
            Ok(db_name) => db_name,
            Err(_) => return,
        };

        match &self.engine.storage_api().find_database(&db_name) {
            Ok(result) => {
                if let Some(db) = result {
                    let _ = &self.connected_db.replace(ConnectedDatabase::new(
                        db.name(),
                        db.file_path()
                    ));
                    println!("Connected to database");
                } else {
                    println!("Cannot find database");
                }
            },
            Err(e) => eprintln!("[Error] Failed to connect to database: {}", e),
        }
    }

    /// Show menu to connect to a database by its file path.
    pub fn connect_db_by_file_path(&mut self) {
        let file_path = match ask_user_input("Database file path: ") {
            Ok(file_path) => file_path,
            Err(_) => return,
        };
        let file_path = Path::new(&file_path);

        match self.engine.storage_api().find_database_by_file_path(file_path) {
            Ok(result) => {
                if let Some(db) = result {
                    let _ = &self.connected_db.replace(ConnectedDatabase::new(
                        db.name(),
                        db.file_path()
                    ));
                    println!("Connected to database");
                } else {
                    println!("Cannot find database");
                }
            },
            Err(e) => eprintln!("[Error] Failed to connect to database: {}", e),
        }
    }

    /// Show menu to create a new database.
    pub fn create_database(&self) {
        let db_name = match ask_user_input("Database name: ") {
            Ok(db_name) => db_name,
            Err(_) => return,
        };

        match &self.engine.storage_api().create_database_to_db_dir(&db_name) {
            Ok(()) => println!("Database created"),
            Err(e) => eprintln!("[Error] Failed to create database: {}", e),
        }
    }

    /// Show menu to delete a database.
    pub fn delete_database(&mut self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };

        let confirm = match ask_action_confirm(
            &format!("All data in this database will be lost. Delete database?")
        ) {
            Ok(confirm) => confirm,
            Err(_) => return,
        };

        match confirm.as_str() {
            CONFIRM_OPTION_YES => {
                match &self.engine.storage_api().delete_database(connected_db.file_path()) {
                    Ok(()) => {
                        // Disconnect database if it is connected
                        let _ = &self.connected_db.take();
                        println!("Database deleted");
                    },
                    Err(e) => eprintln!("[Error] Failed to delete database: {e}"),
                }
            },
            _ => return println!("Canceled action"),
        }
    }

    /// List all databases and display information about them.
    pub fn list_all_databases(&self) {
        let databases = match self.engine
            .storage_api()
            .find_all_databases()
        {
            Ok(databases) => databases,
            Err(e) => return eprintln!("[Error] Failed to list databases: {e}"),
        };

        println!("\nNumber of databases: {}", databases.len());

        for database in databases {
            println!(
"
  Name:        {}
  Size:        {} bytes
  Description: {}
  File path:   {}",
            database.name(),
            database.size(),
            database.description(),
            database.file_path().display()
            );
        }
    }

    /// Show menu to change database description.
    pub fn change_database_description(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };

        let description = match ask_user_input("Description: ") {
            Ok(description) => description,
            Err(_) => return,
        };

        if !&self.database_exists(connected_db) {
            return;
        }

        match &self.engine
            .storage_api()
            .change_database_description(connected_db.file_path(), &description)
        {
            Ok(()) => println!("Database description changed"),
            Err(e) => return eprintln!("[Error] Failed to change database description: {e}"),
        }
    }
}
