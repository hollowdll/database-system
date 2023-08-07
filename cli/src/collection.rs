use crate::{
    Cli,
    ask_user_input,
    ask_action_confirm,
    CONFIRM_OPTION_YES,
    ConnectedDatabase,
    db_not_connected,
};

impl<'a> Cli<'a> {
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
            if let Some(e) = result.log_error {
                eprintln!("Failed to log event: {}", e);
            }

            if let Some(collection) = result.data {
                if collection.is_none() {
                    println!("Cannot find collection '{collection_name}'");
                    return false;
                }
            }
        } else {
            if let Some(e) = result.log_error {
                eprintln!("Failed to log error: {}", e);
            }

            eprintln!("Failed to find collection: {}", result.message);
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
        let collection_name = match ask_user_input("Collection: ") {
            Ok(collection_name) => collection_name,
            Err(_) => return,
        };

        if !&self.database_exists(connected_db) {
            return;
        }

        let result = self.engine
            .storage_api()
            .create_collection(&collection_name, connected_db.file_path());

        if result.success {
            if let Some(e) = result.log_error {
                eprintln!("Failed to log event: {}", e);
            }

            println!("Collection created");
        } else {
            if let Some(e) = result.log_error {
                eprintln!("Failed to log error: {}", e);
            }

            eprintln!("Failed to create collection: {}", result.message);
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
                if !&self.database_exists(connected_db) {
                    return;
                }

                let result = self.engine
                    .storage_api()
                    .delete_collection(&collection_name, connected_db.file_path());

                if result.success {
                    if let Some(e) = result.log_error {
                        eprintln!("Failed to log event: {}", e);
                    }

                    println!("Collection deleted");
                } else {
                    if let Some(e) = result.log_error {
                        eprintln!("Failed to log error: {}", e);
                    }

                    eprintln!("Failed to delete collection: {}", result.message);
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

        if !&self.database_exists(connected_db) {
            return;
        }

        let result = self.engine
            .storage_api()
            .find_all_collections(connected_db.file_path());

        if result.success {
            if let Some(e) = result.log_error {
                eprintln!("Failed to log event: {}", e);
            }

            if let Some(collections) = result.data {
                println!("\nNumber of collections: {}", collections.len());

                for collection in collections {
                    println!("{}", collection.name());
                }
            }
        } else {
            if let Some(e) = result.log_error {
                eprintln!("Failed to log error: {}", e);
            }

            eprintln!("Failed to list collections: {}", result.message);
        }
    }
}
