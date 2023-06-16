// This module contains code to handle input data

use crate::db::DataType;
use crate::db::error::ConvertError;

/// Input data field which is used to create fields to documents
pub struct InputDataField {
    field: String,
    data_type: String,
    value: String,
}

impl InputDataField {
    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn data_type(&self) -> &str {
        &self.data_type
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    /// Converts input data to correct database document data type.
    pub fn convert_to_document_data_type(&self, input_data: &str, data_type: &str) -> Result<DataType, ConvertError> {
        match data_type {
            "Int32" => {
                match input_data.parse::<i32>() {
                    Ok(data) => return Ok(DataType::Int32(data)),
                    Err(_) => return Err(ConvertError::Int32),
                };
            },
            "Int64" => {
                match input_data.parse::<i64>() {
                    Ok(data) => return Ok(DataType::Int64(data)),
                    Err(_) => return Err(ConvertError::Int64),
                };
            },
            "Decimal" => {
                match input_data.parse::<f64>() {
                    Ok(data) => return Ok(DataType::Decimal(data)),
                    Err(_) => return Err(ConvertError::Decimal),
                };
            },
            "Bool" => {
                match input_data.to_lowercase().parse::<bool>() {
                    Ok(data) => return Ok(DataType::Bool(data)),
                    Err(_) => return Err(ConvertError::Bool),
                };
            },
            "Text" => {
                match input_data.parse::<String>() {
                    Ok(data) => return Ok(DataType::Text(data)),
                    Err(_) => return Err(ConvertError::Text),
                };
            },
            _ => return Err(ConvertError::Unknown),
        }
    }
}

impl InputDataField {
    pub fn from(field: &str, data_type: &str, value: &str) -> Self {
        Self {
            field: field.to_string(),
            data_type: data_type.to_string(),
            value: value.to_string(),
        }
    }
}
