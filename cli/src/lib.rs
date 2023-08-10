// #![allow(unused)]

mod database;
mod collection;
mod document;
pub mod util;
pub mod config;

use std::{
    process,
    path::{
        PathBuf,
        Path,
    },
};
use engine::{
    self,
    config::{
        Config,
        config_manager::ConfigManager,
    },
    Logger,
};
use util::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NO_CONNECTED_DB: &str = "No connected database";
const CONFIRM_OPTION_YES: &str = "Y";

/// Program structure.
pub struct Cli<'a> {
    engine: engine::Engine<'a>,
    version: &'static str,
    connected_db: Option<ConnectedDatabase>,
}

impl<'a> Cli<'a> {
    /// Builds program structure.
    pub fn build(
        config: &'a Config,
        logger: &'a engine::Logger,
    ) -> Self {
        Self {
            engine: engine::Engine::build(config, logger),
            version: VERSION,
            connected_db: None,
        }
    }

    pub fn connected_db(&self) -> &Option<ConnectedDatabase> {
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
            Some(db) => {
                println!("Connected database: {}", db.name());
                println!("File path: {}", db.file_path().display());
            },
            None => db_not_connected(),
        }
    }
}

/// Represents the connected database.
/// 
/// Holds its name and file path.
pub struct ConnectedDatabase {
    name: String,
    file_path: PathBuf,
}

impl ConnectedDatabase {
    fn name(&self) -> &str {
        &self.name
    }

    fn file_path(&self) -> &Path {
        &self.file_path
    }

    fn new(name: &str, file_path: &Path) -> Self {
        Self {
            name: String::from(name),
            file_path: PathBuf::from(file_path),
        }
    }
}

/// Runs the program.
pub fn run() {
    let config = match ConfigManager::load_config() {
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
    println!("\nVersion: {}", cli.version);
    println!("Database engine CLI client");
    println!("\n{}", help_message);

    // Program main loop
    loop {
        cli.refresh_connected_db();

        if let Some(db) = cli.connected_db() {
            connected_db_name = format!("Connected database: {}", db.name());
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

  /connect db name               Connect to a database by its name. Tries to find it in the database directory.
  /connect db file_path          Connect to a database by its file path.
  /get dbs                       List all databases
  /create db                     Create a new database
  /delete db                     Delete the connected database
  /change db desc                Change description of the connected database

  ** COLLECTION COMMANDS **

  /get cols                      List all collections of the connected database
  /create col                    Create a new collection to the connected database
  /delete col                    Delete a collection and all its documents from the connected database

  ** DOCUMENT COMMANDS **

  /get all docs                  List all documents of a collection
  /get docs                      List the first documents of a collection specified by limit
                                 This command will be changed in the future to support different kinds of queries.
  /get doc                       List a single document of a collection
  /create doc                    Create a new document to a collection
  /replace doc                   Replace a document with new data
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
            "/connect db name" => {
                cli.connect_db_by_name();
            },
            "/connect db file_path" => {
                cli.connect_db_by_file_path();
            }
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
            "/replace doc" => {
                cli.replace_document();
            },
            "/delete doc" => {
                cli.delete_document();
            },
            "/config get all" => {
                Cli::list_all_configs(&config);
            },
            "/config set db_dir_path" => {
                cli.set_db_dir_path();
            },
            "/config set logs_dir_path" => {
                cli.set_logs_dir_path();
            },
            "/config get db_dir_path" => {
                println!("{}", config.db_dir_path().display());
            },
            "/config get logs_dir_path" => {
                println!("{}", config.logs_dir_path().display());
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



#[cfg(test)]
mod tests {
    
}

