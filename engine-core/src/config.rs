use std::path::PathBuf;

use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize, Deserialize)]
pub struct ConfigDto {
    db_dir_path: PathBuf,
    logs_dir_path: PathBuf,
}

