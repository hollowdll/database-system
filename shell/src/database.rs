use std::path::{
    Path,
    PathBuf,
};
use crate::{
    cli::Cli,
    ask_user_input,
    ask_action_confirm,
    cli::CONFIRM_OPTION_YES,
    db_not_connected,
    event_log_failed,
    error_log_failed,
};

/// Display text that tells there is no connected database.
pub const NO_CONNECTED_DB: &str = "No connected database";

/// Represents the connected database.
/// 
/// Holds its name and file path.
pub struct ConnectedDatabase {
    name: String,
    file_path: PathBuf,
}

impl ConnectedDatabase {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    pub fn new(name: &str, file_path: &Path) -> Self {
        Self {
            name: String::from(name),
            file_path: PathBuf::from(file_path),
        }
    }
}

impl Cli {
    /// Resets connected database to `None` if it doesn't exist anymore.
    pub fn refresh_connected_db(&mut self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return,
        };
        let result = self.engine
            .storage_api()
            .find_database_by_file_path(connected_db.file_path());

        if result.success {
            event_log_failed(result.log_error);

            if let Some(db) = result.data {
                if db.is_none() {
                    let _ = &self.connected_db.take();
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: Failed to find connected database: {}", e.message);

                // Disconnect db to prevent ceaseless loop
                let _ = &self.connected_db.take();
            }
        }
    }

    /// Checks if connected database exists.
    pub fn database_exists(
        &self,
        connected_db: &ConnectedDatabase,
    ) -> bool
    {
        let result = self.engine
            .storage_api()
            .find_database_by_file_path(connected_db.file_path());

        if result.success {
            event_log_failed(result.log_error);

            if let Some(db) = result.data {
                if db.is_none() {
                    println!("Cannot find connected database");
                    return false;
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: Failed to find connected database: {}", e.message);
            }
            return false;
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
        let result = self.engine
            .storage_api()
            .find_database(&db_name);

        if result.success {
            event_log_failed(result.log_error);

            if let Some(db) = result.data {
                if let Some(db) = db {
                    let _ = &self.connected_db.replace(ConnectedDatabase::new(
                        db.name(),
                        db.file_path()
                    ));
                    println!("Connected to database");
                } else {
                    println!("Cannot find database");
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: Failed to connect to database: {}", e.message);
            }
        }
    }

    /// Show menu to connect to a database by its file path.
    pub fn connect_db_by_file_path(&mut self) {
        let file_path = match ask_user_input("Database file path: ") {
            Ok(file_path) => file_path,
            Err(_) => return,
        };
        let file_path = Path::new(&file_path);
        let result = self.engine
            .storage_api()
            .find_database_by_file_path(file_path);

        if result.success {
            event_log_failed(result.log_error);

            if let Some(db) = result.data {
                if let Some(db) = db {
                    let _ = &self.connected_db.replace(ConnectedDatabase::new(
                        db.name(),
                        db.file_path()
                    ));
                    println!("Connected to database");
                } else {
                    println!("Cannot find database");
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: Failed to connect to database: {}", e.message);
            }
        }
    }

    /// Show menu to create a new database.
    pub fn create_database(&self) {
        let db_name = match ask_user_input("Database name: ") {
            Ok(db_name) => db_name,
            Err(_) => return,
        };
        let result = self.engine
            .storage_api()
            .create_database_to_db_dir(&db_name);

        if result.success {
            event_log_failed(result.log_error);

            println!("Database created");
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
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
                let result = self.engine
                    .storage_api()
                    .delete_database(connected_db.file_path());

                if result.success {
                    event_log_failed(result.log_error);

                    // Disconnect database if it is connected
                    let _ = &self.connected_db.take();
                    println!("Database deleted");
                    
                } else {
                    error_log_failed(result.log_error);

                    if let Some(e) = result.error {
                        eprintln!("Error: {}", e);
                    }
                }
            },
            _ => return println!("Canceled action"),
        }
    }

    /// List all databases and display information about them.
    pub fn list_all_databases(&self) {
        let result = self.engine
            .storage_api()
            .find_all_databases();

        if result.success {
            event_log_failed(result.log_error);

            if let Some(databases) = result.data {
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
                    database.file_path().display());
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
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
        let result = self.engine
            .storage_api()
            .change_database_description(connected_db.file_path(), &description);

        if result.success {
            event_log_failed(result.log_error);

            println!("Database description changed");
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }
}
