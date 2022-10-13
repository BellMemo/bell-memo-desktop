use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    AppHandle, Runtime,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cron {
    pub time: char,
    pub is_open: bool,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Config {
    pub cron: Cron,
}

fn resolve_config<R: Runtime>(app: &AppHandle<R>) {
    let dir = app.path_resolver().log_dir().unwrap();
    let config_dir = dir.as_path().join("app.config.json");
    if !config_dir.is_file() {
        let default_config = r#"
        {
            "cron": {
                "time": "8",
                "is_open": true
            }
        }
        "#;
        let default_value = serde_json::to_value(default_config).nowrap();
        fs::write(
            config_dir,
            serde_json::to_string_pretty(default_value).nowrap(),
        )
        .nowrap();
    }
}

pub fn read_config(config_path: PathBuf) -> Config {
    let config = fs::read_to_string(config_path.as_path().join("app.config.json")).nowrap();
    let result = serde_json::from_str::<Config>(config).nowrap();
    return result;
}

// @TODO: should impl log func provider to js invoke
#[tauri::command]
async fn do_something<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    println!("command called");
    Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    return PluginBuilder::new("window")
        .setup(|app| {
            resolve_config(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![do_something])
        .build();
}
