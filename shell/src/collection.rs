use crate::{
    cli::Cli,
    ask_user_input,
    ask_action_confirm,
    cli::CONFIRM_OPTION_YES,
    database::ConnectedDatabase,
    db_not_connected,
    event_log_failed,
    error_log_failed,
};

impl Cli {
    /// Checks if collection exists.
    pub fn collection_exists(
        &self,
        collection_name: &str,
        connected_db: &ConnectedDatabase,
    ) -> bool
    {
        let result = self.engine
            .storage_api()
            .find_collection(collection_name, connected_db.file_path());

        if result.success {
            event_log_failed(result.log_error);

            if let Some(collection) = result.data {
                if collection.is_none() {
                    println!("Cannot find collection '{collection_name}'");
                    return false;
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
            return false;
        }

        return true;
    }

    /// Show menu to create a new collection to the connected database.
    pub fn create_collection(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };
        let collection_name = match ask_user_input("Collection name: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };
        let result = self.engine
            .storage_api()
            .create_collection(&collection_name, connected_db.file_path());

        if result.success {
            event_log_failed(result.log_error);

            println!("Collection created");
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }

    /// Show menu to delete a collection from the connected database.
    pub fn delete_collection(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };
        let confirm = match ask_action_confirm(
            &format!("Delete collection '{}'?", collection_name)
        ) {
            Ok(confirm) => confirm,
            Err(_) => return,
        };

        match confirm.as_str() {
            CONFIRM_OPTION_YES => {
                let result = self.engine
                    .storage_api()
                    .delete_collection(&collection_name, connected_db.file_path());

                if result.success {
                    event_log_failed(result.log_error);

                    println!("Collection deleted");
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

    /// List all collections of the connected database.
    pub fn list_all_collections(&self) {
        let connected_db = match &self.connected_db {
            Some(db) => db,
            None => return db_not_connected(),
        };
        let result = self.engine
            .storage_api()
            .find_all_collections(connected_db.file_path());

        if result.success {
            event_log_failed(result.log_error);

            if let Some(collections) = result.data {
                println!("Number of collections: {}", collections.len());

                for collection in collections {
                    println!("{}", collection.name());
                }
            }
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: {}", e);
            }
        }
    }
}
