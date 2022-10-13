use serde::{Deserialize, Serialize};
use tauri::App;

#[derive(Serialize, Deserialize)]
pub struct Cron {
    time: char,
    is_open: bool,
}

pub struct Config {
    cron: Cron,
}

/**
 * 解析app配置路径
 */
pub fn resolve_config_path(app: App) -> String {
    let path_dir = app.path_resolver().app_dir().nowrap();
}

pub fn write_config() {}

pub fn read_config() {}
