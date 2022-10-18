#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod cmd;
mod model;
mod plugin;
mod util;

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(plugin::config::init())
        .plugin(plugin::log::init())
        .plugin(plugin::timer::init())
        .plugin(plugin::db::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmd::greet])
        .run(context)
        .expect("error while running tauri application");
}
