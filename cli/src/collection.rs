use crate::{
    Cli,
    ask_user_input,
    ask_action_confirm,
    CONFIRM_OPTION_YES,
    NO_CONNECTED_DB,
};

impl<'a> Cli<'a> {
    /// Checks if collection exists.
    pub fn collection_exists(
        &self,
        collection_name: &str,
        connected_db_name: &str,
    ) -> bool
    {
        match &self.engine.api().find_collection(collection_name, connected_db_name) {
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
        let connected_db_name = match &self.connected_db {
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

        match &self.engine.api().create_collection(&collection_name, connected_db_name) {
            Ok(()) => println!("Collection created"),
            Err(e) => return eprintln!("[Error] {e}"),
        }
    }

    /// Show menu to delete a collection from the connected database.
    pub fn delete_collection(&self) {
        let connected_db_name = match &self.connected_db {
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
                match &self.engine.api().delete_collection(&collection_name, connected_db_name) {
                    Ok(()) => println!("Collection deleted"),
                    Err(e) => return eprintln!("[Error] {e}"),
                }
            },
            _ => return println!("Canceled action"),
        }

    }

    /// List all collections of the connected database.
    pub fn list_all_collections(&self) {
        let connected_db_name = match &self.connected_db {
            Some(db_name) => db_name,
            None => return println!("{}", NO_CONNECTED_DB),
        };

        if !&self.database_exists(connected_db_name) {
            return;
        }

        let collections = match self
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
}
