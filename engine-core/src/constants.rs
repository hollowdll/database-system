// This file contains constants used in the engine library

pub const DB_EVENT_LOG_ERROR: &str = "Error occurred while trying to log database event";
pub const DB_NOT_FOUND: &str = "Database was not found";
pub const DB_EXISTS: &str = "Database already exists";
pub const COLLECTION_NOT_FOUND: &str = "Collection was not found";
pub const DOCUMENT_NOT_FOUND: &str = "Document was not found";

// Path to databases directory in filesystem
pub const DATABASES_DIR_PATH: &str = "./databases";

// Path to temporary databases directory.
// This directory is mainly used in tests.
pub const TEMP_DATABASES_DIR_PATH: &str = "./temp-databases";

// Database files have JSON file extension
pub const DATABASE_FILE_EXTENSION: &str = "json";
