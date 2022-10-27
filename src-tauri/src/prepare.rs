use crate::{constants::APP_NAME, plugin::log::log_init};
use std::{fs, path::Path};
use tauri::{api::path, Config as TauriConfig};

// check some folders already existed
fn check_folder(config: &mut TauriConfig) {
    let log_folder = path::log_dir(&config).unwrap().join(APP_NAME);

    if !log_folder.is_dir() {
        let log_folder_path = log_folder.to_str().unwrap();
        fs::create_dir(log_folder_path).unwrap();
    }

    let config_folder = path::config_dir().unwrap().join(APP_NAME);

    if !config_folder.is_dir() {
        let config_folder_path = config_folder.to_str().unwrap();
        fs::create_dir(config_folder_path).unwrap();
    }
}

// application instance init before should be called
pub fn prepare(config: &mut TauriConfig) {
    check_folder(config);

    let log_dir = path::log_dir(&config).unwrap();
    log_init(log_dir);
}
