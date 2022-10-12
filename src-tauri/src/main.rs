#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
mod plugin;

fn main() {
    tauri::Builder::default()
        .plugin(plugin::log::init())
        .setup(|app| {
            // 获取日志路径
            let log_dir = app.path_resolver().log_dir().unwrap();
            println!("{}", log_dir.as_path().display());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmd::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
