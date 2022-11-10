use serde::{Deserialize, Serialize};
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
pub fn save_data(app: AppHandle, state: State<Db>) {
    let data_path_pf = app
        .path_resolver()
        .app_config_dir()
        .unwrap()
        .join("data.json");

    let data_path = data_path_pf.to_str().unwrap();

    // 文件存在先删除
    if !Path::new(data_path).is_file() {
        fs::remove_file(data_path).unwrap();
    }

    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();

    let mut memo_data_stmt = db.prepare("select * from memo_data").unwrap();
    let memo_data = memo_data_stmt
        .query_map([], |row| {
            Ok(MemoData {
                id: row.get("id")?,
                title: row.get("title")?,
                content: row.get("content")?,
                created: row.get("created")?,
                updated: row.get("updated")?,
            })
        })
        .unwrap()
        .filter_map(|row| row.ok())
        .collect();

    let mut memo_tag_stmt = db.prepare("select * from memo_tag").unwrap();
    let memo_tag = memo_tag_stmt
        .query_map([], |record| {
            Ok(MemoTag {
                id: record.get("id")?,
                name: record.get("name")?,
                created: record.get("created")?,
                updated: record.get("updated")?,
            })
        })
        .unwrap()
        .filter_map(|row| row.ok())
        .collect();

    let mut memo_tag_data_stmt = db.prepare("select * from memo_tag_data").unwrap();
    let memo_tag_data = memo_tag_data_stmt
        .query_map([], |row| {
            Ok(MemoTagData {
                id: row.get("id")?,
                tag_id: row.get("tag_id")?,
                memo_id: row.get("memo_id")?,
                created: row.get("created")?,
                updated: row.get("updated")?,
            })
        })
        .unwrap()
        .filter_map(|row| row.ok())
        .collect();

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
