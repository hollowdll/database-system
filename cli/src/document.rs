use crate::{
    Cli,
    ask_user_input,
    ask_action_confirm,
    CONFIRM_OPTION_YES,
    NO_CONNECTED_DB,
};
use engine_core::{
    db::{
        pb::document::data_type::DataType,
        document::DocumentDto,
    },
    DocumentInputDataField,
};

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

impl Cli {
    /// Show menu to create a new document to a collection
    pub fn create_document_menu(&self) {
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
    pub fn list_documents_of_collection(&self) {
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
    pub fn list_document(&self) {
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
    pub fn delete_document_menu(&self) {
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
    pub fn create_test_documents(&self) {
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
