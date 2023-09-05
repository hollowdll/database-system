use std::path::{
    PathBuf,
    Path,
};

pub struct DatabaseClient {
    connection_string: PathBuf,
}

impl DatabaseClient {
    pub fn build(connection_string: &Path) -> DatabaseClient {
        DatabaseClient {
            connection_string: PathBuf::from(connection_string),
        }
    }
}

pub struct Database {
    pub name: String,
}
