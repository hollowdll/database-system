use std::{
    io,
    fs,
    path::{
        Path,
        PathBuf,
    },
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

    /// Validates database by checking its field values.
    /// 
    /// Returns any errors that may occur during the process.
    pub fn validate_errors(&self) -> Result<(), DatabaseError> {
        if self.name.is_empty() {
            return Err(DatabaseError::EmptyName);
        }

        Ok(())
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
    file_path: PathBuf,
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

    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    /// Creates a new instance of `DatabaseDto`.
    pub fn new(
        name: String,
        description: String,
        size: u64,
        file_path: PathBuf
    ) -> Self
    {
        Self {
            name,
            description,
            size,
            file_path,
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

    let database = pb::Database::from(db_name);
    if let Err(e) = database.validate_errors() {
        return Err(Box::new(e));
    }

    let file = fs::File::create(file_path)?;
    let buf = serialize_database(&database)?;

    match write_database_to_file(&buf, file_path) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(e.into()),
    }
}

/// Deletes a database file.
/// 
/// Checks if the file contains a valid database.
pub fn delete_database_file(
    file_path: &Path
) -> Result<(), Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }

    deserialize_database(&fs::read(file_path)?)?;
    fs::remove_file(file_path)?;
    
    Ok(())
}

/// Changes description of a database.
/// 
/// Writes the modified database to the database file.
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
    if let Err(e) = database.validate_errors() {
        return Err(Box::new(e));
    }

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
                    let buf = fs::read(&path)?;
                    let database = match deserialize_database(&buf) {
                        Ok(database) => database,
                        Err(e) => {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                format!("Error parsing database: {} ({:?})", e, entry.file_name())
                            ));
                        },
                    };
                    if let Err(_) = database.validate_errors() {
                        continue;
                    }

                    let database_dto = DatabaseDto::new(
                        database.name,
                        database.description,
                        entry.metadata()?.len(),
                        path
                    );
                    
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
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if entry.file_name() == format!("{db_name}.{DB_FILE_EXTENSION}").as_str() {
                let database = deserialize_database(&fs::read(&path)?)?;
                if let Err(_) = database.validate_errors() {
                    return Ok(None);
                }

                if database.name() == db_name {
                    let database_dto = DatabaseDto::new(
                        database.name,
                        database.description,
                        entry.metadata()?.len(),
                        path
                    );

                    return Ok(Some(database_dto));
                }
            }
        }
    }

    Ok(None)
}

/// Finds a database using the given file path.
/// 
/// Returns the found database.
pub fn find_database_by_file_path(
    file_path: &Path,
) -> io::Result<Option<DatabaseDto>>
{
    if !file_path.is_file() {
        return Ok(None);
    }

    let database = deserialize_database(&fs::read(file_path)?)?;
    if let Err(_) = database.validate_errors() {
        return Ok(None)
    }

    let database_dto = DatabaseDto::new(
        database.name,
        database.description,
        file_path.metadata()?.len(),
        PathBuf::from(file_path),
    );

    Ok(Some(database_dto))
}



#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use fs::File;
    use std::io::{self, Read, Write};
    use crate::storage::pb::Database;

    #[test]
    fn test_create_database_file() {
        let db = Database::from("test");
        let expected_db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));

        assert!(create_database_file(db.name(), &file_path).is_ok());
        assert_eq!(fs::read(&file_path).unwrap(), expected_db_buf);

        dir.close().unwrap();
    }

    #[test]
    fn test_delete_database_file() {
        let db = Database::from("test");
        let db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        assert_eq!(file_path.try_exists().unwrap(), true);
        assert!(delete_database_file(&file_path).is_ok());
        assert_eq!(file_path.try_exists().unwrap(), false);
        
        drop(file);
        dir.close().unwrap();
    }
    
    #[test]
    fn test_change_database_description() {
        let description = "Test desc";
        let mut db = Database::from("test");
        let db_buf = serialize_database(&db).unwrap();
        db.description = String::from(description);
        let expected_db_buf = serialize_database(&db).unwrap();
    
        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();
    
        assert!(file.write_all(&db_buf).is_ok());
        assert!(change_database_description(description, &file_path).is_ok());
        assert_eq!(fs::read(&file_path).unwrap(), expected_db_buf);
    
        drop(file);
        dir.close().unwrap();
    }
    
    #[test]
    fn test_find_all_databases() {
        let db = Database::from("test");
        let db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        let databases = find_all_databases(dir.path()).unwrap();
        assert_eq!(databases.get(0).unwrap().name(), db.name());
        assert!(databases.len() == 1);

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn test_find_database() {
        let db = Database::from("test");
        let db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        let found_db = find_database(db.name(), dir.path()).unwrap();
        assert!(found_db.is_some());
        assert_eq!(found_db.unwrap().name(), db.name());

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn test_find_database_by_file_path() {
        let db = Database::from("test");
        let db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        let found_db = find_database_by_file_path(&file_path).unwrap();
        assert!(found_db.is_some());
        assert_eq!(found_db.unwrap().name(), db.name());

        drop(file);
        dir.close().unwrap();
    }
}
