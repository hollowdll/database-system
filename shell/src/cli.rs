use engine::{
    self,
    config::{
        Config,
        get_config_file_path,
        load_config,
    },
};
use crate::{
    util::*,
    database::{
        ConnectedDatabase,
        NO_CONNECTED_DB,
    },
};

/// Client version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The input to give to confirm Y/N questions.
pub const CONFIRM_OPTION_YES: &str = "Y";

/// Program structure.
pub struct Cli {
    pub engine: engine::Engine,
    pub version: &'static str,
    pub connected_db: Option<ConnectedDatabase>,
}

impl Cli {
    /// Builds program structure.
    pub fn build(config: &Config) -> Self {
        Self {
            engine: engine::Engine::build(config),
            version: VERSION,
            connected_db: None,
        }
    }

    pub fn connected_db(&self) -> &Option<ConnectedDatabase> {
        &self.connected_db
    }
}

impl Cli {
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

/// Runs the program.
pub fn run() {
    let config = match load_config(&get_config_file_path()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configurations from config file: {}", e);
            println!("Restart the program to try again.\n");
            // Panic if config loading fails.
            panic!("Failed to load configs: {}", e)
        },
    };
    let mut cli = Cli::build(&config);
    let help_message = "Write /help for all available commands";
    let mut connected_db_name;

    println!("Database shell");
    println!("Version: {}", cli.version);
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
  /q, exit                       Quit program
  /status                        Display currently connected database
  /version                       Display client and engine versions

  ** DATABASE COMMANDS **

  /connect db name               Connect to a database by its name. Tries to find it in the database directory
  /connect db file_path          Connect to a database by its file path
  /get dbs                       List all databases
  /create db                     Create a new database
  /delete db                     Delete the connected database
  /change db desc                Change description of the connected database
  /db details                    Show details of the connected database

  ** COLLECTION COMMANDS **

  /get collections               List all collections in the connected database
  /create collection             Create a new collection to the connected database
  /delete collection             Delete a collection from the connected database. The collection has to be empty

  ** DOCUMENT COMMANDS **

  /get all documents             List all documents in a collection
  /get documents                 List the first documents in a collection specified by limit
                                 This command will be changed in the future to support different kinds of queries
  /get document                  List a single document in a collection
  /create document               Create a new document to a collection
  /replace document              Replace a document with new data
  /delete document               Delete a document from a collection
  /delete all documents          Delete all documents from a collection

  ** CONFIG COMMANDS **

  /config get all                List all configurations
  /config get db_dir_path        Get directory where databases are created
  /config set db_dir_path        Set directory where databases will be created
  /config get logs_dir_path      Get directory where logs are created
  /config set logs_dir_path      Set directory where logs will be created

More commands in the future...");
            },
            "/q" => {
                exit_program();
            },
            "exit" => {
                exit_program();
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
            "/db details" => {
                cli.show_database_details();
            }
            "/get collections" => {
                cli.list_all_collections();
            },
            "/create collection" => {
                cli.create_collection();
            },
            "/delete collection" => {
                cli.delete_collection();
            },
            "/get all documents" => {
                cli.list_all_documents();
            },
            "/get documents" => {
                cli.list_documents();
            },
            "/get document" => {
                cli.list_single_document();
            },
            "/create document" => {
                cli.create_document();
            },
            "/replace document" => {
                cli.replace_document();
            },
            "/delete document" => {
                cli.delete_document();
            },
            "/delete all documents" => {
                cli.delete_all_documents();
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
            _ => {
                println!("Command not found\n{}", help_message);
                continue
            },
        }
    }
}