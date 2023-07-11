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
const CONFIG_FILE_NAME: &str = "engine.config.json";

/// Engine configuration.
/// 
/// Configuration file contains configs used by the system.
/// The contents of the config file are parsed into this.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub db_dir_path: PathBuf,
    pub logs_dir_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_dir_path: PathBuf::from("./databases"),
            logs_dir_path: PathBuf::from("./logs")
        }
    }
}

/// Gets file path to config file.
pub fn get_config_file_path() -> io::Result<PathBuf> {
    let mut dir = current_exe()?;
    dir.pop();
    let file_path = dir.join(CONFIG_FILE_NAME);

    Ok(file_path)
}

/// Creates config file with default configs if it doesn't exist.
pub fn create_config_file_if_not_exists() -> io::Result<()> {
    let file_path = get_config_file_path()?;
    
    if !file_path.is_file() {
        let mut file = File::create(file_path)?;
        let config = Config::default();
        let json = serialize_config_to_json(&config)?;

        file.write_all(json.as_bytes())?;
    }

    Ok(())
}

/// Deserializes config data from JSON string.
pub fn deserialize_config_from_json(json: &str) -> serde_json::Result<Config> {
    Ok(serde_json::from_str(&json)?)
}

/// Serialized config data to JSON string.
pub fn serialize_config_to_json(config: &Config) -> serde_json::Result<String> {
    Ok(serde_json::to_string_pretty(config)?)
}

/// Reads config file and returns the contents.
pub fn read_config_file() -> io::Result<String> {
    let file_path = get_config_file_path()?;
    Ok(fs::read_to_string(file_path)?)
}

/// Writes config data to config file.
pub fn write_config_file_json(json: &str) -> io::Result<()> {
    let file_path = get_config_file_path()?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(json.as_bytes())?;

    Ok(())
}

