use tauri::{api::path, Config as TauriConfig};

use crate::plugin::log::log_init;

// application instance init before should be called
pub fn prepare(config: &mut TauriConfig) {
    let log_dir = path::log_dir(&config).unwrap();

    log_init(log_dir)
}
