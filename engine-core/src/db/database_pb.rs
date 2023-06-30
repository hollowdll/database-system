// Database Protocol Buffers module

use std::{
    io,
    fs,
    path::Path,
    error::Error,
};
use crate::db::{
    error::DatabaseError,
    pb,
    serialize_database,
    deserialize_database,
    write_database_to_file,
    FormattedDatabase,
    DB_FILE_EXTENSION,
};

impl pb::Database {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn collections(&self) -> &Vec<pb::Collection> {
        &self.collections
    }

    pub fn collections_mut(&mut self) -> &mut Vec<pb::Collection> {
        &mut self.collections
    }

    pub fn id_count(&self) -> &u64 {
        &self.id_count
    }
}

impl From<&str> for pb::Database {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::new(),
            collections: Vec::new(),
            id_count: 0,
        }
    }
}

impl From<(&str, &str)> for pb::Database {
    fn from((name, description): (&str, &str)) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            collections: Vec::new(),
            id_count: 0,
        }
    }
}

/// Creates a database file and writes initial data to it.
pub fn create_database_file(
    db_name: &str,
    file_path: &Path
) -> Result<(), Box<dyn Error>>
{
    if file_path.is_file() {
        return Err(Box::new(DatabaseError::Exists))
    }

    let file = fs::File::create(file_path)?;
    let database = pb::Database::from(db_name);
    let buf = serialize_database(&database)?;

    match write_database_to_file(&buf, file_path) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(e.into()),
    }
}

/// Deletes a database file.
pub fn delete_database_file(
    file_path: &Path
) -> Result<(), Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    fs::remove_file(file_path)?;
    
    Ok(())
}

/// Changes description of a database and saves the changes to the database file.
pub fn change_database_description(
    description: &str,
    file_path: &Path
) -> Result<(), Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }

    let mut database = deserialize_database(&fs::read(file_path)?)?;
    database.description = description.to_string();
    let buf = serialize_database(&database)?;

    match write_database_to_file(&buf, file_path) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(e.into()),
    }
}

/// Finds all databases from a directory.
pub fn find_all_databases(
    dir_path: &Path
) -> io::Result<Vec<FormattedDatabase>>
{
    let mut databases = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(file_extension) = path.extension() {
                if file_extension == DB_FILE_EXTENSION {
                    let buf = fs::read(path)?;
                    let database = match deserialize_database(&buf) {
                        Ok(database) => database,
                        Err(e) => {
                            eprintln!("Error parsing database: {} ({:?})", e, entry.file_name());
                            continue
                        },
                    };

                    let formatted_database = FormattedDatabase::from(
                        database.name,
                        database.description,
                        entry.metadata()?.len()
                    );
                    
                    databases.push(formatted_database);
                }
            }
        }
    }

    Ok(databases)
}

/// Finds a database from a directory.
pub fn find_database(
    db_name: &str,
    dir_path: &Path
) -> io::Result<Option<FormattedDatabase>>
{
    let mut error_message = "";
    
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if entry.file_name() == format!("{db_name}.{DB_FILE_EXTENSION}").as_str() {
                let database = deserialize_database(&fs::read(path)?)?;

                if database.name() == db_name {
                    let formatted_database = FormattedDatabase::from(
                        database.name,
                        database.description,
                        entry.metadata()?.len()
                    );

                    return Ok(Some(formatted_database));
                }
            }
        }
    }

    Ok(None)
}
