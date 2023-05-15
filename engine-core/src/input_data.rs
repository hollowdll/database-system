// This module contains code to handle input data

use crate::db::DataType;

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

/// Converts input data from string
/// to correct database document data type.
pub fn convert_input_data(input_data: &str, data_type: &str) -> Option<DataType> {
    match data_type {
        "Int32" => {
            match input_data.parse::<i32>() {
                Ok(data) => return Some(DataType::Int32(data)),
                Err(e) => eprintln!("Failed to convert input data to 'Int32': {e}"),
            };
        },
        "Int64" => {
            match input_data.parse::<i64>() {
                Ok(data) => return Some(DataType::Int64(data)),
                Err(e) => eprintln!("Failed to convert input data to 'Int64': {e}"),
            };
        },
        "Decimal" => {
            match input_data.parse::<f64>() {
                Ok(data) => return Some(DataType::Decimal(data)),
                Err(e) => eprintln!("Failed to convert input data to 'Decimal': {e}"),
            };
        },
        "Bool" => {
            match input_data.parse::<bool>() {
                Ok(data) => return Some(DataType::Bool(data)),
                Err(e) => eprintln!("Failed to convert input data to 'Bool': {e}"),
            };
        },
        "Text" => {
            match input_data.parse::<String>() {
                Ok(data) => return Some(DataType::Text(data)),
                Err(e) => eprintln!("Failed to convert input data to 'Text': {e}"),
            };
        },
        _ => return None,
    }

    return None;
}