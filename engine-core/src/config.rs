// Module used to read from and write to config file

#![allow(unused)]

use std::{
    io::{
        self,
        Write,
    },
    path::PathBuf,
    env::current_exe,
    fs::{
        self,
        File, OpenOptions,
    },
};
use serde::{
    Serialize,
    Deserialize,
};

/// Configuration file name. Config file uses JSON format.
/// 
/// Configuration file contains configs used by the system.
const CONFIG_FILE_NAME: &str = "engine.config.json";

#[derive(Serialize, Deserialize)]
pub struct ConfigDto {
    db_dir_path: PathBuf,
    logs_dir_path: PathBuf,
}

impl Default for ConfigDto {
    fn default() -> Self {
        Self {
            db_dir_path: PathBuf::from("./databases"),
            logs_dir_path: PathBuf::from("./logs")
        }
    }
}

fn get_config_file_path() -> io::Result<PathBuf> {
    let mut dir = current_exe()?;
    dir.pop();
    let file_path = dir.join(CONFIG_FILE_NAME);

    Ok(file_path)
}

pub fn create_config_file_if_not_exists() -> io::Result<()> {
    let file_path = get_config_file_path()?;
    
    if !file_path.is_file() {
        File::create(file_path)?;
    }

    Ok(())
}

fn deserialize_config_from_json(json: &str) -> serde_json::Result<ConfigDto> {
    Ok(serde_json::from_str(&json)?)
}

fn serialize_config_to_json(config: &ConfigDto) -> serde_json::Result<String> {
    Ok(serde_json::to_string(config)?)
}

pub fn read_config_file() -> io::Result<String> {
    let file_path = get_config_file_path()?;
    Ok(fs::read_to_string(file_path)?)
}

pub fn write_config_file_json(json: &str) -> io::Result<()> {
    let file_path = get_config_file_path()?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(json.as_bytes())?;

    Ok(())
}

