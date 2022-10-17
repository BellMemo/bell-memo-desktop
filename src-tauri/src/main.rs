#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod cmd;
mod plugin;
mod util;
mod model;

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(plugin::config::init())
        .plugin(plugin::log::init())
        .plugin(plugin::timer::init())
        .setup(|app| {
            // 在log初始化之后 进行DB初始化 避免无法打印日志
            let db = model::db::Database::new();
            // test db init success
            db.ping();

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
