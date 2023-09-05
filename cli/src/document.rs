use crate::{
    Cli,
    ask_user_input,
    ask_action_confirm,
    CONFIRM_OPTION_YES,
    db_not_connected,
    event_log_failed,
    error_log_failed,
};
use engine::{
    storage::{
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
    /// Show menu to create a new document to a collection.
    pub fn create_document(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };

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

        let result = self.engine
            .storage_api()
            .create_document(connected_db.file_path(), &collection_name, data);

        if result.success {
            event_log_failed(result.log_error);

            println!("Document created");
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }

    /// Show menu to replace a document.
    pub fn replace_document(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
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

        let result = self.engine
            .storage_api()
            .replace_document(connected_db.file_path(), &document_id, &collection_name, data);

        if result.success {
            event_log_failed(result.log_error);

            println!("Document replaced");
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }

    /// Show menu to delete a document.
    pub fn delete_document(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
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
            &format!("Delete document with ID '{}'?", document_id)
        ) {
            Ok(confirm) => confirm,
            Err(_) => return,
        };

        match confirm.as_str() {
            CONFIRM_OPTION_YES => {
                let result = self.engine
                    .storage_api()
                    .delete_document(connected_db.file_path(), &document_id, &collection_name);

                if result.success {
                    event_log_failed(result.log_error);

                    println!("Document deleted");
                } else {
                    error_log_failed(result.log_error);

                    if let Some(e) = result.error {
                        eprintln!("Error: {}", e);
                    }
                }
            },
            _ => return println!("Canceled action"),
        }
    }

    /// Show menu to list all documents of a collection.
    pub fn list_all_documents(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };
        let result = self.engine
            .storage_api()
            .find_all_documents(connected_db.file_path(), &collection_name);

        if result.success {
            event_log_failed(result.log_error);

            if let Some(documents) = result.data {
                println!("\nNumber of documents: {}", documents.len());

                for document in documents {
                    display_document(&document);
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }

    /// Show menu to list specific documents of a collection.
    pub fn list_documents(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };
        let limit = match ask_user_input("Limit: ") {
            Ok(limit) => limit,
            Err(_) => return,
        };
        let limit: usize = match limit.parse() {
            Ok(limit) => limit,
            Err(e) => return eprintln!("Invalid limit. Limit must be a positive integer: {e}"),
        };
        let result = self.engine
            .storage_api()
            .find_documents_limit(connected_db.file_path(), &collection_name, limit);

        if result.success {
            event_log_failed(result.log_error);

            if let Some(documents) = result.data {
                println!("\nNumber of documents: {}", documents.len());

                for document in documents {
                    display_document(&document);
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }

    /// Show menu to list a single document of a collection.
    pub fn list_single_document(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
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
        let result = self.engine
            .storage_api()
            .find_document_by_id(&document_id, connected_db.file_path(), &collection_name);

        if result.success {
            event_log_failed(result.log_error);

            if let Some(data) = result.data {
                if let Some(document) = data {
                    display_document(&document);
                } else {
                    println!("Document with this ID was not found");
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }

    /// Creates test documents to a collection.
    /// 
    /// Asks the number of documents to create.
    /// 
    /// WARNING! Be aware that there is no batch creating yet.
    /// This method will create each document individually!
    pub fn create_test_documents(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };
        let count = match ask_user_input("Count: ") {
            Ok(count) => count,
            Err(_) => return,
        };
        let count: usize = match count.parse() {
            Ok(count) => count,
            Err(e) => return eprintln!("Invalid document count: {e}"),
        };
        let mut document_count = 0;

        for i in 1..=count {
            let mut data: Vec<DocumentInputDataField> = Vec::new();
            let field = format!("field_{i}");
            let data_type = "Text";
            let value = format!("value_{i}");

            data.push(DocumentInputDataField::new(&field, data_type, &value));

            let result = self.engine
                .storage_api()
                .create_document(connected_db.file_path(), &collection_name, data);

            if result.success {
                event_log_failed(result.log_error);

                println!("Document created");
                document_count += 1;
            } else {
                error_log_failed(result.log_error);

                if let Some(e) = result.error {
                    eprintln!("Error: {}", e);
                }
            }
        }

        println!("Created {} documents to collection '{}'", document_count, collection_name);
    }
}
