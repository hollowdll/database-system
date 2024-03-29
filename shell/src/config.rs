use engine::config::Config;
use crate::{
    cli::Cli,
    ask_user_input,
    event_log_failed,
    error_log_failed,
};
use std::path::Path;

fn config_save_success() {
    println!("Restart the program for the changes to take effect");
}

impl Cli {
    /// Lists the current configurations the program is using.
    pub fn list_all_configs(config: &Config) {
        println!("  Database directory: {}", config.db_dir_path().display());
        println!("  Logs directory:     {}", config.logs_dir_path().display());
    }

    /// Sets new `db_dir_path` configuration.
    pub fn set_db_dir_path(&self) {
        let input = match ask_user_input("New database directory path: ") {
            Ok(input) => input,
            Err(_) => return,
        };
        let path = Path::new(&input);

        if !path.is_dir() {
            return println!("Invalid directory path");
        }

        let result = self.engine
            .config_api()
            .set_db_dir_path(path);

        if result.success {
            event_log_failed(result.log_error);
            
            println!("Database directory path set successfully");
            config_save_success();
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: Failed to set database directory path: {}", e);
            }
        }
    }

    /// Sets new `logs_dir_path` configuration.
    pub fn set_logs_dir_path(&self) {
        let input = match ask_user_input("New logs directory path: ") {
            Ok(input) => input,
            Err(_) => return,
        };
        let path = Path::new(&input);

        if !path.is_dir() {
            return println!("Invalid directory path");
        }

        let result = self.engine
            .config_api()
            .set_logs_dir_path(path);

        if result.success {
            event_log_failed(result.log_error);

            println!("Logs directory path set successfully");
            config_save_success();
        } else {
            error_log_failed(result.log_error);

            if let Some(e) = result.error {
                eprintln!("Error: Failed to set logs directory path: {}", e);
            }
        }
    }
}
