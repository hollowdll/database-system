use crate::{
    cli::Cli,
    ask_user_input,
    ask_action_confirm,
    cli::CONFIRM_OPTION_YES,
    db_not_connected,
    event_log_failed,
    error_log_failed,
};
use engine::DocumentInputDataField;

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

    /// Show menu to delete all documents.
    pub fn delete_all_documents(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };
        let confirm = match ask_action_confirm(
            &format!("Delete all documents in collection '{}'?", collection_name)
        ) {
            Ok(confirm) => confirm,
            Err(_) => return,
        };

        match confirm.as_str() {
            CONFIRM_OPTION_YES => {
                let result = self.engine
                    .storage_api()
                    .delete_all_documents(connected_db.file_path(), &collection_name);

                if result.success {
                    event_log_failed(result.log_error);
                    
                    if let Some(deleted_count) = result.data {
                        println!("Documents deleted: {}", deleted_count);
                    } else {
                        println!("Documents deleted");
                    }
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

    /// Show menu to list all documents in a collection.
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
                println!("Number of documents: {}", documents.len());

                for document in documents {
                    println!("{}", &document);
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }

    /// Show menu to list specific documents in a collection.
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
                println!("Number of documents: {}", documents.len());

                for document in documents {
                    println!("{}", &document);
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }

    /// Show menu to list a single document in a collection.
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
                    println!("{}", &document);
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

    /// Show menu to list documents in a collection using query.
    /// 
    /// The query contains data fields with values that the document needs to match.
    pub fn list_documents_query(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };
        let mut query: Vec<DocumentInputDataField> = Vec::new();
        
        println!("Specify fields that will be added to query");
        loop {
            let field = match ask_user_input("Field: ") {
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
            query.push(DocumentInputDataField::new(&field, &data_type, &value));

            let confirm = match ask_action_confirm("Field added to query. Stop inserting fields?") {
                Ok(confirm) => confirm,
                Err(_) => return,
            };
            if confirm.as_str() == CONFIRM_OPTION_YES {
                break;
            }
        }

        let result = self.engine
            .storage_api()
            .find_documents(connected_db.file_path(), &collection_name, &query);

        if result.success {
            event_log_failed(result.log_error);

            if let Some(documents) = result.data {
                println!("Number of documents: {}", documents.len());

                for document in documents {
                    println!("{}", &document);
                }
            } else {
                println!("No documents found");
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }
}
