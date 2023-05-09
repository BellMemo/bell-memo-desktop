use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use std::{fs, io::BufWriter, path::Path, process::Command};
use tauri::{api::dialog, AppHandle, State};

use crate::{
    model::model::{MemoData, MemoTag, MemoTagData},
    plugin::db::Db,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportDataStruct {
    memo_data: Vec<MemoData>,
    memo_tag: Vec<MemoTag>,
    memo_tag_data: Vec<MemoTagData>,
}

/**
 * 导出数据生成JSON文件
 */
#[tauri::command]
pub async fn save_data(app: AppHandle, state: State<'_, Db>) -> Result<(),()> {
    let data_path_pf = app
        .path_resolver()
        .app_config_dir()
        .unwrap()
        .join("data.json");

    let data_path = data_path_pf.to_str().unwrap();

    // 文件存在先删除
    if Path::new(data_path).is_file() {
        fs::remove_file(data_path).unwrap();
    }

    let db = state.connection.lock().await;
    let mut conn = db.get_connection().await;

    let memo_data = sqlx::query_as::<Sqlite, MemoData>("select * from memo_data")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    let memo_tag = sqlx::query_as::<Sqlite, MemoTag>("select * from memo_tag")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    let memo_tag_data = sqlx::query_as::<Sqlite, MemoTagData>("select * from memo_tag_data")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    let export_data = ExportDataStruct {
        memo_data: memo_data,
        memo_tag: memo_tag,
        memo_tag_data: memo_tag_data,
    };

    let file = fs::File::create(data_path).unwrap();
    let buffer_write = BufWriter::new(file);

    serde_json::to_writer(buffer_write, &export_data).unwrap();

    // open folder
    if cfg!(target_os = "macos") {
        Command::new("open")
            .args(["-R", data_path])
            .spawn()
            .unwrap();
    }
    // open file explorer
    if cfg!(target_os = "windows") {
        Command::new("explorer")
            .args(["/select,", data_path])
            .spawn()
            .unwrap();
    }
    Ok(())
}

#[tauri::command(async)]
pub fn import_data() {
    // sync dialog api crashed https://github.com/tauri-apps/tauri/issues/5607
    let file_path_buf = dialog::blocking::FileDialogBuilder::new()
        .add_filter("JSON", &["json"])
        .pick_file()
        .unwrap_or_default();
    let file_path = file_path_buf.as_path().to_str().unwrap();
    println!("{}", file_path);
}
