// CLI management system library

#![allow(unused)]

use std::{
    process,
    io::{self, Write, Read},
    collections::HashMap,
};
use engine_core::{
    self,
    DatabaseManager,
    DataType,
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

        refresh_connected_database(engine.database_manager(), &mut connected_database);

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
  /status                              Display currently connected database

  ** DATABASE COMMANDS **

  /databases                           List all databases
  /create db                           Create a new database
  /delete db                           Delete a database
  /connect db                          Connect to a database
  /change db desc                      Change description of connected database

  ** COLLECTION COMMANDS **

  /collections                         List all collections of connected database
  /create collection                   Create a new collection to the connected database
  /delete collection                   Delete a collection from the connected database

  ** THESE COMMANDS ARE NOT FINAL **
  ** DOCUMENT COMMANDS **
  
  (DISABLED) /documents                List documents of a collection
  /create document                     Create a new document to a collection
  (DISABLED) /delete document          Delete a document from a collection

  ** COMMANDS FOR TESTING **

  /create test log                     Creates test log
  
  More commands in the future...");
            },
            "/q" => {
                exit_program()
            },
            "/status" => {
                display_connection_status(&connected_database);
            },
            "/databases" => {
                list_all_databases(engine.database_manager());
            },
            "/create db" => {
                create_database_menu(engine.database_manager());
            },
            "/delete db" => {
                delete_database_menu(engine.database_manager(), &mut connected_database);
            },
            "/connect db" => {
                connect_database_menu(engine.database_manager(), &mut connected_database);
            },
            "/change db desc" => {
                change_database_description_menu(engine.database_manager(), &connected_database)
            },
            "/collections" => {
                list_collections_of_connected_database(engine.database_manager(), &connected_database);
            }
            "/create collection" => {
                create_collection_menu(engine.database_manager(), &connected_database);
            },
            "/delete collection" => {
                delete_collection_menu(engine.database_manager(), &connected_database);
            }
            "/create document" => {
                create_document_menu(engine.database_manager(), &connected_database);
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

/// If connected database doesn't exists anymore, reset it to `None`
fn refresh_connected_database(
    database_manager: &DatabaseManager,
    connected_database: &mut Option<String>
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return,
    };

    match database_manager.find_database(connected_database_name) {
        Ok(result) => {
            if !result {
                connected_database.take();   
            }
        },
        Err(e) => eprintln!("Error occurred while trying to find database: {e}"),
    }
}

/// Display connected database.
fn display_connection_status(connected_database: &Option<String>) {
    match connected_database {
        Some(database_name) => println!("Connected database: {database_name}"),
        None => println!("No connected database"),
    }
}

/// Show menu to create a new database.
fn create_database_menu(database_manager: &DatabaseManager) {
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
fn delete_database_menu(
    database_manager: &DatabaseManager,
    connected_database: &mut Option<String>
) {
    let mut database_name = String::new();
    let mut confirm = String::new();

    println!("\n{}", "Database name:");
    if let Err(e) = io::stdin().read_line(&mut database_name) {
        return eprintln!("Failed to read line: {e}");
    }

    let database_name = database_name.trim();

    println!("Are you sure you want to delete database '{}'?", database_name);
    print!("'Y' to confirm: ");
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
                        // Disconnect database if it is connected
                        if let Some(connected_database_name) = connected_database {
                            if connected_database_name == database_name {
                                connected_database.take();
                            }
                        }
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
fn connect_database_menu(
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
  Size: {} bytes
  Description: {}",
        database.name(),
        database.size(),
        database.description(),
        );
    }
}

