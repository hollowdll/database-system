use std::{
    io,
    fs,
    path::Path,
    error::Error,
};
use crate::storage::{
    error::DatabaseError,
    pb,
    serialize_database,
    deserialize_database,
    write_database_to_file,
    DB_FILE_EXTENSION,
};

// Implements methods for protobuf type
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
}

impl From<&str> for pb::Database {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::new(),
            collections: Vec::new(),
        }
    }
}

impl From<(&str, &str)> for pb::Database {
    fn from((name, description): (&str, &str)) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            collections: Vec::new(),
        }
    }
}

/// Database data transfer object (DTO).
/// 
/// Exposes database data that clients can use.
/// 
/// `size` = database file size in bytes.
#[derive(Debug, PartialEq)]
pub struct DatabaseDto {
    name: String,
    description: String,
    size: u64,
}

impl DatabaseDto {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }
}

impl From<(String, String, u64)> for DatabaseDto {
    fn from((name, description, size): (String, String, u64)) -> Self {
        Self {
            name,
            description,
            size,
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
/// 
/// Returns the found databases.
pub fn find_all_databases(
    dir_path: &Path
) -> io::Result<Vec<DatabaseDto>>
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

                    let database_dto = DatabaseDto::from((
                        database.name,
                        database.description,
                        entry.metadata()?.len()
                    ));
                    
                    databases.push(database_dto);
                }
            }
        }
    }

    Ok(databases)
}

/// Finds a database from a directory.
/// 
/// Returns the found database.
pub fn find_database(
    db_name: &str,
    dir_path: &Path
) -> io::Result<Option<DatabaseDto>>
{
    let mut error_message = "";
    
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if entry.file_name() == format!("{db_name}.{DB_FILE_EXTENSION}").as_str() {
                let database = deserialize_database(&fs::read(path)?)?;

                if database.name() == db_name {
                    let database_dto = DatabaseDto::from((
                        database.name,
                        database.description,
                        entry.metadata()?.len()
                    ));

                    return Ok(Some(database_dto));
                }
            }
        }
    }

    Ok(None)
}



/*
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use fs::File;
    use std::io::{self, Read, Write};

    #[test]
    fn test_create_database_file() {
        let database_name = "test";
        let expected_json = serde_json::to_string_pretty(
            &Database::from(database_name)
        ).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        assert!(create_database_file(database_name, file_path.as_path()).is_ok());

        let buf = fs::read_to_string(&file_path).unwrap();
        assert_eq!(buf, expected_json);

        dir.close().expect("Failed to clean up tempdir before dropping.");
    }

    #[test]
    fn test_delete_database_file() {
        let database_name = "";
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let file = File::create(&file_path).unwrap();

        assert_eq!(file_path.try_exists().unwrap(), true);
        assert!(delete_database_file(database_name, file_path.as_path()).is_ok());
        assert_eq!(file_path.try_exists().unwrap(), false);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }

    #[test]
    fn test_find_all_databases() {
        let dir = tempdir().unwrap();
        let databases = find_all_databases(dir.path()).unwrap();
        assert_eq!(databases.len() == 0, true);

        dir.close().expect("Failed to clean up tempdir before dropping.");
    }

    #[test]
    fn test_find_database() {
        let database_name = "test";
        let dir = tempdir().unwrap();

        let result = find_database(database_name, dir.path()).unwrap();
        assert_eq!(result, None);

        dir.close().expect("Failed to clean up tempdir before dropping.");
    }

    #[test]
    fn test_change_database_description() {
        let description = "Test database 123";
        let mut database = Database::from("test");
        let json = serde_json::to_string_pretty(&database).unwrap();
        database.description = String::from(description);
        let expected_json = serde_json::to_string_pretty(&database).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write(json.as_bytes()).is_ok());
        assert!(change_database_description(description, file_path.as_path()).is_ok());

        let buf = fs::read_to_string(&file_path).unwrap();
        assert_eq!(buf, expected_json);

        drop(file);
        dir.close().expect("Failed to clean up tempdir before dropping.");
    }
}
*/
