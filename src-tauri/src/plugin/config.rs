use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Runtime,
};

use crate::{constants::APP_NAME, util};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cron {
    // 触发周期
    #[serde(default)]
    pub time: String,
    // 是否开启定时任务
    #[serde(default)]
    pub is_open: bool,
    // 是否在应用启动时触发
    #[serde(default)]
    pub immediately: bool,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Config {
    pub cron: Cron,
}

fn resolve_config() {
    let file_path = util::app_path::app_config_path()
        .join(APP_NAME)
        .join("app.config.json");

    if !file_path.is_file() {
        let default_config = Config {
            cron: Cron {
                time: String::from("8"),
                is_open: true,
                immediately: true,
            },
        };
        let default_value = serde_json::to_string(&default_config).unwrap();
        fs::write(file_path, default_value).unwrap();
    }
}

pub fn read_config() -> Config {
    let file_path = util::app_path::app_config_path()
        .as_path()
        .join("app.config.json");
    let config = fs::read_to_string(file_path).unwrap();
    let result: Config = serde_json::from_str(&config).unwrap();
    return result;
}

#[tauri::command]
fn get<R: Runtime>(_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<Config, String> {
    let config = read_config();
    Ok(config)
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    return PluginBuilder::new("config")
        .setup(|app| {
            resolve_config();
            let config = read_config();
            // 默认启动注入全局配置
            app.manage(config);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get])
        .build();
}
