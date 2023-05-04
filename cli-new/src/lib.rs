// CLI management system library

// #![allow(unused)]

mod constants;

use std::{
    process,
    io::{self, Write},
};
use engine_core::{
    self,
    DatabaseManager,
    DataType,
    InputDataField,
};
use constants::NO_CONNECTED_DATABASE_TEXT;

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

/// Runs the program.
pub fn run(config: Config) {
    let engine = config.engine_core_config;
    let mut connected_database = config.connected_database;
    let help_message = "Write /help for all available commands";

    println!("{}", "NOTE: You are using the newer version of this program - 'cli-new' is the crate name");
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
  
  /documents                           List documents of a collection
  /create document                     Create a new document to a collection
  /delete document                     Delete a document from a database
  (DISABLED) /delete doc col           Delete a document from a collection. This is faster if the collection is known.

  ** COMMANDS FOR TESTING **

  /create test log                     Creates test log
  /create test documents               Creates test documents to a collection
  
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
            },
            "/documents" => {
                list_documents_of_collection(engine.database_manager(), &connected_database);
            }
            "/create document" => {
                create_document_menu(engine.database_manager(), &connected_database);
            },
            "/delete document" => {
                delete_document_menu(engine.database_manager(), &connected_database);
            },
            "/create test log" => {
                use engine_core::logs;
                for _ in 0..5 {
                    if let Err(e) = logs::create_test_log() {
                        eprintln!("Error: Failed to create test log. {e}");
                    }
                }
            },
            "/create test documents" => {
                create_test_documents(engine.database_manager(), &connected_database);
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

/// If connected database doesn't exists anymore, reset it to `None`.
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

/// Checks if connected database exists.
fn database_exists(
    database_manager: &DatabaseManager,
    connected_database_name: &str,
) -> bool
{
    match database_manager.find_database(connected_database_name) {
        Ok(result) => {
            if !result {
                println!("Cannot find database '{connected_database_name}'");
                return false;
            }
        },
        Err(e) => {
            eprintln!("Error occurred while trying to find connected database: {e}");
            return false;
        },
    }

    return true;
}

/// Checks if collection exists.
fn collection_exists(
    database_manager: &DatabaseManager,
    collection_name: &str,
    connected_database_name: &str,
) -> bool
{
    match database_manager.find_collection(collection_name, connected_database_name) {
        Ok(result) => {
            if !result {
                println!("Cannot find collection '{collection_name}'");
                return false;
            }
        },
        Err(e) => {
            eprintln!("Error occurred while trying to find collection: {e}");
            return false;
        },
    }

    return true;
}

/// Asks for user input and returns it trimmed.
fn ask_user_input(input_name: &str) -> io::Result<String> {
    let mut input = String::new();

    println!("\n{}", input_name);
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read line: {e}");
        return Err(e);
    }
    let input = input.trim().to_string();

    Ok(input)
}

/// Display connected database.
fn display_connection_status(connected_database: &Option<String>) {
    match connected_database {
        Some(database_name) => println!("Connected database: {database_name}"),
        None => println!("{}", NO_CONNECTED_DATABASE_TEXT),
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
        _ => return println!("Canceled database deletion"),
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
        None => return println!("{}", NO_CONNECTED_DATABASE_TEXT),
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
        None => return println!("{}", NO_CONNECTED_DATABASE_TEXT),
    };

    let mut confirm = String::new();
    println!("Are you sure you want to delete collection '{}'?", collection_name);
    print!("'Y' to confirm: ");
    io::stdout().flush().unwrap();
    if let Err(e) = io::stdin().read_line(&mut confirm) {
        return eprintln!("Failed to read line: {e}");
    }
    let confirm = confirm.trim();

    match confirm {
        "Y" => {
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
        },
        _ => return println!("Canceled collection deletion"),
    }

}

/// List all collections of the connected database
fn list_collections_of_connected_database(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE_TEXT),
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

    println!("\nNumber of collections: {}", collections.len());

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
        None => return println!("{}", NO_CONNECTED_DATABASE_TEXT),
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
        None => return println!("{}", NO_CONNECTED_DATABASE_TEXT),
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
    println!("\n{}", "Insert data");
    let mut data: Vec<InputDataField> = Vec::new();

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

        // Insert field to data
        data.push(InputDataField::from(field, data_type, value));

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
    match database_manager.create_document(connected_database_name, collection_name, data) {
        Ok((_result, message)) => println!("{message}"),
        Err(e) => return eprintln!("Error occurred while trying to create a document: {e}"),
    }

}

