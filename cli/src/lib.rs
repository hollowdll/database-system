// #![allow(unused)]

mod database;
mod collection;
mod document;

use std::{
    process,
    io::{self, Write},
};
use engine_core::{
    self,
    db::{
        pb::document::data_type::DataType,
        document::DocumentDto,
    },
    DocumentInputDataField,
};

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
    pub fn engine(&self) -> &engine_core::Config {
        &self.engine
    }

    pub fn connected_db(&self) -> &Option<String> {
        &self.connected_db
    }

    pub fn connected_db_mut(&mut self) -> &mut Option<String> {
        &mut self.connected_db
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
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
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

    /// If connected database doesn't exists anymore, reset it to `None`.
    fn refresh_connected_db(&mut self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return,
        };

        match &self.config.engine.api().find_database(connected_db_name) {
            Ok(result) => {
                if result.is_none() {
                    let _ = &self.config.connected_db.take();
                }
            },
            Err(e) => eprintln!("[Error] Failed to find connected database: {e}"),
        }
    }

    /// Checks if connected database exists.
    fn database_exists(
        &self,
        connected_db_name: &str,
    ) -> bool
    {
        match &self.config.engine.api().find_database(connected_db_name) {
            Ok(result) => {
                if result.is_none() {
                    println!("Cannot find database '{connected_db_name}'");
                    return false;
                }
            },
            Err(e) => {
                eprintln!("[Error] Failed to find connected database: {e}");
                return false;
            },
        }

        return true;
    }

    /// Checks if collection exists.
    fn collection_exists(
        &self,
        collection_name: &str,
        connected_db_name: &str,
    ) -> bool
    {
        match &self.config.engine.api().find_collection(collection_name, connected_db_name) {
            Ok(result) => {
                if result.is_none() {
                    println!("Cannot find collection '{collection_name}'");
                    return false;
                }
            },
            Err(e) => {
                eprintln!("[Error] Failed to find collection: {e}");
                return false;
            },
        }

        return true;
    }

    /// Show menu to connect to a database.
    fn connect_database_menu(&mut self) {
        let db_name = match ask_user_input("Database name: ") {
            Ok(db_name) => db_name,
            Err(_) => return,
        };

        match &self.config.engine.api().find_database(&db_name) {
            Ok(result) => {
                if result.is_some() {
                    let _ = &self.config.connected_db.replace(db_name);
                    println!("Connected to database");
                } else {
                    println!("Failed to connect to database. Database does not exist.");
                }
            },
            Err(e) => eprintln!("[Error] {e}"),
        }
    }

    /// Show menu to create a new database.
    fn create_database_menu(&self) {
        let db_name = match ask_user_input("Database name: ") {
            Ok(db_name) => db_name,
            Err(_) => return,
        };

        match &self.config.engine.api().create_database(&db_name) {
            Ok(()) => println!("Database created"),
            Err(err) => eprintln!("[Error] {}", err),
        }
    }

    /// Show menu to delete a database.
    fn delete_database_menu(&mut self) {
        let db_name = match ask_user_input("Database name: ") {
            Ok(db_name) => db_name,
            Err(_) => return,
        };

        let confirm = match ask_action_confirm(
            &format!("Are you sure you want to delete database '{}'?", db_name)
        ) {
            Ok(confirm) => confirm,
            Err(_) => return,
        };

        match confirm.as_str() {
            CONFIRM_OPTION_YES => {
                match &self.config.engine.api().delete_database(&db_name) {
                    Ok(()) => {
                        // Disconnect database if it is connected
                        if let Some(connected_db_name) = &self.config.connected_db {
                            if connected_db_name == &db_name {
                                let _ = &self.config.connected_db.take();
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
    fn list_all_databases(&self) {
        let databases = match self.config.engine.api().find_all_databases() {
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
    fn create_collection_menu(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
        };
        
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };

        if !&self.database_exists(connected_db_name) {
            return;
        }

        match &self.config.engine.api().create_collection(&collection_name, connected_db_name) {
            Ok(()) => println!("Collection created"),
            Err(e) => return eprintln!("[Error] {e}"),
        }
    }

    /// Show menu to delete a collection
    /// from the connected database
    fn delete_collection_menu(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
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
                if !&self.database_exists(connected_db_name) {
                    return;
                }
                match &self.config.engine.api().delete_collection(&collection_name, connected_db_name) {
                    Ok(()) => println!("Collection deleted"),
                    Err(e) => return eprintln!("[Error] {e}"),
                }
            },
            _ => return println!("Canceled action"),
        }

    }

    /// List all collections of the connected database
    fn list_collections_of_connected_db(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
        };

        if !&self.database_exists(connected_db_name) {
            return;
        }

        // find all collections and list them
        let collections = match self
            .config
            .engine
            .api()
            .find_all_collections(connected_db_name)
        {
            Ok(collections) => collections,
            Err(e) => return eprintln!("[Error] {e}"),
        };

        println!("\nNumber of collections: {}", collections.len());

        for collection in collections {
            println!("{}", collection.name());
        }
    }

    /// Show menu to change database description
    fn change_database_description_menu(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
        };

        let description = match ask_user_input("Description: ") {
            Ok(description) => description,
            Err(_) => return,
        };

        if !&self.database_exists(connected_db_name) {
            return;
        }

        // Change description of connected database
        match &self.config.engine.api().change_database_description(connected_db_name, &description) {
            Ok(()) => println!("Database description changed"),
            Err(e) => return eprintln!("[Error] {e}"),
        }
    }

    /// Show menu to create a new document to a collection
    fn create_document_menu(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
        };

        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };

        if !&self.collection_exists(&collection_name, connected_db_name) {
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

        if !&self.database_exists(connected_db_name) {
            return;
        }

        match &self.config.engine.api().create_document(connected_db_name, &collection_name, data) {
            Ok(()) => println!("Document created"),
            Err(e) => return eprintln!("[Error] {e}"),
        }
    }

    /// List all documents of a collection
    fn list_documents_of_collection(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
        };
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };

        if !&self.collection_exists(&collection_name, connected_db_name) {
            return;
        }
        if !&self.database_exists(connected_db_name) {
            return;
        }

        let documents = match self.config.engine.api().find_all_documents(
            connected_db_name,
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
    fn list_document(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
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

        if !&self.database_exists(connected_db_name) {
            return;
        }

        let result = match self.config.engine.api().find_document_by_id(
            &document_id,
            connected_db_name,
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
    fn delete_document_menu(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
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
                if !&self.database_exists(connected_db_name) {
                    return;
                }
                match &self.config.engine.api().delete_document(connected_db_name, &document_id, &collection_name) {
                    Ok(()) => println!("Document deleted"),
                    Err(e) => return eprintln!("[Error] {e}"),
                }
            },
            _ => return println!("Canceled action"),
        }
    }

    /// Creates test documents to a collection
    fn create_test_documents(&self) {
        let connected_db_name = match &self.config.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
        };

        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };

        if !&self.collection_exists(&collection_name, connected_db_name) {
            return;
        }
        
        if !&self.database_exists(connected_db_name) {
            return;
        }
        
        for i in 1..=10 {
            let mut data: Vec<DocumentInputDataField> = Vec::new();
            let field = format!("field_{i}");
            let data_type = "Text";
            let value = format!("value_{i}");

            data.push(DocumentInputDataField::new(&field, data_type, &value));

            match &self.config.engine.api().create_document(connected_db_name, &collection_name, data) {
                Ok(()) => println!("Document created"),
                Err(e) => eprintln!("[Error] {e}"),
            }
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
                cli.list_documents_of_collection();
            },
            "/get doc" => {
                cli.list_document();
            },
            "/create doc" => {
                cli.create_document_menu();
            },
            "/delete doc" => {
                cli.delete_document_menu();
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

/// Exit the program.
fn exit_program() {
    println!("Exiting...");
    process::exit(0);
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

