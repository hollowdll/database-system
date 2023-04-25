// This module contains code to handle input data

use std::error::Error;
use crate::db::DataType;

/// Converts input data from string
/// to correct database document data type.
pub fn convert_input_data(input_data: &str, data_type: &str) -> Option<DataType> {
    match data_type {
        "Int" => {
            match input_data.parse::<i32>() {
                Ok(data) => return Some(DataType::Int(data)),
                Err(e) => eprintln!("Failed to convert input data: {e}"),
            };
        },
        "BigInt" => {

        },
        "Decimal" => {

        },
        "Bool" => {

        },
        "Text" => {

        },
        _ => return None,
    }

    return None;
}