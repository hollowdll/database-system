// This module contains setup code for integration tests

use engine::config::Config;
use tempfile::{tempdir, TempDir};

// Holds temporary directories.
// Directories are removed after test has completed.
pub struct ConfigSettings {
    pub db_dir: TempDir,
    pub logs_dir: TempDir,
    pub config: Config,
}

impl ConfigSettings {
    pub fn new() -> Self {
        let db_dir = tempdir().unwrap();
        let logs_dir = tempdir().unwrap();
        let config = Config::new(db_dir.path(), logs_dir.path());

        Self { db_dir, logs_dir, config }
    }
}
