// This file contains database manager related code

use std::{
    io,
    collections::HashMap,
    path::{
        Path,
        PathBuf,
    },
    error::Error,
};
use crate::{
    logging::*,
    constants::{
        DB_EVENT_LOG_ERROR,
        DB_FILE_EXTENSION,
    },
    InputDataField,
};
use crate::db::{
    self,
    DataType,
    FormattedDatabase,
    FormattedDocumentCollection,
    FormattedDocument,
};

/// Database manager that manages all databases
/// and database related operations
#[derive(PartialEq, Debug)]
pub struct DatabaseManager {
    /// Directory path where databases will be created.
    db_dir_path: PathBuf,

    /// Directory path where logs will be created.
    logs_dir_path: PathBuf,
}

impl DatabaseManager {
    /// Build a new database manager.
    pub fn build(db_dir_path: PathBuf, logs_dir_path: PathBuf) -> Self {
        Self {
            db_dir_path,
            logs_dir_path,
        }
    }
}

impl DatabaseManager {
    fn db_dir_path(&self) -> &Path {
        &self.db_dir_path
    }

    fn logs_dir_path(&self) -> &Path {
        &self.logs_dir_path
    }

    fn db_file_path(&self, db_name: &str) -> PathBuf {
        let mut path = PathBuf::from(&self.db_dir_path().join(db_name));
        path.set_extension(DB_FILE_EXTENSION);

        return path
    }
}

impl DatabaseManager {
    /// Creates a new database 
    pub fn create_database(
        &self,
        database_name: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::create_databases_dir_if_not_exists(&self.db_dir_path())?;

        db::create_database_file(
            database_name,
            &self.db_file_path(database_name)
        )?;

        if let Err(err) = Logger::log_event(
            &format!("Created database '{}'", database_name),
            &self.logs_dir_path(),
            &self.logs_dir_path().join(DB_EVENTS_LOG),
        ) {
            eprintln!("{}", err);
        }

        Ok("Created database".to_string())
    }

    /// Deletes a database
    pub fn delete_database(
        &self,
        database_name: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::delete_database_file(
            database_name,
            &self.db_file_path(database_name)
        )?;

        if let Err(err) = Logger::log_event(
            &format!("Deleted database '{}'", database_name),
            &self.logs_dir_path(),
            &self.logs_dir_path().join(DB_EVENTS_LOG),
        ) {
            eprintln!("{}", err);
        }
        
        Ok("Deleted database".to_string())
    }

    /// Changes description of a database
    pub fn change_database_description(
        &self,
        database_name: &str,
        description: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::change_database_description(
            description,
            &self.db_file_path(database_name)
        )?;

        if let Err(err) = Logger::log_event(
            &format!("Changed description of database '{}'", database_name),
            &self.logs_dir_path(),
            &self.logs_dir_path().join(DB_EVENTS_LOG),
        ) {
            eprintln!("{}", err);
        }

        Ok("Changed database description".to_string())
    }

    /// Creates a new collection to a database
    pub fn create_collection(
        &self,
        collection_name: &str,
        database_name: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::create_collection_to_database_file(
            collection_name,
            &self.db_file_path(database_name)
        )?;

        if let Err(err) = Logger::log_event(
            &format!(
                "Created collection '{}' to database '{}'",
                collection_name,
                database_name
            ),
            &self.logs_dir_path(),
            &self.logs_dir_path().join(DB_EVENTS_LOG),
        ) {
            eprintln!("{}", err);
        }

        Ok("Created collection".to_string())
    }

    /// Deletes a collection from a database
    pub fn delete_collection(
        &self,
        collection_name: &str,
        database_name: &str,
    ) -> Result<String, Box<dyn Error>>
    {
        db::delete_collection_from_database_file(
            collection_name,
            &self.db_file_path(database_name)
        )?;

        if let Err(err) = Logger::log_event(
            &format!(
                "Deleted collection '{}' from database '{}'",
                collection_name,
                database_name
            ),
            &self.logs_dir_path(),
            &self.logs_dir_path().join(DB_EVENTS_LOG),
        ) {
            eprintln!("{}", err);
        }

        Ok("Deleted collection".to_string())
    }

    /// Finds all databases
    pub fn find_all_databases(&self) -> io::Result<Vec<FormattedDatabase>> {
        db::create_databases_dir_if_not_exists(&self.db_dir_path())?;

        return db::find_all_databases(&self.db_dir_path())
    }

    /// Finds a database
    pub fn find_database(&self, database_name: &str) -> io::Result<Option<FormattedDatabase>> {
        db::create_databases_dir_if_not_exists(&self.db_dir_path())?;

        return db::find_database(database_name, &self.db_dir_path())
    }

    /// Finds all collections of a database
    pub fn find_all_collections_of_database(
        &self,
        database_name: &str,
    ) -> io::Result<Vec<FormattedDocumentCollection>>
    {
        return db::find_all_collections_of_database(
            &self.db_file_path(database_name)
        )
    }

    /// Finds a collection in a database
    pub fn find_collection(
        &self,
        collection_name: &str,
        database_name: &str,
    ) -> Result<Option<FormattedDocumentCollection>, Box<dyn Error>>
    {
        return db::find_collection(
            collection_name,
            &self.db_file_path(database_name)
        )
    }

    /// Creates a new document to a collection
    pub fn create_document(
        &self,
        database_name: &str,
        collection_name: &str,
        data: Vec<InputDataField>,
    ) -> Result<String, Box<dyn Error>>
    {
        let mut document_data: HashMap<String, DataType> = HashMap::new();

        // convert input data to correct document data types
        for data_field in data {
            let converted_value = match data_field.convert_to_document_data_type(
                data_field.value(),
                data_field.data_type()
            ) {
                Ok(converted_value) => converted_value,
                Err(err) => return Err(format!(
                    "Data type '{}' is not valid: {}",
                    data_field.data_type(),
                    err
                ).into()),
            };

            document_data.insert(data_field.field().to_string(), converted_value);
        }

        db::create_document_to_collection(
            &self.db_file_path(database_name),
            collection_name,
            document_data
        )?;

        if let Err(err) = Logger::log_event(
            &format!(
                "Created document to collection '{}' in database '{}'",
                collection_name,
                database_name
            ),
            &self.logs_dir_path(),
            &self.logs_dir_path().join(DB_EVENTS_LOG),
        ) {
            eprintln!("{}", err);
        }

        Ok("Created document".to_string())
    }

    /// Deletes a document from database
    pub fn delete_document(
        &self,
        database_name: &str,
        document_id: &u64,
    ) -> Result<String, Box<dyn Error>>
    {
        db::delete_document(&self.db_file_path(database_name), document_id)?;

        if let Err(e) = Logger::log_event(
            &format!(
                "Deleted document with ID '{}' from database '{}'",
                document_id, database_name
            ),
            &self.logs_dir_path(),
            &self.logs_dir_path().join(DB_EVENTS_LOG), 
        ) {
            eprintln!("{}: {e}", DB_EVENT_LOG_ERROR);
        }

        Ok("Deleted document".to_string())
    }

    /// Finds all documents of collection
    pub fn find_all_documents_of_collection(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> io::Result<Vec<FormattedDocument>>
    {
        return db::find_all_documents_of_collection(
            &self.db_file_path(database_name),
            collection_name
        )
    }

    /// Finds a document from a database by its id.
    pub fn find_document_by_id(
        &self,
        document_id: &u64,
        database_name: &str,
    ) -> Result<Option<FormattedDocument>, Box<dyn Error>>
    {
        return db::find_document_by_id(
            document_id,
            &self.db_file_path(database_name)
        )
    }
}
