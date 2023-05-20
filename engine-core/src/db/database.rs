use serde::{Serialize, Deserialize};
use std::{
    io,
    fs,
    path::Path,
};
use crate::db::{
    DocumentCollection,
    database_file_path,
    temp_database_file_path,
    write_database_json,
    create_databases_dir_if_not_exists,
    create_temp_databases_dir_if_not_exists,
};
use crate::constants::{
    DB_NOT_FOUND,
    DB_EXISTS,
    DATABASES_DIR_PATH,
    DATABASE_FILE_EXTENSION,
    TEMP_DATABASES_DIR_PATH,
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
pub fn create_database_file(database_name: &str, file_path: &str) -> io::Result<(bool, String)> {
    let mut message = "";

    if !Path::new(&file_path).is_file() {
        let file = fs::File::create(&file_path)?;
        let database = Database::from(database_name);
        
        match write_database_json(&database, &file_path) {
            Ok(()) => return Ok((true, message.to_string())),
            Err(e) => return Err(e),
        }
    } else {
        message = DB_EXISTS;
    }

    Ok((false, message.to_string()))
}

/// Deletes a database file in databases directory
pub fn delete_database_file(database_name: &str, file_path: &str) -> io::Result<(bool, String)> {
    let mut message = "";

    if Path::new(&file_path).is_file() {
        fs::remove_file(&file_path)?;

        return Ok((true, message.to_string()));
    } else {
        message = DB_NOT_FOUND;
    }
    
    Ok((false, message.to_string()))
}

/// Finds all database files in databases directory
pub fn find_all_databases() -> io::Result<Vec<FormattedDatabase>> {
    create_databases_dir_if_not_exists()?;

    let mut databases = Vec::new();

    for entry in fs::read_dir(DATABASES_DIR_PATH)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(file_extension) = path.extension() {
                if file_extension == DATABASE_FILE_EXTENSION {
                    let contents = fs::read_to_string(path)?;
                    let database: Database = match serde_json::from_str(contents.as_str()) {
                        Ok(database) => database,
                        Err(e) => {
                            eprintln!("Error parsing database: {e} ({:?})", entry.file_name());
                            continue
                        },
                    };

                    let formatted_database = FormattedDatabase::from(
                        String::from(database.name()),
                        String::from(database.description()),
                        entry.metadata()?.len()
                    );
                    
                    databases.push(formatted_database);
                }
            }
        }
    }

    Ok(databases)
}

/// Finds a database file in databases directory.
pub fn find_database(database_name: &str) -> io::Result<bool> {
    create_databases_dir_if_not_exists()?;

    for entry in fs::read_dir(DATABASES_DIR_PATH)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if entry.file_name() == format!("{database_name}.{DATABASE_FILE_EXTENSION}").as_str() {
                // Check if json file contains the name
                let contents = fs::read_to_string(path)?;
                let database: Database = serde_json::from_str(contents.as_str())?;

                if database.name() == database_name {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

/// Changes description of a database.
/// 
/// Modifies `description` field in a database file.
pub fn change_database_description(database_name: &str, description: &str) -> io::Result<(bool, String)> {
    let file_path = database_file_path(database_name);
    let mut message = "";

    if Path::new(&file_path).is_file() {
        let contents = fs::read_to_string(&file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        database.description = String::from(description);
        
        match write_database_json(&database, &file_path) {
            Ok(()) => return Ok((true, message.to_string())),
            Err(e) => return Err(e),
        }
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((false, message.to_string()))
}



#[cfg(test)]
mod tests {
    use crate::db::temp_database_file_path;

    use super::*;

    #[test]
    fn test_database_struct() {
        let database_name = "test_database_struct";
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
        let database_name = "test_create_database_file";
        let file_path = temp_database_file_path(database_name);

        create_temp_databases_dir_if_not_exists().unwrap();
        
        let (result, message) = match create_database_file(
            database_name,
            &file_path,
        ) {
            Ok((result, message)) => (result, message),
            Err(e) => panic!("function create_database_file failed: {e}"),
        };
        assert_eq!((result, message), (true, "".to_string()));

        fs::remove_file(&file_path).unwrap();
        assert_eq!(Path::new(&file_path).try_exists().unwrap(), false);
    }

    #[test]
    fn test_delete_database_file() {
        let database_name = "test_delete_database_file";
        let file_path = temp_database_file_path(database_name);

        create_temp_databases_dir_if_not_exists().unwrap();
        let file = fs::File::create(&file_path).unwrap();

        let (result, message) = match delete_database_file(
            database_name,
            &file_path,
        ) {
            Ok((result, message)) => (result, message),
            Err(e) => panic!("function delete_database_file failed: {e}"),
        };
        assert_eq!((result, message), (true, "".to_string()));

        if Path::new(&file_path).is_file() {
            fs::remove_file(&file_path).unwrap();
        }
        assert_eq!(Path::new(&file_path).try_exists().unwrap(), false);
    }
}
