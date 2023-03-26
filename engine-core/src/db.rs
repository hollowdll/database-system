// Restructured version of database
// Not final

use std::{
    fs,
    io,
    path::Path,
};

struct Database {
    name: String,
}

struct DatabaseDocumentCollection {
    name: String,
}

struct DatabaseDocument {
    id: u64,
}

fn create_databases_dir() -> io::Result<()> {
    if !Path::new("./databases").is_dir() {
        fs::create_dir("./databases")?;
    }

    Ok(())
}

fn create_database_file(database_name: &str) -> io::Result<bool> {
    if !Path::new(format!("./databases/{database_name}.json").as_str()).is_file() {
        let file = fs::File::create(format!("./databases/{database_name}.json"))?;

        return Ok(true);
    } else {
        return Ok(false);
    }
}

fn database_file_exists(database_name: &str) -> bool {
    if Path::new(format!("./databases/{database_name}.json").as_str()).is_file() {
        return true;
    } else {
        return false;
    }
}