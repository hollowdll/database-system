use std::{
    io,
    path::Path,
};
use super::*;

/// Configuration manager.
/// 
/// Manages configuration loading and changes.
pub struct ConfigManager<'a> {
    config: &'a Config,
}

impl<'a> ConfigManager<'a> {
    /// Builds config manager.
    pub fn build(config: &'a Config) -> Self {
        Self {
            config,
        }
    }
}

impl<'a> ConfigManager<'a> {
    fn config_file_path(&self) -> &Path {
        &self.config.config_file_path
    }
}

impl<'a> ConfigManager<'a> {
    /// Sets database directory path config and saves it to config file.
    /// 
    /// A program restart is required for the changes to take effect.
    pub fn set_db_dir_path(&self, path: &Path) -> io::Result<()> {
        let new_config = Config::new(
            &self.config.config_file_path(),
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
            &self.config.config_file_path(),
            &self.config.db_dir_path(),
            path,
        );
        save_config(self.config_file_path(), &new_config)?;

        Ok(())
    }
}
