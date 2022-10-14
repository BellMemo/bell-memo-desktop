use std::path::PathBuf;
use tauri::api::path;

#[warn(dead_code, unused)]
pub fn app_config_path() -> PathBuf {
    return path::config_dir().unwrap();
}
