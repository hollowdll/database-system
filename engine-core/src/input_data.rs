// This module contains code to handle input data

use crate::db::DataType;

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