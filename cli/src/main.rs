// Database engine CLI client

use std::path::PathBuf;
use cli::{
    Cli,
    run,
};
use engine_core::{
    Logger,
    Config,
};

fn main() {
    let config = Config::build();
    let logger = Logger::build(PathBuf::from(&config.logs_dir_path));
    let cli = Cli::build(config, &logger);
    run(cli);
}
