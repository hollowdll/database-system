// Module used to read from and write to config file

#![allow(unused)]

pub mod api;

use std::{
    io::{
        self,
        Write,
    },
    path::{
        PathBuf,
        Path,
    },
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
use crate::Logger;

/// Configuration file name. Config file uses JSON format.
pub const CONFIG_FILE_NAME: &str = "engine.config.json";

/// Engine configuration.
/// 
/// Configuration file contains configs used by the system.
/// The contents of the config file are parsed into this.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db_dir_path: PathBuf,
    pub logs_dir_path: PathBuf,
}

impl Config {
    pub fn db_dir_path(&self) -> &Path {
        &self.db_dir_path
    }

    pub fn logs_dir_path(&self) -> &Path {
        &self.logs_dir_path
    }
}

impl Config {
    /// Creates a new config.
    pub fn new(
        db_dir_path: &Path,
        logs_dir_path: &Path,
    ) -> Config
    {
        Config {
            db_dir_path: PathBuf::from(db_dir_path),
            logs_dir_path: PathBuf::from(logs_dir_path)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_dir_path: PathBuf::from("./databases"),
            logs_dir_path: PathBuf::from("./logs")
        }
    }
}

/// Configuration manager.
/// 
/// Manages configuration loading and changes.
pub struct ConfigManager<'a> {
    config: &'a Config,
    config_file_path: PathBuf,
}

impl<'a> ConfigManager<'a> {
    /// Builds config manager.
    pub fn build(config: &'a Config) -> Self {
        Self {
            config,
            config_file_path: get_config_file_path(),
        }
    }
}

impl<'a> ConfigManager<'a> {
    fn config_file_path(&self) -> &Path {
        &self.config_file_path
    }
}

impl<'a> ConfigManager<'a> {
    /// Loads configuration data from config file.
    /// 
    /// Configuration loading is intended to be done only once.
    /// 
    /// Will panic if cannot get config file path.
    pub fn load_config() -> io::Result<Config> {
        let file_path = get_config_file_path();
        create_config_file_if_not_exists(&file_path)?;
        let contents = read_config_file(&file_path)?;
        let config = deserialize_config_from_json(&contents)?;

        Ok(config)
    }

    /// Sets database directory path config and saves it to config file.
    /// 
    /// A program restart is required for the changes to take effect.
    pub fn set_db_dir_path(&self, path: &Path) -> io::Result<()> {
        let new_config = Config::new(
            path,
            &self.config.logs_dir_path(),
        );
        save_config(self.config_file_path(), &new_config)?;

        Ok(())
    }

    /// Sets logs directory path config and saves it to config file.
    /// 
    /// A program restart is required for the changes to take effect.
    pub fn set_logs_dir_path(&self, path: &Path) -> io::Result<()> {
        let new_config = Config::new(
            &self.config.db_dir_path(),
            path,
        );
        save_config(self.config_file_path(), &new_config)?;

        Ok(())
    }
}

/// Sets default directory paths to config.
fn set_default_config_dir_paths(config: &mut Config) -> io::Result<()> {
    let mut dir = current_exe()?;
    dir.pop();

    config.db_dir_path = dir.join("databases");
    config.logs_dir_path = dir.join("logs");

    Ok(())
}

/// Gets file path to config file.
/// 
/// Use this when building `ConfigManager`.
fn get_config_file_path() -> PathBuf {
    let mut dir = match current_exe() {
        Ok(dir) => dir,
        Err(e) => {
            // Panic if a system error occurs to ensure only valid file path will be used.
            panic!("Failed to get config file path due to system error. Try again or fix the problem. {}", e);
        },
    };
    dir.pop();
    let file_path = dir.join(CONFIG_FILE_NAME);

    file_path
}

/// Deserializes config data from JSON string.
fn deserialize_config_from_json(json: &str) -> serde_json::Result<Config> {
    Ok(serde_json::from_str(&json)?)
}

/// Serialized config data to JSON string.
fn serialize_config_to_json(config: &Config) -> serde_json::Result<String> {
    Ok(serde_json::to_string_pretty(config)?)
}

/// Creates config file with default configs if it doesn't exist.
fn create_config_file_if_not_exists(file_path: &Path) -> io::Result<()> {
    if !file_path.is_file() {
        let mut file = File::create(file_path)?;
        let mut config = Config::default();
        set_default_config_dir_paths(&mut config)?;
        let json = serialize_config_to_json(&config)?;

        file.write_all(json.as_bytes())?;
    }

    Ok(())
}

/// Reads config file and returns the contents.
fn read_config_file(file_path: &Path) -> io::Result<String> {
    Ok(fs::read_to_string(file_path)?)
}

/// Writes json to config file.
fn write_config_file(file_path: &Path, json: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(json.as_bytes())?;

    Ok(())
}

/// Saves configuration data to config file.
fn save_config(file_path: &Path, config: &Config) -> io::Result<()> {
    let json = serialize_config_to_json(config)?;
    write_config_file(file_path, &json)?;

    Ok(())
}
