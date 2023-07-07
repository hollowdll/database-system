// CLI client library

// #![allow(unused)]

mod constants;

use std::{
    process,
    io::{self, Write},
};
use engine_core::{
    self,
    EngineApi,
    db::{
        pb::document::data_type::DataType,
        document::DocumentDto,
    },
    DocumentInputDataField,
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

    println!("NOTE: This is an early version. Nothing is final.");
    println!("The engine uses Protocol Buffers for storing data.");
    println!("\nVersion: {}", config.version);
    println!("Database engine CLI client");
    println!("\n{}", help_message);

    // Program main loop
    loop {
        let mut connected_database_name: String = NO_CONNECTED_DATABASE.to_string();

        refresh_connected_database(engine.api(), &mut connected_database);

        if let Some(name) = &connected_database {
            connected_database_name = format!("Connected database: {}", name);
        }

        let input_command = match ask_user_input(
            &format!("\n<{}>\nEnter a command: ", connected_database_name)
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
  /version                             Display client and engine versions

  ** DATABASE COMMANDS **

  /connect db                          Connect to a database
  /get dbs                             List all databases
  /create db                           Create a new database
  /delete db                           Delete a database
  /change db desc                      Change description of the connected database

  ** COLLECTION COMMANDS **

  /get cols                            List all collections of the connected database
  /create col                          Creates a new collection to the connected database
  /delete col                          Deletes a collection from the connected database

  ** DOCUMENT COMMANDS **
  
  /get docs                            List all documents of a collection
  /get doc                             Fetch a document from a collection and list it
  /create doc                          Create a new document to a collection
  /delete doc                          Delete a document from a collection

  ** COMMANDS FOR TESTING **

  /create test docs                    Creates test documents to a collection
  
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
            "/connect db" => {
                connect_database_menu(engine.api(), &mut connected_database);
            },
            "/get dbs" => {
                list_all_databases(engine.api());
            },
            "/create db" => {
                create_database_menu(engine.api());
            },
            "/delete db" => {
                delete_database_menu(engine.api(), &mut connected_database);
            },
            "/change db desc" => {
                change_database_description_menu(engine.api(), &connected_database)
            },
            "/get cols" => {
                list_collections_of_connected_database(engine.api(), &connected_database);
            },
            "/create col" => {
                create_collection_menu(engine.api(), &connected_database);
            },
            "/delete col" => {
                delete_collection_menu(engine.api(), &connected_database);
            },
            "/get docs" => {
                list_documents_of_collection(engine.api(), &connected_database);
            },
            "/get doc" => {
                list_document(engine.api(), &connected_database);
            },
            "/create doc" => {
                create_document_menu(engine.api(), &connected_database);
            },
            "/delete doc" => {
                delete_document_menu(engine.api(), &connected_database);
            },
            "/create test docs" => {
                create_test_documents(engine.api(), &connected_database);
            },
            _ => {
                println!("Command not found!");
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

/// Displays connected database.
fn display_connection_status(connected_database: &Option<String>) {
    match connected_database {
        Some(database_name) => println!("Connected database: {database_name}"),
        None => println!("{}", NO_CONNECTED_DATABASE),
    }
}

/// Displays document in a more readable format.
fn display_document(document: &DocumentDto) {
    println!("{}\n  [DocumentId] _id: {}", "{", document.id());
    for (key, value) in document.data().iter() {
        // Get data type and value
        let (data_type, field_value) = match &value.data_type {
            Some(DataType::Int32(value)) => ("Int32", value.to_string()),
            Some(DataType::Int64(value)) => ("Int64", value.to_string()),
            Some(DataType::Decimal(value)) => ("Decimal", value.to_string()),
            Some(DataType::Bool(value)) => ("Bool", value.to_string()),
            Some(DataType::Text(value)) => ("Text", format!("\"{}\"", value)),
            _ => return eprintln!("Invalid document data type"),
        };

        println!("  [{data_type}] \"{key}\": {field_value}");
    }
    println!("{}", "}");
}

/// If connected database doesn't exists anymore, reset it to `None`.
fn refresh_connected_database(
    api: &EngineApi,
    connected_database: &mut Option<String>
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return,
    };

    match api.find_database(connected_database_name) {
        Ok(result) => {
            if result.is_none() {
                connected_database.take();   
            }
        },
        Err(e) => eprintln!("[Error] {e}"),
    }
}

/// Checks if connected database exists.
fn database_exists(
    api: &EngineApi,
    connected_database_name: &str,
) -> bool
{
    match api.find_database(connected_database_name) {
        Ok(result) => {
            if result.is_none() {
                println!("Cannot find database '{connected_database_name}'");
                return false;
            }
        },
        Err(e) => {
            eprintln!("[Error] {e}");
            return false;
        },
    }

    return true;
}

/// Checks if collection exists.
fn collection_exists(
    api: &EngineApi,
    collection_name: &str,
    connected_database_name: &str,
) -> bool
{
    match api.find_collection(collection_name, connected_database_name) {
        Ok(result) => {
            if result.is_none() {
                println!("Cannot find collection '{collection_name}'");
                return false;
            }
        },
        Err(e) => {
            eprintln!("[Error] {e}");
            return false;
        },
    }

    return true;
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

/// Show menu to connect to a database.
fn connect_database_menu(
    api: &EngineApi,
    connected_database: &mut Option<String>
) {
    let database_name = match ask_user_input("Database name: ") {
        Ok(database_name) => database_name,
        Err(_) => return,
    };

    match api.find_database(&database_name) {
        Ok(result) => {
            if result.is_some() {
                connected_database.replace(database_name);
                println!("Connected to database");
            } else {
                println!("Failed to connect to database. Database does not exist.");
            }
        },
        Err(e) => eprintln!("[Error] {e}"),
    }
}

/// Show menu to create a new database.
fn create_database_menu(api: &EngineApi) {
    let database_name = match ask_user_input("Database name: ") {
        Ok(database_name) => database_name,
        Err(_) => return,
    };

    match api.create_database(&database_name) {
        Ok(()) => println!("Database created"),
        Err(err) => eprintln!("[Error] {}", err),
    }
}

/// Show menu to delete a database.
fn delete_database_menu(
    api: &EngineApi,
    connected_database: &mut Option<String>
) {
    let database_name = match ask_user_input("Database name: ") {
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
            match api.delete_database(&database_name) {
                Ok(()) => {
                    // Disconnect database if it is connected
                    if let Some(connected_database_name) = connected_database {
                        if connected_database_name == &database_name {
                            connected_database.take();
                        }
                    }
                    println!("Database deleted");
                },
                Err(e) => eprintln!("[Error] {e}"),
            }
        },
        _ => return println!("Canceled action"),
    }
}

/// List all databases and display information about them.
fn list_all_databases(api: &EngineApi) {
    let databases = match api.find_all_databases() {
        Ok(databases) => databases,
        Err(e) => return eprintln!("[Error] {e}"),
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
    api: &EngineApi,
    connected_database: &Option<String>
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };
    
    let collection_name = match ask_user_input("Collection: ") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };

    if !database_exists(api, connected_database_name) {
        return;
    }

    match api.create_collection(&collection_name, connected_database_name) {
        Ok(()) => println!("Collection created"),
        Err(e) => return eprintln!("[Error] {e}"),
    }
}

/// Show menu to delete a collection
/// from the connected database
fn delete_collection_menu(
    api: &EngineApi,
    connected_database: &Option<String>
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    let collection_name = match ask_user_input("Collection: ") {
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
            if !database_exists(api, connected_database_name) {
                return;
            }
            match api.delete_collection(&collection_name, connected_database_name) {
                Ok(()) => println!("Collection deleted"),
                Err(e) => return eprintln!("[Error] {e}"),
            }
        },
        _ => return println!("Canceled action"),
    }

}

/// List all collections of the connected database
fn list_collections_of_connected_database(
    api: &EngineApi,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    if !database_exists(api, connected_database_name) {
        return;
    }

    // find all collections and list them
    let collections = match api.find_all_collections(connected_database_name) {
        Ok(collections) => collections,
        Err(e) => return eprintln!("[Error] {e}"),
    };

    println!("\nNumber of collections: {}", collections.len());

    for collection in collections {
        println!("{}", collection.name());
    }
}

/// Show menu to change database description
fn change_database_description_menu(
    api: &EngineApi,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    let description = match ask_user_input("Description: ") {
        Ok(description) => description,
        Err(_) => return,
    };

    if !database_exists(api, connected_database_name) {
        return;
    }

    // Change description of connected database
    match api.change_database_description(connected_database_name, &description) {
        Ok(()) => println!("Database description changed"),
        Err(e) => return eprintln!("[Error] {e}"),
    }
}

/// Show menu to create a new document to a collection
fn create_document_menu(
    api: &EngineApi,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    let collection_name = match ask_user_input("Collection: ") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };

    if !collection_exists(api, &collection_name, connected_database_name) {
        return;
    }

    // input data for the new document
    let mut data: Vec<DocumentInputDataField> = Vec::new();
    
    loop {
        println!("\n{}", "Insert new field");

        let field = match ask_user_input("Field name: ") {
            Ok(field) => field,
            Err(_) => return,
        };
        let data_type = match ask_user_input("Data type: ") {
            Ok(data_type) => data_type,
            Err(_) => return,
        };
        let value = match ask_user_input("Value: ") {
            Ok(value) => value,
            Err(_) => return,
        };

        data.push(DocumentInputDataField::new(&field, &data_type, &value));

        let confirm = match ask_action_confirm("Stop inserting data and save this document?") {
            Ok(confirm) => confirm,
            Err(_) => return,
        };
        if confirm.as_str() == CONFIRM_OPTION_YES {
            break;
        }
    }

    if !database_exists(api, connected_database_name) {
        return;
    }

    match api.create_document(connected_database_name, &collection_name, data) {
        Ok(()) => println!("Document created"),
        Err(e) => return eprintln!("[Error] {e}"),
    }
}

