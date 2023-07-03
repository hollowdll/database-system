// This module contains code to handle input data

use crate::db::{
    error::ParseError,
    pb::document::{
        DataType,
        data_type,
    }
};

/// Input data field which is used to create fields to documents.
pub struct DocumentInputDataField {
    field: String,
    data_type: String,
    value: String,
}

impl DocumentInputDataField {
    /// Creates a new document input data field.
    pub fn new(field: &str, data_type: &str, value: &str) -> Self {
        Self {
            field: field.to_string(),
            data_type: data_type.to_string(),
            value: value.to_string(),
        }
    }
}

/// Creates a new data type value from `pb::document::data_type::DataType` enum.
fn create_new_data_type_value(data_type: data_type::DataType) -> DataType {
    DataType {
        data_type: Some(data_type)
    }
}

impl DocumentInputDataField {
    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn data_type(&self) -> &str {
        &self.data_type
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    /// Parses input data into correct database document data type.
    pub fn parse_to_document_data_type(
        &self,
        input_data: &str,
        data_type: &str
    ) -> Result<DataType, ParseError>
    {
        match data_type {
            "Int32" => {
                match input_data.parse::<i32>() {
                    Ok(data) => return Ok(
                        create_new_data_type_value(data_type::DataType::Int32(data))
                    ),
                    Err(_) => return Err(ParseError::Int32),
                };
            },
            "Int64" => {
                match input_data.parse::<i64>() {
                    Ok(data) => return Ok(
                        create_new_data_type_value(data_type::DataType::Int64(data))
                    ),
                    Err(_) => return Err(ParseError::Int64),
                };
            },
            "Decimal" => {
                match input_data.parse::<f64>() {
                    Ok(data) => return Ok(
                        create_new_data_type_value(data_type::DataType::Decimal(data))
                    ),
                    Err(_) => return Err(ParseError::Decimal),
                };
            },
            "Bool" => {
                match input_data.to_lowercase().parse::<bool>() {
                    Ok(data) => return Ok(
                        create_new_data_type_value(data_type::DataType::Bool(data))
                    ),
                    Err(_) => return Err(ParseError::Bool),
                };
            },
            "Text" => {
                match input_data.parse::<String>() {
                    Ok(data) => return Ok(
                        create_new_data_type_value(data_type::DataType::Text(data))
                    ),
                    Err(_) => return Err(ParseError::Text),
                };
            },
            _ => return Err(ParseError::Unknown),
        }
    }
}
