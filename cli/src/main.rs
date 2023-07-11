// Database engine CLI client

use std::path::PathBuf;
use cli::{
    Cli,
    run,
    load_config,
};
use engine_core::{
    Logger,
    config::Config,
};

fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            // Use default configs if config loading fails.
            // Might be changed in the future.
            eprintln!("Failed to load configs from config file: {}", e);
            println!("Using default configs. Restart the program to try again.\n");
            Config::default()
        },
    };
    let logger = Logger::build(PathBuf::from(&config.logs_dir_path));
    let cli = Cli::build(config, &logger);
    run(cli);
}
