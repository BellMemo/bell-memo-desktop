#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let desc = app.package_info().description;
            println!("{}", desc);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmd::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
