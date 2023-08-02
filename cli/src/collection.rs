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
        match &self.engine
            .storage_api()
            .find_collection(collection_name, connected_db.file_path())
        {
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

        match &self.engine
            .storage_api()
            .create_collection(&collection_name, connected_db.file_path())
        {
            Ok(()) => println!("Collection created"),
            Err(e) => return eprintln!("[Error] Failed to create collection: {e}"),
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
                match &self.engine
                    .storage_api()
                    .delete_collection(&collection_name, connected_db.file_path())
                {
                    Ok(()) => println!("Collection deleted"),
                    Err(e) => return eprintln!("[Error] Failed to delete collection: {e}"),
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

        let collections = match self.engine
            .storage_api()
            .find_all_collections(connected_db.file_path())
        {
            Ok(collections) => collections,
            Err(e) => return eprintln!("[Error] Failed to list collections: {e}"),
        };

        println!("\nNumber of collections: {}", collections.len());

        for collection in collections {
            println!("{}", collection.name());
        }
    }
}
