use std::process::Command;
use tauri::{api::dialog, AppHandle};

/**
 * 导出数据生成JSON文件
 */
#[tauri::command]
pub fn save_data(app: AppHandle) {
    let log_path = app
        .path_resolver()
        .app_config_dir()
        .unwrap()
        .join("data.json");

    // open folder
    if cfg!(target_os = "macos") {
        Command::new("open")
            .args(["-R", log_path.as_path().to_str().unwrap()])
            .spawn()
            .unwrap();
    }
    // open file explorer
    if cfg!(target_os = "windows") {
        Command::new("explorer")
            .args(["/select,", log_path.as_path().to_str().unwrap()])
            .spawn()
            .unwrap();
    }
}

#[tauri::command]
pub fn import_data() {
    dialog::FileDialogBuilder::new()
        .add_filter("JSON", &["json"])
        .pick_file(|file_path| match file_path {
            Some(p) => {
                println!("{:#?}", p.as_path().to_str());
            }
            _ => {}
        });
}
