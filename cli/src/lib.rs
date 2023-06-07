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
    db::{
        DataType,
        FormattedDocument,
    },
    InputDataField,
};
use constants::{
    NO_CONNECTED_DATABASE,
    CONFIRM_OPTION_YES,
};

// CLI version
const VERSION: &str = env!("CARGO_PKG_VERSION");

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
            version: VERSION,
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

    println!("{}", "NOTE: Nothing is final in the current version. Colored text might be added later.");
    println!("Version: {}", config.version);
    println!("\n{}\n", "Database Engine Project ");
    println!("{}", help_message);

    // Program main loop
    loop {
        let mut connected_database_name: String = NO_CONNECTED_DATABASE.to_string();

        refresh_connected_database(engine.database_manager(), &mut connected_database);

        if let Some(name) = &connected_database {
            connected_database_name = format!("Connected database: {}", name);
        }

        let input_command = match ask_user_input(
            &format!("<{connected_database_name}>\nEnter a command:")
        ) {
            Ok(input_command) => input_command,
            Err(_) => continue,
        };

        match input_command.as_str() {
            "/help" => {
                println!("\n{}", "All available commands:");
                println!(
"
  /help                                List all available commands
  /q                                   Quit program
  /status                              Display currently connected database
  /version                             Display software version

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

  ** DOCUMENT COMMANDS **
  
  /documents                           List documents of a collection
  /get document                        Fetch a document from a database by id and list it
  /create document                     Create a new document to a collection
  /delete document                     Delete a document from a database
  (DISABLED) /delete doc col           Delete a document from a collection. This is faster if the collection is known beforehand.

  ** COMMANDS FOR TESTING **

  /create test documents               Creates test documents to a collection
  
  More commands in the future...");
            },
            "/q" => {
                exit_program()
            },
            "/status" => {
                display_connection_status(&connected_database);
            },
            "/version" => {
                display_program_version(config.version, engine.version());
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
            },
            "/create collection" => {
                create_collection_menu(engine.database_manager(), &connected_database);
            },
            "/delete collection" => {
                delete_collection_menu(engine.database_manager(), &connected_database);
            },
            "/documents" => {
                list_documents_of_collection(engine.database_manager(), &connected_database);
            },
            "/get document" => {
                list_document(engine.database_manager(), &connected_database);
            },
            "/create document" => {
                create_document_menu(engine.database_manager(), &connected_database);
            },
            "/delete document" => {
                delete_document_menu(engine.database_manager(), &connected_database);
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

/// Displays the program version.
fn display_program_version(client_version: &str, engine_version: &str) {
    println!("Client version: {}", client_version);
    println!("Engine version: {}", engine_version);
}

/// Display connected database.
fn display_connection_status(connected_database: &Option<String>) {
    match connected_database {
        Some(database_name) => println!("Connected database: {database_name}"),
        None => println!("{}", NO_CONNECTED_DATABASE),
    }
}

/// Display formatted document in more readable format
fn display_formatted_document(document: &FormattedDocument) {
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
        Err(e) => eprintln!("Error occurred while trying to find connected database: {e}"),
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
fn ask_user_input(text_to_ask: &str) -> io::Result<String> {
    let mut input = String::new();

    println!("\n{}", text_to_ask);
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read line: {e}");
        return Err(e);
    }
    let input = input.trim().to_string();

    Ok(input)
}

/// Asks for user input and returns it trimmed.
/// 
/// This is inline version meaning that `text_to_ask`
/// and input are in the same line.
fn ask_user_input_inline(text_to_ask: &str) -> io::Result<String> {
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

/// Show menu to create a new database.
fn create_database_menu(database_manager: &DatabaseManager) {
    let database_name = match ask_user_input("Database name:") {
        Ok(database_name) => database_name,
        Err(_) => return,
    };

    match database_manager.create_database(&database_name) {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Failed to create database: {e}"),
    }
}

/// Show menu to delete a database.
fn delete_database_menu(
    database_manager: &DatabaseManager,
    connected_database: &mut Option<String>
) {
    let database_name = match ask_user_input("Database name:") {
        Ok(database_name) => database_name,
        Err(_) => return,
    };

    let confirm = match ask_action_confirm(
        &format!("Are you sure you want to delete database '{}'?", database_name)
    ) {
        Ok(confirm) => confirm,
        Err(_) => return,
    };

    match confirm.as_str() {
        CONFIRM_OPTION_YES => {
            match database_manager.delete_database(&database_name) {
                Ok((result, message)) => {
                    if result {
                        // Disconnect database if it is connected
                        if let Some(connected_database_name) = connected_database {
                            if connected_database_name == &database_name {
                                connected_database.take();
                            }
                        }
                    }
                    println!("{message}");
                },
                Err(e) => eprintln!("Error occurred: {e}"),
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
    let database_name = match ask_user_input("Database name:") {
        Ok(database_name) => database_name,
        Err(_) => return,
    };

    match database_manager.find_database(&database_name) {
        Ok(result) => {
            if result {
                connected_database.replace(database_name);
                println!("Connected to database");
            } else {
                println!("Failed to connect to database. It might not exist.");
            }
        },
        Err(e) => eprintln!("Error occurred: {e}"),
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
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };
    
    let collection_name = match ask_user_input("Collection name:") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };

    if !database_exists(database_manager, connected_database_name) {
        return;
    }

    match database_manager.create_collection(&collection_name, connected_database_name) {
        Ok((_result, message)) => println!("{message}"),
        Err(e) => return eprintln!("Error occurred: {e}"),
    }
}

/// Show menu to delete a collection
/// from the connected database
fn delete_collection_menu(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    let collection_name = match ask_user_input("Collection name:") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };

    let confirm = match ask_action_confirm(
        &format!("Are you sure you want to delete collection '{}'?", collection_name)
    ) {
        Ok(confirm) => confirm,
        Err(_) => return,
    };

    match confirm.as_str() {
        CONFIRM_OPTION_YES => {
            if !database_exists(database_manager, connected_database_name) {
                return;
            }
            match database_manager.delete_collection(&collection_name, connected_database_name) {
                Ok((_result, message)) => println!("{message}"),
                Err(e) => return eprintln!("Error occurred: {e}"),
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
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    if !database_exists(database_manager, connected_database_name) {
        return;
    }

    // find all collections and list them
    let collections = match database_manager.find_all_collections_of_database(connected_database_name) {
        Ok(collections) => collections,
        Err(e) => return eprintln!("Error occurred: {e}"),
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
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    let description = match ask_user_input("Description:") {
        Ok(description) => description,
        Err(_) => return,
    };

    if !database_exists(database_manager, connected_database_name) {
        return;
    }

    // Change description of connected database
    match database_manager.change_database_description(connected_database_name, &description) {
        Ok((_result, message)) => println!("{message}"),
        Err(e) => return eprintln!("Error occurred: {e}"),
    }
}

/// Show menu to create a new document to a collection
fn create_document_menu(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    let collection_name = match ask_user_input("Collection name:") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };

    if !collection_exists(database_manager, &collection_name, connected_database_name) {
        return;
    }

    // input data for the new document
    let mut data: Vec<InputDataField> = Vec::new();
    
    loop {
        println!("\n{}", "Insert new field");

        let field = match ask_user_input_inline("Field name: ") {
            Ok(field) => field,
            Err(_) => return,
        };
        let data_type = match ask_user_input_inline("Data type: ") {
            Ok(data_type) => data_type,
            Err(_) => return,
        };
        let value = match ask_user_input_inline("Value: ") {
            Ok(value) => value,
            Err(_) => return,
        };

        data.push(InputDataField::from(&field, &data_type, &value));

        let confirm = match ask_action_confirm("Stop inserting data and save this document?") {
            Ok(confirm) => confirm,
            Err(_) => return,
        };
        if confirm.as_str() == CONFIRM_OPTION_YES {
            break;
        }
    }

    if !database_exists(database_manager, connected_database_name) {
        return;
    }

    match database_manager.create_document(connected_database_name, &collection_name, data) {
        Ok((_result, message)) => println!("{message}"),
        Err(e) => return eprintln!("Error occurred: {e}"),
    }
}

/// List all documents of a collection
fn list_documents_of_collection(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };
    let collection_name = match ask_user_input("Collection name:") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };

    if !collection_exists(database_manager, &collection_name, connected_database_name) {
        return;
    }
    if !database_exists(database_manager, connected_database_name) {
        return;
    }

    let documents = match database_manager.find_all_documents_of_collection(
        connected_database_name,
        &collection_name,
    ) {
        Ok(documents) => documents,
        Err(e) => return eprintln!("Error occurred: {e}"),
    };

    println!("\nNumber of documents: {}", documents.len());

    for document in documents {
        display_formatted_document(&document);
    }
}

/// Lists document of a database
fn list_document(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };
    let document_id = match ask_user_input_inline("Document ID: ") {
        Ok(id) => id,
        Err(_) => return,
    };
    let document_id: u64 = match document_id.parse() {
        Ok(id) => id,
        Err(e) => return eprintln!("Invalid document ID: {e}"),
    };

    if !database_exists(database_manager, connected_database_name) {
        return;
    }

    let (result, message) = match database_manager.find_document_by_id(
        &document_id,
        connected_database_name
    ) {
        Ok((result, message)) => (result, message),
        Err(e) => return eprintln!("Error occurred: {e}"),
    };

    match result {
        Some(document) => {
            println!("Collection: {}", document.collection());
            display_formatted_document(&document);
        },
        None => return println!("{message}"),
    }
}

/// Show menu to delete a document
fn delete_document_menu(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    let document_id = match ask_user_input_inline("Document ID: ") {
        Ok(document_id) => document_id,
        Err(_) => return,
    };
    let document_id: u64 = match document_id.parse() {
        Ok(id) => id,
        Err(e) => return eprintln!("Invalid document ID: {e}"),
    };

    let confirm = match ask_action_confirm(
        &format!("Are you sure you want to delete document with ID '{}'?", document_id)
    ) {
        Ok(confirm) => confirm,
        Err(_) => return,
    };

    match confirm.as_str() {
        CONFIRM_OPTION_YES => {
            if !database_exists(database_manager, connected_database_name) {
                return;
            }
            match database_manager.delete_document(connected_database_name, &document_id) {
                Ok((_result, message)) => println!("{message}"),
                Err(e) => return eprintln!("Error occurred: {e}"),
            }
        },
        _ => return println!("Canceled document deletion"),
    }
}

/// Creates test documents to a collection
fn create_test_documents(
    database_manager: &DatabaseManager,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    let collection_name = match ask_user_input("Collection name:") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };

    if !collection_exists(database_manager, &collection_name, connected_database_name) {
        return;
    }
    
    if !database_exists(database_manager, connected_database_name) {
        return;
    }
    
    for i in 1..=10 {
        let mut data: Vec<InputDataField> = Vec::new();
        let field = format!("field_{i}");
        let data_type = "Text";
        let value = format!("value_{i}");

        data.push(InputDataField::from(&field, data_type, &value));

        match database_manager.create_document(connected_database_name, &collection_name, data) {
            Ok((_result, message)) => println!("{message}"),
            Err(e) => eprintln!("Error occurred: {e}"),
        }
    }
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
