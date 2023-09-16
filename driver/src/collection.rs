use std::collections::HashMap;
use engine::{
    storage::{
        error::DatabaseOperationError,
        document::DocumentDto,
        pb::document::data_type,
    },
    DocumentInputDataField,
};
use crate::{
    client::{
        DatabaseClient,
        error::{
            DatabaseClientError,
            DatabaseClientErrorKind,
        },
    },
    database::Database,
    document::{
        DocumentModel,
        DataType,
        DocumentId
    },
};

/// Collection API.
/// 
/// This provides methods to work with a collection's documents.
/// 
/// T = type for this collection's document model.
pub struct Collection<'a> {
    client: &'a DatabaseClient,
    database: &'a Database<'a>,
    name: String,
}

impl<'a> Collection<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<'a> Collection<'a> {
    pub fn new(client: &'a DatabaseClient, database: &'a Database, name: &str) -> Collection<'a> {
        Collection {
            client,
            database,
            name: name.to_string(),
        }
    }

    /// Finds all documents in this collection.
    pub fn find_all() -> Result<Vec<DocumentModel>, DatabaseOperationError> {
        Ok(Vec::new())
    }

    /// Finds a document by id from this collection.
    pub fn find_one_by_id(id: &u64) -> Result<Option<DocumentModel>, DatabaseOperationError> {
        Ok(None)
    }

    /// Inserts a document to this collection.
    /// 
    /// Returns the new document with id populated.
    pub fn insert_one(&self, document: DocumentModel) -> Result<DocumentModel, DatabaseClientError> {
        let input_data = transform_document_to_input(document);

        let result = self.client.engine
            .storage_api()
            .create_document(self.database.connection_string(), self.name(), input_data);

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::InsertOneDocument,
                e.message));
        }

        if result.success {
            if let Some(document) = result.data {
                return Ok(transform_document_dto_to_document(document));
            }
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::Unexpected,
            "Failed to insert a document".to_string()));
    }

    /// Replaces a document in this collection with a new one.
    /// 
    /// Only the data is replaced, id remains the same.
    pub fn replace_one_by_id(id: &u64) -> Result<(), DatabaseOperationError> {
        Ok(())
    }

    /// Deletes a document in this collection.
    pub fn delete_one_by_id(id: &u64) -> Result<(), DatabaseOperationError> {
        Ok(())
    }
}

/// Transforms driver document model to engine input data.
fn transform_document_to_input(document: DocumentModel) -> Vec<DocumentInputDataField> {
    let mut data = Vec::new();

    for (key, value) in document.data {
        let (data_type, data_value) = match value {
            DataType::Int32(v) => ("Int32", v.to_string()),
            DataType::Int64(v) => ("Int64", v.to_string()),
            DataType::Decimal(v) => ("Decimal", v.to_string()),
            DataType::Bool(v) => ("Bool", v.to_string()),
            DataType::Text(v) => ("Text", v.to_string()),
        };

        data.push(DocumentInputDataField::new(&key, data_type, &data_value));
    }

    return data;
}

/// Transforms engine `DocumentDto` to driver document model.
fn transform_document_dto_to_document(document_dto: DocumentDto) -> DocumentModel {
    let mut data = HashMap::new();

    for (key, value) in document_dto.data {
        let data_type = match value.data_type {
            Some(data_type::DataType::Int32(v)) => DataType::Int32(v),
            Some(data_type::DataType::Int64(v)) => DataType::Int64(v),
            Some(data_type::DataType::Decimal(v)) => DataType::Decimal(v),
            Some(data_type::DataType::Bool(v)) => DataType::Bool(v),
            Some(data_type::DataType::Text(v)) => DataType::Text(v),
            _ => continue,
        };

        data.insert(key, data_type);
    }

    return DocumentModel {
        id: DocumentId(document_dto.id),
        data
    };
}
