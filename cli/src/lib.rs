// #![allow(unused)]

mod database;
mod collection;
mod document;

use std::{
    process,
    io::{self, Write},
};
use engine_core;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NO_CONNECTED_DB: &str = "No connected database";
const CONFIRM_OPTION_YES: &str = "Y";

/// Program configuration.
pub struct Config {
    engine: engine_core::Config,
    version: &'static str,
    connected_db: Option<String>,
}

impl Config {
    /// Builds a new program configuration.
    pub fn build() -> Self {
        Self {
            engine: engine_core::Config::build(),
            version: VERSION,
            connected_db: None,
        }
    }
}

impl Config {
    pub fn connected_db(&self) -> &Option<String> {
        &self.connected_db
    }
}

/// Program structure.
pub struct Cli {
    config: Config,
}

impl Cli {
    /// Builds program structure.
    pub fn build() -> Self {
        Self {
            config: Config::build(),
        }
    }
}

impl Cli {
    /// Displays the program version.
    fn display_version(&self) {
        println!("Client version: {}", &self.config.version);
        println!("Engine version: {}", &self.config.engine.version());
    }

    /// Displays connected database.
    fn display_connection_status(&self) {
        match &self.config.connected_db {
            Some(db_name) => println!("Connected database: {}", db_name),
            None => println!("{}", NO_CONNECTED_DB),
        }
    }
}

/// Runs the program.
pub fn run(mut cli: Cli) {
    let help_message = "Write /help for all available commands";

    println!("NOTE: This is an early version. Nothing is final.");
    println!("The engine uses Protocol Buffers for storing data.");
    println!("\nVersion: {}", cli.config.version);
    println!("Database engine CLI client");
    println!("\n{}", help_message);

    // Program main loop
    loop {
        let mut connected_db_name = NO_CONNECTED_DB.to_string();

        cli.refresh_connected_db();

        if let Some(name) = cli.config.connected_db() {
            connected_db_name = format!("Connected database: {}", name);
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
  /help                        List all available commands
  /q                           Quit program
  /status                      Display currently connected database
  /version                     Display client and engine versions

  ** DATABASE COMMANDS **

  /connect db                  Connect to a database
  /get dbs                     List all databases
  /create db                   Create a new database
  /delete db                   Delete a database
  /change db desc              Change description of the connected database

  ** COLLECTION COMMANDS **

  /get cols                    List all collections of the connected database
  /create col                  Creates a new collection to the connected database
  /delete col                  Deletes a collection from the connected database

  ** DOCUMENT COMMANDS **

  /get docs                    List all documents of a collection
  /get doc                     List a single document of a collection
  /create doc                  Create a new document to a collection
  /delete doc                  Delete a document from a collection

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
                cli.connect_database_menu();
            },
            "/get dbs" => {
                cli.list_all_databases();
            },
            "/create db" => {
                cli.create_database_menu();
            },
            "/delete db" => {
                cli.delete_database_menu();
            },
            "/change db desc" => {
                cli.change_database_description_menu();
            },
            "/get cols" => {
                cli.list_collections_of_connected_db();
            },
            "/create col" => {
                cli.create_collection_menu();
            },
            "/delete col" => {
                cli.delete_collection_menu();
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
/// Returns input trimmed.
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
    use super::*;

    #[test]
    fn test_program_version() {
        let config = Config::build();
        assert_eq!(config.version, env!("CARGO_PKG_VERSION"));
    }
}

