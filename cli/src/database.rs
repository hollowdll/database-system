use crate::{
    Cli,
    ask_user_input,
    ask_action_confirm,
    CONFIRM_OPTION_YES,
    NO_CONNECTED_DB,
};

impl Cli {
    /// Resets connected database to `None` if it doesn't exist anymore.
    pub fn refresh_connected_db(&mut self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return,
        };

        match &self.config.engine.api().find_database(connected_db_name) {
            Ok(result) => {
                if result.is_none() {
                    let _ = &self.config.connected_db.take();
                }
            },
            Err(e) => eprintln!("[Error] Failed to find connected database: {e}"),
        }
    }

    /// Checks if connected database exists.
    pub fn database_exists(
        &self,
        connected_db_name: &str,
    ) -> bool
    {
        match &self.config.engine.api().find_database(connected_db_name) {
            Ok(result) => {
                if result.is_none() {
                    println!("Cannot find database '{connected_db_name}'");
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

    /// Show menu to connect to a database.
    pub fn connect_database_menu(&mut self) {
        let db_name = match ask_user_input("Database name: ") {
            Ok(db_name) => db_name,
            Err(_) => return,
        };

        match &self.config.engine.api().find_database(&db_name) {
            Ok(result) => {
                if result.is_some() {
                    let _ = &self.config.connected_db.replace(db_name);
                    println!("Connected to database");
                } else {
                    println!("Failed to connect to database. Database does not exist.");
                }
            },
            Err(e) => eprintln!("[Error] {e}"),
        }
    }

    /// Show menu to create a new database.
    pub fn create_database_menu(&self) {
        let db_name = match ask_user_input("Database name: ") {
            Ok(db_name) => db_name,
            Err(_) => return,
        };

        match &self.config.engine.api().create_database(&db_name) {
            Ok(()) => println!("Database created"),
            Err(err) => eprintln!("[Error] {}", err),
        }
    }

    /// Show menu to delete a database.
    pub fn delete_database_menu(&mut self) {
        let db_name = match ask_user_input("Database name: ") {
            Ok(db_name) => db_name,
            Err(_) => return,
        };

        let confirm = match ask_action_confirm(
            &format!("Are you sure you want to delete database '{}'?", db_name)
        ) {
            Ok(confirm) => confirm,
            Err(_) => return,
        };

        match confirm.as_str() {
            CONFIRM_OPTION_YES => {
                match &self.config.engine.api().delete_database(&db_name) {
                    Ok(()) => {
                        // Disconnect database if it is connected
                        if let Some(connected_db_name) = &self.config.connected_db {
                            if connected_db_name == &db_name {
                                let _ = &self.config.connected_db.take();
                            }
                        }
                        println!("Database deleted");
                    },
                    Err(e) => eprintln!("[Error] {e}"),
                }
            },
            _ => return println!("Canceled action"),
        }
    }

    /// List all databases and display information about them.
    pub fn list_all_databases(&self) {
        let databases = match self.config.engine.api().find_all_databases() {
            Ok(databases) => databases,
            Err(e) => return eprintln!("[Error] {e}"),
        };

        println!("\nNumber of databases: {}", databases.len());

        for database in databases {
            println!(
"
  Name: {}
  Size: {} bytes
  Description: {}",
            database.name(),
            database.size(),
            database.description(),
            );
        }
    }

    /// Show menu to change database description.
    pub fn change_database_description_menu(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
        };

        let description = match ask_user_input("Description: ") {
            Ok(description) => description,
            Err(_) => return,
        };

        if !&self.database_exists(connected_db_name) {
            return;
        }

        // Change description of connected database
        match &self.config.engine.api().change_database_description(connected_db_name, &description) {
            Ok(()) => println!("Database description changed"),
            Err(e) => return eprintln!("[Error] {e}"),
        }
    }
}
