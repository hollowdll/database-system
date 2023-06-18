use serde::{Serialize, Deserialize};
use std::{
    io,
    fs,
    path::Path,
    error::Error,
};
use crate::{
    db::{
        error::DatabaseError,
        DocumentCollection,
        write_database_json,
    },
    constants::{
        DB_NOT_FOUND,
        DB_EXISTS,
        DB_FILE_EXTENSION,
        DB_DIR_PATH,
    },
};

/// Database structure for database files
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    name: String,
    pub description: String,
    pub collections: Vec<DocumentCollection>,
    pub id_count: u64,
}

impl Database {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut str {
        &mut self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn collections(&self) -> &Vec<DocumentCollection> {
        &self.collections
    }

    pub fn collections_mut(&mut self) -> &mut Vec<DocumentCollection> {
        &mut self.collections
    }

    pub fn id_count(&self) -> &u64 {
        &self.id_count
    }
}

impl From<&str> for Database {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::new(),
            collections: Vec::new(),
            id_count: 0,
        }
    }
}

impl From<(&str, &str)> for Database {
    fn from((name, description): (&str, &str)) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            collections: Vec::new(),
            id_count: 0,
        }
    }
}

/// Formatted database that can be listed in clients.
/// 
/// Size = database file size in bytes.
#[derive(Debug, PartialEq)]
pub struct FormattedDatabase {
    name: String,
    description: String,
    size: u64,
}

impl FormattedDatabase {
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

impl FormattedDatabase {
    pub fn from(name: String, description: String, size: u64) -> Self {
        Self {
            name,
            description,
            size,
        }
    }
}



/// Creates a database file in databases directory
pub fn create_database_file(
    database_name: &str,
    file_path: &Path
) -> Result<(), Box<dyn Error>>
{
    if !file_path.is_file() {
        let file = fs::File::create(file_path)?;
        let database = Database::from(database_name);
        
        match write_database_json(&database, file_path) {
            Ok(()) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    } else {
        return Err(Box::new(DatabaseError::Exists))
    }
}

/// Deletes a database file in databases directory
pub fn delete_database_file(
    database_name: &str,
    file_path: &Path
) -> Result<(), Box<dyn Error>>
{
    if file_path.is_file() {
        fs::remove_file(file_path)?;

        return Ok(())
    } else {
        return Err(Box::new(DatabaseError::NotFound));
    }
}

/// Finds all databases in databases directory
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
                    let contents = fs::read_to_string(path)?;
                    let database: Database = match serde_json::from_str(contents.as_str()) {
                        Ok(database) => database,
                        Err(e) => {
                            eprintln!("Error parsing database: {e} ({:?})", entry.file_name());
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

/// Finds a database in databases directory.
pub fn find_database(
    database_name: &str,
    dir_path: &Path
) -> io::Result<Option<FormattedDatabase>>
{
    let mut error_message = "";
    
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if entry.file_name() == format!("{database_name}.{DB_FILE_EXTENSION}").as_str() {
                // Check if json file contains the name
                let contents = fs::read_to_string(path)?;
                let database: Database = serde_json::from_str(contents.as_str())?;

                if database.name() == database_name {
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

/// Changes description of a database.
/// 
/// Modifies `description` field in a database file.
pub fn change_database_description(
    description: &str,
    file_path: &Path,
) -> Result<(), Box<dyn Error>>
{
    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        database.description = String::from(description);
        
        match write_database_json(&database, file_path) {
            Ok(()) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    } else {
        return Err(Box::new(DatabaseError::NotFound));
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use fs::File;
    use std::io::{self, Read, Write};

    #[test]
    fn test_database_struct() {
        let database_name = "test";
        let database = Database {
            name: String::from(database_name),
            description: String::new(),
            collections: Vec::new(),
            id_count: 0,
        };

        assert_eq!(database, Database::from(database_name));
    }

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