/// Show menu to create a new collection
/// to the connected database
fn create_collection_menu(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>
) {
    let mut collection_name = String::new();

    println!("\n{}", "Collection name:");
    if let Err(e) = io::stdin().read_line(&mut collection_name) {
        return eprintln!("Failed to read line: {e}");
    }

    let collection_name = collection_name.trim();

    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("No connected database. Connect to a database to create a collection"),
    };

    // Check if connected database exists
    match database_manager.find_database(connected_database_name) {
        Ok(result) => {
            if !result {
                return println!("Cannot find database '{connected_database_name}'");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to find connected database: {e}"),
    }

    // Create collection
    match database_manager.create_collection(collection_name, connected_database_name) {
        Ok(result) => {
            if result {
                println!("Created collection");
            } else {
                println!("Failed to create collection. Database might not exist or collection name already exists.");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to create a collection: {e}"),
    }
}

/// Show menu to delete a collection
/// from the connected database
fn delete_collection_menu(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>
) {
    let mut collection_name = String::new();

    println!("\n{}", "Collection name:");
    if let Err(e) = io::stdin().read_line(&mut collection_name) {
        return eprintln!("Failed to read line: {e}");
    }

    let collection_name = collection_name.trim();

    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("No connected database. Connect to a database to delete a collection"),
    };

    // Check if connected database exists
    match database_manager.find_database(connected_database_name) {
        Ok(result) => {
            if !result {
                return println!("Cannot find database '{connected_database_name}'");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to find connected database: {e}"),
    }

    match database_manager.delete_collection(collection_name, connected_database_name) {
        Ok(result) => {
            if result {
                println!("Deleted collection");
            } else {
                println!("Failed to delete collection. Database or collection might not exist.");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to delete a collection: {e}"),
    }
}

/// List all collections of the connected database
fn list_collections_of_connected_database(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("No connected database."),
    };

    // Check if connected database exists
    match database_manager.find_database(connected_database_name) {
        Ok(result) => {
            if !result {
                return println!("Cannot find database '{connected_database_name}'");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to find connected database: {e}"),
    }

    // find all collections and list them
    let collections = match database_manager.find_all_collections_of_database(connected_database_name) {
        Ok(collections) => collections,
        Err(e) => return eprintln!("Error occurred while trying to find collections: {e}"),
    };

    println!("\nNumber of collections: {}\n", collections.len());

    for collection in collections {
        println!("{}", collection.name());
    }
}

/// Show menu to change database description
fn change_database_description_menu(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("No connected database."),
    };

    let mut description = String::new();
    println!("\n{}", "Description:");
    if let Err(e) = io::stdin().read_line(&mut description) {
        return eprintln!("Failed to read line: {e}");
    }
    let description = description.trim();

    // Check if connected database exists
    match database_manager.find_database(connected_database_name) {
        Ok(result) => {
            if !result {
                return println!("Cannot find database '{connected_database_name}'");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to find connected database: {e}"),
    }

    // Change description of connected database
    match database_manager.change_database_description(connected_database_name, description) {
        Ok(result) => {
            if result {
                println!("Changed database description");
            } else {
                println!("Failed to change database description. Database might not exist.");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to change database description: {e}"),
    }
}

/// Show menu to create a new document
/// to a collection
fn create_document_menu(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("No connected database."),
    };

    let mut collection_name = String::new();
    println!("\n{}", "Collection name:");
    if let Err(e) = io::stdin().read_line(&mut collection_name) {
        return eprintln!("Failed to read line: {e}");
    }
    let collection_name = collection_name.trim();

    // If collection exists
    match database_manager.find_collection(collection_name, connected_database_name) {
        Ok(result) => {
            if !result {
                return println!("Cannot find collection '{collection_name}'");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to find collection: {e}"),
    }

    // data input
    println!("\n{}\n", "Insert data");
    let mut data: HashMap<String, DataType> = HashMap::new();

    /*
    let mut input_data = String::new();
    loop {
        let mut line = String::new();
        if let Err(e) = io::stdin().read_line(&mut line) {
            return eprintln!("Failed to read line: {e}");
        }
        let line = line.trim();

        if line == "end" {
            break;
        }

        input_data += line;
    }
    println!("{input_data}");*/

    loop {
        print!("Field name: ");
        io::stdout().flush().unwrap();
        let mut field = String::new();
        if let Err(e) = io::stdin().read_line(&mut field) {
            return eprintln!("Failed to read line: {e}");
        }
        let field = field.trim();

        print!("Data type: ");
        io::stdout().flush().unwrap();
        let mut data_type = String::new();
        if let Err(e) = io::stdin().read_line(&mut data_type) {
            return eprintln!("Failed to read line: {e}");
        }
        let data_type = data_type.trim();

        print!("Value: ");
        io::stdout().flush().unwrap();
        let mut value = String::new();
        if let Err(e) = io::stdin().read_line(&mut value) {
            return eprintln!("Failed to read line: {e}");
        }
        let value = value.trim();
        
        println!("{field} {data_type} {value}");
        println!("Type 'end' without quotes to stop inserting data and save this document");

        let mut end = String::new();
        if let Err(e) = io::stdin().read_line(&mut end) {
            return eprintln!("Failed to read line: {e}");
        }
        let end = end.trim();

        if end == "end" {
            break;
        }
    }

    // If connected database exists
    match database_manager.find_database(connected_database_name) {
        Ok(result) => {
            if !result {
                return println!("Cannot find database '{connected_database_name}'");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to find connected database: {e}"),
    }

    // create document

}
