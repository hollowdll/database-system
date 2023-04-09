// CLI management system library

#![allow(unused)]

use std::{
    process,
    io::{self, Write},
};
use engine_core::{
    self,
    DatabaseManager,
};

/// Configures program data
pub struct Config {
    engine_core_config: engine_core::Config,
    version: &'static str,
    connected_database: Option<String>,
}

impl Config {
    /// Builds a new program configuration
    pub fn build() -> Self {
        Self {
            engine_core_config: engine_core::Config::build(),
            version: "0.0.0",
            connected_database: None,
        }
    }
}

/* Disabled
/// Currently connected database
struct ConnectedDatabase {
    name: &'static str,
}

impl ConnectedDatabase {
    fn name(&self) -> &str {
        &self.name
    }
}
*/

/// Runs the program
pub fn run(config: Config) {
    let mut engine = config.engine_core_config;
    let mut connected_database = config.connected_database;
    let help_message = "Write /help for all available commands";

    println!("\n{}", "NOTE: You are using the newer version of this program - 'cli-new' is the crate name");
    println!("Version: {}", config.version);
    println!("\n{}\n", "Database Engine Project ");
    println!("{}", help_message);

    // Program main loop
    loop {
        let mut input_command = String::new();

        println!();
        if let Some(database_name) = &connected_database {
            println!("Connected database: {database_name}");
        }
        println!("{}", "Enter a command:");

        if let Err(e) = io::stdin().read_line(&mut input_command) {
            eprintln!("Failed to read line: {e}");
            continue
        }

        let input_command = input_command.trim();

        match input_command {
            "/help" => {
                println!("\n{}", "All available commands:");
                println!(
"
  /help                                List all available commands
  /q                                   Quit program
  /connection status                   Display currently connected database

  /databases                           List all databases
  /create database                     Create a new database
  /delete database                     Delete a database
  /connect database                    Connect to a database

  ** THESE COMMANDS ARE NOT FINAL **

  (DISABLED) /collections              List all collection of a connected database
  /create collection                   Create a new collection in a connected database
  (DISABLED) /delete collection        Delete a collection in a connected database
  
  (DISABLED) /documents                List documents of a collection
  (DISABLED) /create document          Create a new document in a collection
  (DISABLED) /delete document          Delete a document in a collection

  ** COMMANDS FOR TESTING **

  /create test log                     Creates test log
  
  More commands in the future...");
            },
            "/q" => {
                exit_program()
            },
            "/connection status" => {
                display_connection_status(&connected_database);
            },
            "/databases" => {
                list_all_databases(engine.database_manager());
            },
            "/create database" => {
                show_create_database_menu(engine.database_manager());
            },
            "/delete database" => {
                show_delete_database_menu(engine.database_manager());
            },
            "/connect database" => {
                show_connect_database_menu(engine.database_manager(), &mut connected_database);
            },
            "/create collection" => {
                show_create_collection_menu(engine.database_manager(), &connected_database);
            },
            "/create test log" => {
                use engine_core::logs;
                for _ in 0..5 {
                    logs::create_test_log();
                }
            },
            _ => {
                println!("No such command found!");
                println!("{}", help_message);
                continue
            },
        }
    }
}

/// Exit the program.
fn exit_program() {
    println!("Exiting...");
    process::exit(0);
}

/// Display connected database.
fn display_connection_status(connected_database: &Option<String>) {
    match connected_database {
        Some(database_name) => println!("Connected database: {database_name}"),
        None => println!("No connected database"),
    }
}

/// Show menu to create a new database.
fn show_create_database_menu(database_manager: &DatabaseManager) {
    let mut database_name = String::new();

    println!("\n{}", "Database name:");
    if let Err(e) = io::stdin().read_line(&mut database_name) {
        return eprintln!("Failed to read line: {e}");
    };

    let database_name = database_name.trim();

    match database_manager.create_database(database_name) {
        Ok(result) => {
            if result {
                println!("Created database");
            } else {
                println!("Failed to create database. It might already exist.");
            }
        },
        Err(e) => eprintln!("Error occurred while trying to create a database: {e}"),
    }
}

/// Show menu to delete a database.
fn show_delete_database_menu(database_manager: &DatabaseManager) {
    let mut database_name = String::new();
    let mut confirm = String::new();

    println!("\n{}", "Database name:");
    if let Err(e) = io::stdin().read_line(&mut database_name) {
        return eprintln!("Failed to read line: {e}");
    }

    let database_name = database_name.trim();

    println!("Are you sure you want to delete database '{}'?", database_name);
    print!("Press 'Y' to confirm: ");
    io::stdout().flush().unwrap();

    if let Err(e) = io::stdin().read_line(&mut confirm) {
        return eprintln!("Failed to read line: {e}");
    }

    let confirm = confirm.trim();

    match confirm {
        // Delete database
        "Y" => {
            match database_manager.delete_database(database_name) {
                Ok(result) => {
                    if result {
                        println!("Deleted database");
                    } else {
                        println!("Failed to delete database. It might not exist.");
                    }
                },
                Err(e) => eprintln!("Error occurred while trying to delete database: {e}"),
            }
        },
        _ => println!("Canceled database deletion"),
    }
}

/// Show menu to connect to a database.
fn show_connect_database_menu(
    database_manager: &DatabaseManager,
    connected_database: &mut Option<String>
) {
    let mut database_name = String::new();

    println!("\n{}", "Database name:");
    if let Err(e) = io::stdin().read_line(&mut database_name) {
        return eprintln!("Failed to read line: {e}");
    }

    let database_name = database_name.trim();

    match database_manager.find_database(database_name) {
        Ok(result) => {
            if result {
                connected_database.replace(database_name.to_string());
                println!("Connected to database");
            } else {
                println!("Failed to connect to database. It might not exist.");
            }
        },
        Err(e) => eprintln!("Error occurred while trying to connect to database: {e}"),
    }
}

/// List all databases and display information about them.
fn list_all_databases(database_manager: &DatabaseManager) {
    let databases = match database_manager.find_all_databases() {
        Ok(databases) => databases,
        Err(e) => return eprintln!("Error occurred while trying to find databases: {e}"),
    };

    println!("\nNumber of databases: {}", databases.len());

    for database in databases {
        println!(
"
  Name: {}
  Size: {} bytes",
        database.name(),
        database.size()
        );
    }
}

/// Show menu to create a new collection
/// in the connected database
fn show_create_collection_menu(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>
) {
    let mut collection_name = String::new();

    println!("\n{}", "Collection name:");
    if let Err(e) = io::stdin().read_line(&mut collection_name) {
        return eprintln!("Failed to read line: {e}");
    }

    let database_name = collection_name.trim();

    // Check if connected database exists
    // Create collection
}