/// List all documents of a collection
fn list_documents_of_collection(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE_TEXT),
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

    // If connected database exists
    match database_manager.find_database(connected_database_name) {
        Ok(result) => {
            if !result {
                return println!("Cannot find database '{connected_database_name}'");
            }
        },
        Err(e) => return eprintln!("Error occurred while trying to find connected database: {e}"),
    }

    // find all documents and list them
    let documents = match database_manager.find_all_documents_of_collection(
        connected_database_name,
        collection_name
    ) {
        Ok(documents) => documents,
        Err(e) => return eprintln!("Error occurred while trying to find documents: {e}"),
    };

    println!("\nNumber of documents: {}", documents.len());

    for document in documents {
        println!("{}\n  id: {}", "{", document.id());
        for (key, value) in document.data().iter() {
            // Get data type and value
            let (data_type, field_value) = match value {
                DataType::Int32(value) => ("Int32", value.to_string()),
                DataType::Int64(value) => ("Int64", value.to_string()),
                DataType::Decimal(value) => ("Decimal", value.to_string()),
                DataType::Bool(value) => ("Bool", value.to_string()),
                DataType::Text(value) => ("Text", value.to_string()),
            };

            println!("  [{data_type}] {key}: {field_value}");
        }
        println!("{}", "}");
    }
}

/// Show menu to delete a document
fn delete_document_menu(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE_TEXT),
    };

    let mut document_id = String::new();
    print!("Document ID: ");
    io::stdout().flush().expect("Unexpected I/O error");
    if let Err(e) = io::stdin().read_line(&mut document_id) {
        return eprintln!("Failed to read line: {e}");
    }
    let document_id: u64 = match document_id.trim().parse() {
        Ok(id) => id,
        Err(e) => return eprintln!("Failed to parse input data into positive integer: {e}"),
    };

    let mut confirm = String::new();
    println!("Are you sure you want to delete document with ID '{}'?", document_id);
    print!("'Y' to confirm: ");
    io::stdout().flush().expect("Unexpected I/O error");
    if let Err(e) = io::stdin().read_line(&mut confirm) {
        return eprintln!("Failed to read line: {e}");
    }
    let confirm = confirm.trim();

    match confirm {
        "Y" => {
            match database_manager.find_database(connected_database_name) {
                Ok(result) => {
                    if !result {
                        return println!("Cannot find database '{connected_database_name}'");
                    }
                },
                Err(e) => return eprintln!("Error occurred while trying to find connected database: {e}"),
            }
        
            match database_manager.delete_document(connected_database_name, &document_id) {
                Ok((_result, message)) => println!("{message}"),
                Err(e) => return eprintln!("Error occurred while trying to delete a collection: {e}"),
            }
        },
        _ => return println!("Canceled document deletion"),
    }
}

/// Creates test documents to a collection
fn create_test_documents(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) -> bool
{
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => {
            println!("{}", NO_CONNECTED_DATABASE_TEXT);
            return false;
        },
    };

    let collection_name = match ask_user_input("Collection name:") {
        Ok(collection_name) => collection_name,
        Err(_e) => return false,
    };
    let collection_name = collection_name.as_str();

    if !collection_exists(database_manager, collection_name, connected_database_name) {
        return false;
    }
    
    if !database_exists(database_manager, connected_database_name) {
        return false;
    }
    
    for i in 1..=10 {
        let mut data: Vec<InputDataField> = Vec::new();
        let field = format!("field_{i}");
        let data_type = "Text";
        let value = format!("value_{i}");

        data.push(InputDataField::from(field.as_str(), data_type, value.as_str()));

        match database_manager.create_document(connected_database_name, collection_name, data) {
            Ok((_result, message)) => println!("{message}"),
            Err(e) => eprintln!("Error occurred while trying to create a document: {e}"),
        }
    }

    return true;
}
