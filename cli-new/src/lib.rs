// CLI management system library

#![allow(unused)]

use std::{
    process,
    io,
};
use engine_core::{
    self,
    DatabaseManager,
};

/// Configures program data
pub struct Config {
    engine_core_config: engine_core::Config,
    version: &'static str,
}

impl Config {
    /// Builds a new program configuration
    pub fn build() -> Self {
        Self {
            engine_core_config: engine_core::Config::build(),
            version: "0.0.0",
        }
    }
}

/// Runs the program
pub fn run(config: Config) {
    let mut engine = config.engine_core_config;

    let help_message = "Write /help for all available commands";

    println!("\n{}", "NOTE: You are using the newer version of this program - 'cli-new' is the crate name");
    println!("Version: {}", config.version);
    println!("\n{}\n", "Database Engine Project ");
    println!("{}", help_message);

    // Program main loop
    loop {
        let mut input_command = String::new();

        println!("\n{}", "Enter a command:");
        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        let input_command = input_command.trim();

        match input_command {
            "/help" => {
                println!("\n{}", "All available commands:");
                println!(
"
  /help                                List all available commands
  /q                                   Quit program
  (DISABLED) /connection status        Display current connection status

  /databases                           List all databases
  /create database                     Create a new database
  (DISABLED) /delete database          Delete a database
  (DISABLED) /connect database         Connect to a database

  ** THESE COMMANDS ARE NOT FINAL **

  (DISABLED) /collections              List all collection of a connected database
  (DISABLED) /create collection        Create a new collection in a connected database
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
            "/databases" => {
                list_all_databases(engine.database_manager());
            },
            "/create database" => {
                create_database_menu(engine.database_manager());
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
fn display_connection_status(database_manager: &DatabaseManager) {
    println!("Connected databases:");

    // Display connected databases
}

/// Show menu asking the name of the database.
/// After that, ask to confirm.
fn create_database_menu(database_manager: &DatabaseManager) {
    let mut database_name = String::new();
    let mut confirm = String::new();

    println!("\n{}", "Database name:");
    io::stdin()
        .read_line(&mut database_name)
        .expect("Failed to read line");

    let database_name = database_name.trim();

    println!("Confirm to create a new database named {}", database_name);
    println!("Yes?: y");
    io::stdin()
        .read_line(&mut confirm)
        .expect("Failed to read line");

    let confirm = confirm.trim();

    match confirm {
        // Create database
        "y" => {
            match database_manager.create_database(database_name) {
                Ok(result) => {
                    if result {
                        println!("Created database");
                    } else {
                        println!("Failed to create database. It might already exist.");
                    }
                },
                Err(e) => eprintln!("Error: {e}"),
            }
        },
        _ => println!("Canceled database creation"),
    }
    
}

/// List all databases and display information about them.
fn list_all_databases(database_manager: &DatabaseManager) {
    /*
    println!(
        "\n{}",
        "Number of databases: ",
    );
    */

    // Read all database files and iterate over them

    database_manager.find_all_databases();
}