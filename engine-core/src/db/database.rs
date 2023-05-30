use serde::{Serialize, Deserialize};
use std::{
    io,
    fs,
    path::Path,
};
use crate::db::{
    DocumentCollection,
    database_file_path,
    write_database_json,
    create_databases_dir_if_not_exists,
};
use crate::constants::{
    DB_NOT_FOUND,
    DB_EXISTS,
    DB_FILE_EXTENSION,
    DB_DIR_PATH,
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
pub fn create_database_file(
    database_name: &str,
    file_path: &Path
) -> io::Result<(bool, String)>
{
    let mut message = "";

    if file_path.is_file() {
        let file = fs::File::create(file_path)?;
        let database = Database::from(database_name);
        
        match write_database_json(&database, file_path) {
            Ok(()) => return Ok((true, message.to_string())),
            Err(e) => return Err(e),
        }
    } else {
        message = DB_EXISTS;
    }

    Ok((false, message.to_string()))
}

/// Deletes a database file in databases directory
pub fn delete_database_file(
    database_name: &str,
    file_path: &Path
) -> io::Result<(bool, String)>
{
    let mut message = "";

    if file_path.is_file() {
        fs::remove_file(file_path)?;

        return Ok((true, message.to_string()));
    } else {
        message = DB_NOT_FOUND;
    }
    
    Ok((false, message.to_string()))
}

/// Finds all database files in databases directory
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
pub fn find_database(
    database_name: &str,
    dir_path: &Path
) -> io::Result<bool>
{
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if entry.file_name() == format!("{database_name}.{DB_FILE_EXTENSION}").as_str() {
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
pub fn change_database_description(
    database_name: &str,
    description: &str,
    file_path: &Path,
) -> io::Result<(bool, String)>
{
    let mut message = "";

    if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
        let mut database: Database = serde_json::from_str(contents.as_str())?;
        database.description = String::from(description);
        
        match write_database_json(&database, file_path) {
            Ok(()) => return Ok((true, message.to_string())),
            Err(e) => return Err(e),
        }
    } else {
        message = DB_NOT_FOUND;
    }

    Ok((false, message.to_string()))
}



/* THESE WILL BE CHANGED */

#[cfg(test)]
mod tests {
    use crate::db::{
        temp_database_file_path,
        create_temp_databases_dir_if_not_exists,
    };
    use crate::constants::TEMP_DB_DIR_PATH;
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

    /*#[test]
    fn test_create_database_file() {
        let database_name = "test_create_database_file";
        let file_path = temp_database_file_path(database_name);

        if Path::new(&file_path).is_file() {
            fs::remove_file(&file_path).unwrap();
        }
        create_temp_databases_dir_if_not_exists().unwrap();
        
        let (result, message) = match create_database_file(
            database_name,
            &file_path,
        ) {
            Ok((result, message)) => (result, message),
            Err(e) => panic!("create_database_file failed: {e}"),
        };

        assert_eq!((result, message), (true, "".to_string()));
        assert_eq!(Path::new(&file_path).is_file(), true);

        fs::remove_file(&file_path).unwrap();
    }*/

    /*#[test]
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
            Err(e) => panic!("delete_database_file failed: {e}"),
        };

        assert_eq!((result, message), (true, "".to_string()));
        assert_eq!(Path::new(&file_path).try_exists().unwrap(), false);
    }*/

    /*#[test]
    fn test_find_all_databases() {
        create_temp_databases_dir_if_not_exists().unwrap();

        let databases = match find_all_databases(TEMP_DB_DIR_PATH) {
            Ok(databases) => databases,
            Err(e) => return panic!("find_all_databases failed: {e}"),
        };

        assert_eq!(databases.len().ge(&0), true);
    }*/

    /*#[test]
    fn test_find_database() {
        let database_name = "test_find_database";
        create_temp_databases_dir_if_not_exists().unwrap();

        let result = match find_database(database_name, TEMP_DB_DIR_PATH) {
            Ok(result) => result,
            Err(e) => panic!("find_database failed: {e}"),
        };

        assert_eq!(result, false);
    }*/
}

