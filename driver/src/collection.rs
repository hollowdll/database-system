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
            UNEXPECTED_ERROR,
        },
    },
    database::Database,
    document::{
        DocumentModel,
        DataType,
        DocumentId,
        DocumentQuery,
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
    /// 
    /// Returns the found documents.
    pub fn find_all(&self) -> Result<Vec<DocumentModel>, DatabaseClientError> {
        let result = self.client.engine
            .storage_api()
            .find_all_documents(self.database.connection_string(), self.name());

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::FindAllDocuments,
                e.message));
        }

        if result.success {
            if let Some(document_dtos) = result.data {
                let documents: Vec<DocumentModel> = document_dtos
                    .into_iter()
                    .map(|document| transform_document_dto_to_document(document))
                    .collect();

                return Ok(documents);
            }
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::FindAllDocuments,
            UNEXPECTED_ERROR.to_string()));
    }

    /// Finds documents in this collection using query.
    /// 
    /// Query contains fields with values that the document needs to match.
    /// 
    /// Returns the found documents.
    pub fn find_many(&self, query: &DocumentQuery) -> Result<Vec<DocumentModel>, DatabaseClientError> {
        let query = transform_document_data_to_input(&query.data);
        let result = self.client.engine
            .storage_api()
            .find_documents(self.database.connection_string(), self.name(), &query);

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::FindManyDocuments,
                e.message));
        }

        if result.success {
            if let Some(document_dtos) = result.data {
                let documents: Vec<DocumentModel> = document_dtos
                    .into_iter()
                    .map(|document| transform_document_dto_to_document(document))
                    .collect();

                return Ok(documents);
            }
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::FindManyDocuments,
                "Data expected but not received".to_string()));
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::FindManyDocuments,
            UNEXPECTED_ERROR.to_string()));
    }

    /// Finds a document by id in this collection.
    /// 
    /// Returns the found document.
    pub fn find_one_by_id(&self, id: &DocumentId) -> Result<Option<DocumentModel>, DatabaseClientError> {
        let result = self.client.engine
            .storage_api()
            .find_document_by_id(&id.0, self.database.connection_string(), self.name());

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::FindOneDocument,
                e.message));
        }

        if result.success {
            if let Some(data) = result.data {
                if let Some(document_dto) = data {
                    return Ok(Some(transform_document_dto_to_document(document_dto)));
                } else {
                    return Ok(None);
                }
            }
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::FindOneDocument,
            UNEXPECTED_ERROR.to_string()));
    }

    /// Inserts a document to this collection.
    /// 
    /// Returns the new document with id populated.
    pub fn insert_one(&self, document: DocumentModel) -> Result<DocumentModel, DatabaseClientError> {
        let input = transform_document_data_to_input(&document.data);

        let result = self.client.engine
            .storage_api()
            .create_document(self.database.connection_string(), self.name(), input);

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
            DatabaseClientErrorKind::InsertOneDocument,
            UNEXPECTED_ERROR.to_string()));
    }

    /// Replaces a document in this collection with a new one.
    /// 
    /// Only the data is replaced, id remains the same.
    pub fn replace_one_by_id(&self, id: &DocumentId, document: DocumentModel) -> Result<(), DatabaseClientError> {
        let input = transform_document_data_to_input(&document.data);
        
        let result = self.client.engine
            .storage_api()
            .replace_document(self.database.connection_string(), &id.0, self.name(), input);

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::ReplaceOneDocument,
                e.message));
        }

        if result.success {
            return Ok(());
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::ReplaceOneDocument,
            UNEXPECTED_ERROR.to_string()));
    }

    /// Deletes a document by id from this collection.
    pub fn delete_one_by_id(&self, id: &DocumentId) -> Result<(), DatabaseClientError> {
        let result = self.client.engine
            .storage_api()
            .delete_document(self.database.connection_string(), &id.0, self.name());

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::DeleteOneDocument,
                e.message));
        }

        if result.success {
            return Ok(());
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::DeleteOneDocument,
            UNEXPECTED_ERROR.to_string()));
    }

    /// Deletes all documents from this collection.
    /// 
    /// Returns the number of deleted documents.
    pub fn delete_all(&self) -> Result<usize, DatabaseClientError> {
        let result = self.client.engine
            .storage_api()
            .delete_all_documents(self.database.connection_string(), self.name());

        if let Some(e) = result.error {
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::DeleteManyDocuments,
                e.message));
        }

        if result.success {
            if let Some(deleted_count) = result.data {
                return Ok(deleted_count);
            }
            return Err(DatabaseClientError::new(
                DatabaseClientErrorKind::DeleteManyDocuments,
                "Data expected but not received".to_string()));
        }

        return Err(DatabaseClientError::new(
            DatabaseClientErrorKind::DeleteManyDocuments,
            UNEXPECTED_ERROR.to_string()));
    }
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

/// Transforms driver document data to engine input data.
fn transform_document_data_to_input(data: &HashMap<String, DataType>) -> Vec<DocumentInputDataField> {
    let mut input = Vec::new();
    for (key, value) in data {
        let (data_type, data_value) = match value {
            DataType::Int32(v) => ("Int32", v.to_string()),
            DataType::Int64(v) => ("Int64", v.to_string()),
            DataType::Decimal(v) => ("Decimal", v.to_string()),
            DataType::Bool(v) => ("Bool", v.to_string()),
            DataType::Text(v) => ("Text", v.to_string()),
        };

        input.push(DocumentInputDataField::new(&key, data_type, &data_value));
    }

    return input;
}
