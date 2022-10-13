use tauri::State;

use crate::plugin::config::Config;

#[tauri::command]
pub fn greet(name: &str, state: State<Config>) -> String {
    println!("{}", state.inner().cron.is_open);
    log::info!("test info");
    format!("Hello, {}! You've been greeted from Rust!", name)
}
