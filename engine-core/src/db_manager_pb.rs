// Database manager Protocol Buffers

#![allow(unused)]

use std::{
    collections::HashMap,
    path::{
        Path,
        PathBuf,
    },
};
use crate::{
    logging::*,
    InputDataField,
    db::{
        self,
        error::DatabaseOperationError,
        pb,
        database_pb::*,
        collection_pb::*,
        document_pb::*,
        create_db_dir_if_not_exists,
        DB_FILE_EXTENSION,
    },
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
    /// Gets databases directory path.
    fn db_dir_path(&self) -> &Path {
        &self.db_dir_path
    }

    /// Gets logs directory path.
    pub fn logs_dir_path(&self) -> &Path {
        &self.logs_dir_path
    }

    /// Gets database file path.
    fn db_file_path(&self, db_name: &str) -> PathBuf {
        PathBuf::from(&self.db_dir_path()
            .join(format!("{}.{}", db_name, DB_FILE_EXTENSION)))
    }

    /// Attempts to log events to log file.
    pub fn log_event(&self, content: &str) {
        if let Err(e) = Logger::log_event(
            content,
            &self.logs_dir_path(),
            &self.logs_dir_path().join(DB_EVENTS_LOG)
        ) {
            eprintln!("[Error] {}", e);
        }
    }

    /// Attempts to log errors to log file.
    pub fn log_error(&self, content: &str) {
        if let Err(e) = Logger::log_error(
            ErrorLogType::Error,
            content,
            &self.logs_dir_path(),
            &self.logs_dir_path().join(ERRORS_LOG)
        ) {
            eprintln!("[Error] {}", e);
        }
    }
}