/// List all documents of a collection
fn list_documents_of_collection(
    api: &EngineApi,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };
    let collection_name = match ask_user_input("Collection: ") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };

    if !collection_exists(api, &collection_name, connected_database_name) {
        return;
    }
    if !database_exists(api, connected_database_name) {
        return;
    }

    let documents = match api.find_all_documents(
        connected_database_name,
        &collection_name,
    ) {
        Ok(documents) => documents,
        Err(e) => return eprintln!("[Error] {e}"),
    };

    println!("\nNumber of documents: {}", documents.len());

    for document in documents {
        display_document(&document);
    }
}

/// Lists document of a database
fn list_document(
    api: &EngineApi,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };
    let collection_name = match ask_user_input("Collection: ") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };
    let document_id = match ask_user_input("Document ID: ") {
        Ok(id) => id,
        Err(_) => return,
    };
    let document_id: u64 = match document_id.parse() {
        Ok(id) => id,
        Err(e) => return eprintln!("Invalid document ID: {e}"),
    };

    if !database_exists(api, connected_database_name) {
        return;
    }

    let result = match api.find_document_by_id(
        &document_id,
        connected_database_name,
        &collection_name,
    ) {
        Ok(result) => result,
        Err(e) => return eprintln!("[Error] {e}"),
    };

    match result {
        Some(document) => {
            println!("Collection: {}", document.collection());
            display_document(&document);
        },
        None => return println!("Document with this ID was not found from this collection"),
    }
}

