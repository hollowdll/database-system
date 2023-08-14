use std::{
    io,
    fs,
    path::Path,
    error::Error,
};
use crate::storage::{
    error::{
        DatabaseError,
        CollectionError,
    },
    pb,
    serialize_database,
    deserialize_database,
    write_database_to_file,
    DB_FILE_EXTENSION,
};

// Implements methods for protobuf type
impl pb::Collection {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn documents(&self) -> &Vec<pb::Document> {
        &self.documents
    }

    pub fn documents_mut(&mut self) -> &mut Vec<pb::Document> {
        &mut self.documents
    }

    pub fn id_count(&self) -> &u64 {
        &self.id_count
    }
}

impl From<&str> for pb::Collection {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            documents: Vec::new(),
            id_count: 0,
        }
    }
}

/// Collection data transfer object (DTO).
/// 
/// Exposes collection data that clients can use.
#[derive(Debug, PartialEq)]
pub struct CollectionDto {
    name: String,
}

impl CollectionDto {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<&str> for CollectionDto {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}



/// Checks if a collection exists in a database.
pub fn collection_exists(db: &pb::Database, collection_name: &str) -> bool {
    for collection in db.collections() {
        if collection.name() == collection_name {
            return true;
        }
    }

    false
}

/// Creates a new collection to a database.
/// 
/// Writes the modified database to the database file.
pub fn create_collection_to_database(
    collection_name: &str,
    file_path: &Path,
) -> Result<(), Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut database = deserialize_database(&fs::read(file_path)?)?;
    
    // If collection already exists
    if collection_exists(&database, collection_name) {
        return Err(Box::new(CollectionError::Exists));
    }

    let collection = pb::Collection::from(collection_name);
    database.collections_mut().push(collection);
    let buf = serialize_database(&database)?;

    match write_database_to_file(&buf, file_path) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(e.into()),
    }
}

/// Deletes a collection from a database.
/// 
/// Writes the modified database to the database file.
pub fn delete_collection_from_database(
    collection_name: &str,
    file_path: &Path
) -> Result<(), Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }

    let mut database = deserialize_database(&fs::read(file_path)?)?;
    let mut collection_exists = false;

    for collection in database.collections() {
        if collection.name() == collection_name {
            // cancel if collection has documents
            if collection.documents().len() > 0 {
                return Err(Box::new(CollectionError::HasDocuments));
            }
            collection_exists = true;
            break;
        }
    }

    if !collection_exists {
        return Err(Box::new(CollectionError::NotFound));
    }

    database
        .collections_mut()
        .retain(|collection| collection.name() != collection_name);
    let buf = serialize_database(&database)?;

    match write_database_to_file(&buf, file_path) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(e.into()),
    }
}

/// Finds all collections from a database.
/// 
/// Returns the found collections.
pub fn find_all_collections_from_database(
    file_path: &Path
) -> Result<Vec<CollectionDto>, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut collections = Vec::new();
    let mut database = deserialize_database(&fs::read(file_path)?)?;

    for collection in database.collections() {
        let collection_dto = CollectionDto::from(
            collection.name()
        );
        
        collections.push(collection_dto);
    }
    
    Ok(collections)
}

/// Finds a collection from a database.
/// 
/// Returns the found collection.
pub fn find_collection_from_database(
    collection_name: &str,
    file_path: &Path
) -> Result<Option<CollectionDto>, Box<dyn Error>>
{
    if !file_path.is_file() {
        return Err(Box::new(DatabaseError::NotFound));
    }
    let mut database = deserialize_database(&fs::read(file_path)?)?;

    if collection_exists(&database, collection_name) {
        let collection_dto = CollectionDto::from(
            collection_name
        );

        return Ok(Some(collection_dto));
    }

    Ok(None)
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write, Read};
    use tempfile::tempdir;
    use std::fs::File;
    use crate::storage::pb::{
        Database,
        Collection,
    };

    #[test]
    fn test_create_collection_to_database() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        let db_buf = serialize_database(&db).unwrap();

        db.collections_mut().push(Collection::from(collection_name));
        let expected_db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        assert!(create_collection_to_database(collection_name, &file_path).is_ok());
        assert_eq!(fs::read(&file_path).unwrap(), expected_db_buf);

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn test_delete_collection_from_database() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        let expected_db_buf = serialize_database(&db).unwrap();

        db.collections_mut().push(Collection::from(collection_name));
        let db_buf = serialize_database(&db).unwrap();
    
        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        assert!(delete_collection_from_database(collection_name, &file_path).is_ok());
        assert_eq!(fs::read(&file_path).unwrap(), expected_db_buf);

        drop(file);
        dir.close().unwrap();
    }
    
    #[test]
    fn test_find_all_collections_from_database() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        db.collections_mut().push(Collection::from(collection_name));
        let db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        let collections = find_all_collections_from_database(&file_path).unwrap();
        assert_eq!(collections.get(0).unwrap().name(), collection_name);
        assert!(collections.len() == 1);

        drop(file);
        dir.close().unwrap();
    }
    
    #[test]
    fn test_find_collection() {
        let mut db = Database::from("test");
        let collection_name = "test_collection";
        db.collections_mut().push(Collection::from(collection_name));
        let db_buf = serialize_database(&db).unwrap();

        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(&format!("{}.{}", db.name(), DB_FILE_EXTENSION));
        let mut file = File::create(&file_path).unwrap();

        assert!(file.write_all(&db_buf).is_ok());
        let collection = find_collection_from_database(collection_name, &file_path).unwrap();
        assert!(collection.is_some());
        assert_eq!(collection.unwrap().name(), collection_name);

        drop(file);
        dir.close().unwrap();
    }
}