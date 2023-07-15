use engine_core::config::{
    Config,
    save_config,
};
use crate::ask_user_input;
use std::path::Path;

/// Lists the current configurations the program is using.
pub fn list_all_configs(config: &Config) {
    println!("  Databases directory: {:?}", config.db_dir_path());
    println!("  Logs directory:      {:?}", config.logs_dir_path());
}

/// Sets new `db_dir_path` config and saves it to config file.
pub fn set_db_dir_path(config: &Config) {
    let input = match ask_user_input("New database directory path: ") {
        Ok(input) => input,
        Err(_) => return,
    };
    let path = Path::new(&input);

    if path.is_dir() {
        let new_config = Config::new(
            path,
            config.logs_dir_path()
        );
        match save_config(&new_config) {
            Ok(()) => {
                println!("Configurations set successfully.");
                println!("Restart the program for the changes to take effect.");
            },
            Err(e) => eprintln!("Failed to set configuration: {}", e),
        };
    } else {
        println!("Invalid directory path");
    }
}
