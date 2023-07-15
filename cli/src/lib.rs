// #![allow(unused)]

mod database;
mod collection;
mod document;
pub mod config;

use std::{
    process,
    io::{self, Write},
};
use engine_core::{
    self,
    config::{
        Config,
        load_config,
    },
    Logger
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NO_CONNECTED_DB: &str = "No connected database";
const CONFIRM_OPTION_YES: &str = "Y";

/// Program structure.
pub struct Cli<'a> {
    engine: engine_core::Engine<'a>,
    version: &'static str,
    connected_db: Option<String>,
}

impl<'a> Cli<'a> {
    /// Builds program structure.
    pub fn build(
        config: &'a Config,
        logger: &'a engine_core::Logger
    ) -> Self {
        Self {
            engine: engine_core::Engine::build(config, logger),
            version: VERSION,
            connected_db: None,
        }
    }

    pub fn connected_db(&self) -> &Option<String> {
        &self.connected_db
    }
}

impl<'a> Cli<'a> {
    /// Displays the program version.
    fn display_version(&self) {
        println!("Client version: {}", &self.version);
        println!("Engine version: {}", &self.engine.version());
    }

    /// Displays connected database.
    fn display_connection_status(&self) {
        match &self.connected_db {
            Some(db_name) => println!("Connected database: {}", db_name),
            None => println!("{}", NO_CONNECTED_DB),
        }
    }
}

/// Runs the program.
pub fn run() {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configurations from config file: {}", e);
            println!("Restart the program to try again.\n");
            // Panic if config loading fails.
            panic!("Failed to load configs: {}", e)
        },
    };
    let logger = Logger::build(&config);
    let mut cli = Cli::build(&config, &logger);

    let help_message = "Write /help for all available commands";
    let mut connected_db_name;

    println!("NOTE: This is an early version. Nothing is final.");
    println!("The engine uses Protocol Buffers for storing data.");
    println!("\nVersion: {}", cli.version);
    println!("Database engine CLI client");
    println!("\n{}", help_message);

    // Program main loop
    loop {
        cli.refresh_connected_db();

        if let Some(name) = cli.connected_db() {
            connected_db_name = format!("Connected database: {}", name);
        } else {
            connected_db_name = NO_CONNECTED_DB.to_string();
        }

        let input_command = match ask_user_input(
            &format!("\n<{}>\nEnter a command: ", connected_db_name)
        ) {
            Ok(input_command) => input_command,
            Err(_) => continue,
        };

        match input_command.as_str() {
            "/help" => {
                println!("\n{}", "All available commands:");
                println!(
"
  /help                          List all available commands
  /q                             Quit program
  /status                        Display currently connected database
  /version                       Display client and engine versions

  ** DATABASE COMMANDS **

  /connect db                    Connect to a database
  /get dbs                       List all databases
  /create db                     Create a new database
  /delete db                     Delete a database
  /change db desc                Change description of the connected database

  ** COLLECTION COMMANDS **

  /get cols                      List all collections of the connected database
  /create col                    Creates a new collection to the connected database
  /delete col                    Deletes a collection from the connected database

  ** DOCUMENT COMMANDS **

  /get all docs                  List all documents of a collection
  /get docs                      List the first documents of a collection specified by limit
                                 This command will be changed in the future to support different kinds of queries.
  /get doc                       List a single document of a collection
  /create doc                    Create a new document to a collection
  /delete doc                    Delete a document from a collection

  ** CONFIG COMMANDS **

  /config get all                List all configurations
  /config get db_dir_path        Get directory where databases are created
  /config set db_dir_path        Set directory where databases will be created
  /config get logs_dir_path      Get directory where logs are created. 
  /config set logs_dir_path      Set directory where logs will be created.

  ** COMMANDS FOR TESTING **

  /create test docs            Creates test documents to a collection

More commands in the future...");
            },
            "/q" => {
                exit_program()
            },
            "/status" => {
                cli.display_connection_status();
            },
            "/version" => {
                cli.display_version();
            },
            "/connect db" => {
                cli.connect_database();
            },
            "/get dbs" => {
                cli.list_all_databases();
            },
            "/create db" => {
                cli.create_database();
            },
            "/delete db" => {
                cli.delete_database();
            },
            "/change db desc" => {
                cli.change_database_description();
            },
            "/get cols" => {
                cli.list_all_collections();
            },
            "/create col" => {
                cli.create_collection();
            },
            "/delete col" => {
                cli.delete_collection();
            },
            "/get all docs" => {
                cli.list_all_documents();
            },
            "/get docs" => {
                cli.list_documents();
            },
            "/get doc" => {
                cli.list_single_document();
            },
            "/create doc" => {
                cli.create_document();
            },
            "/delete doc" => {
                cli.delete_document();
            },
            "/config get all" => {
                config::list_all_configs(&config);
            },
            "/config set db_dir_path" => {
                config::set_db_dir_path(&config);
            },
            "/config set logs_dir_path" => {
                config::set_logs_dir_path(&config);
            },
            "/create test docs" => {
                cli.create_test_documents();
            },
            _ => {
                println!("Command not found!");
                println!("{}", help_message);
                continue
            },
        }
    }
}

/// Exits the program.
fn exit_program() {
    println!("Exiting...");
    process::exit(0);
}

/// Asks for user input and returns it trimmed.
fn ask_user_input(text_to_ask: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{text_to_ask}");
    io::stdout().flush().expect("Unexpected I/O error");
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read line: {e}");
        return Err(e);
    }
    let input = input.trim().to_string();

    Ok(input)
}

/// Asks user to confirm an action, such as delete action.
/// 
/// Returns the input trimmed.
fn ask_action_confirm(text_to_ask: &str) -> io::Result<String> {
    let mut confirm = String::new();

    println!("{text_to_ask}");
    print!("'Y' to confirm: ");
    io::stdout().flush().expect("Unexpected I/O error");
    if let Err(e) = io::stdin().read_line(&mut confirm) {
        eprintln!("Failed to read line: {e}");
        return Err(e);
    }
    let confirm = confirm.trim().to_string();

    Ok(confirm)
}



#[cfg(test)]
mod tests {
    
}

