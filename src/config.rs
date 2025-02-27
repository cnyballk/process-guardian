use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessConfig {
    pub name: String,
    pub executable_path: PathBuf,
    pub arguments: Vec<String>,
    pub working_directory: Option<PathBuf>,
    pub restart_delay: u64,
    pub max_restarts: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub processes: Vec<ProcessConfig>,
}