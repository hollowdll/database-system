use std::{
    io,
    path::{
        PathBuf,
        Path,
    },
};
use super::*;

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
    /// Loads configuration data from config file. Creates the file
    /// with default configs if it doesn't exist.
    /// 
    /// Configuration loading is intended to be done only once.
    pub fn load_config(file_path: &Path) -> io::Result<Config> {
        create_config_file_if_not_exists(file_path)?;
        let contents = read_config_file(file_path)?;
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
