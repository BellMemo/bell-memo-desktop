#[tauri::command]
pub fn greet(name: &str) -> String {
    log::info!("test info");
    format!("Hello, {}! You've been greeted from Rust!", name)
}
