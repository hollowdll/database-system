use tempfile::{tempdir, TempDir};

// Holds temporary directories.
// Directories are removed after test has completed.
pub struct Config {
    pub db_dir: TempDir,
    pub logs_dir: TempDir,
}

impl Config {
    pub fn new() -> Self {
        let db_dir = tempdir().unwrap();
        let logs_dir = tempdir().unwrap();

        Self { db_dir, logs_dir }
    }

    // Close temp dirs and consume self.
    // We will know in tests if they error.
    pub fn close_temp_dirs(self) {
        self.db_dir.close().unwrap();
        self.logs_dir.close().unwrap();
    }
}