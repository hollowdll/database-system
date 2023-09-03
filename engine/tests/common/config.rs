use engine::config::{Config, CONFIG_FILE_NAME};
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
        let config_dir = tempdir().unwrap();
        let config_file_path = config_dir.path().join(CONFIG_FILE_NAME);
        let db_dir = tempdir().unwrap();
        let logs_dir = tempdir().unwrap();
        
        let config = Config::new(
            &config_file_path,
            db_dir.path(),
            logs_dir.path()
        );

        Self { db_dir, logs_dir, config }
    }
}