/// Show menu to delete a document
fn delete_document_menu(
    api: &EngineApi,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };
    let collection_name = match ask_user_input("Collection: ") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };
    let document_id = match ask_user_input("Document ID: ") {
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
            if !database_exists(api, connected_database_name) {
                return;
            }
            match api.delete_document(connected_database_name, &document_id, &collection_name) {
                Ok(()) => println!("Document deleted"),
                Err(e) => return eprintln!("[Error] {e}"),
            }
        },
        _ => return println!("Canceled action"),
    }
}

/// Creates test documents to a collection
fn create_test_documents(
    api: &EngineApi,
    connected_database: &Option<String>,
) {
    let connected_database_name = match connected_database {
        Some(database_name) => database_name,
        None => return println!("{}", NO_CONNECTED_DATABASE),
    };

    let collection_name = match ask_user_input("Collection: ") {
        Ok(collection_name) => collection_name,
        Err(_) => return,
    };

    if !collection_exists(api, &collection_name, connected_database_name) {
        return;
    }
    
    if !database_exists(api, connected_database_name) {
        return;
    }
    
    for i in 1..=10 {
        let mut data: Vec<DocumentInputDataField> = Vec::new();
        let field = format!("field_{i}");
        let data_type = "Text";
        let value = format!("value_{i}");

        data.push(DocumentInputDataField::new(&field, data_type, &value));

        match api.create_document(connected_database_name, &collection_name, data) {
            Ok(()) => println!("Document created"),
            Err(e) => eprintln!("[Error] {e}"),
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

