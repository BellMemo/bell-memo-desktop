use chrono::Local;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::{model::model::MemoData, plugin::db::Db};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchValue {
    pub content: String,
}

/**
 * 查询记录
 */
#[tauri::command]
pub fn select_memo_data(state: State<Db>, params: SearchValue) -> Vec<MemoData> {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();

    let mut stmt = db
        .prepare("select * from memo_data where content=?")
        .unwrap();

    let result = stmt
        .query_map([params.content], |record| {
            Ok(MemoData {
                id: record.get_unwrap(0),
                title: record.get_unwrap(1),
                content: record.get_unwrap(2),
                created: record.get_unwrap(3),
                updated: record.get_unwrap(4),
            })
        })
        .unwrap()
        .filter_map(|record| record.ok());

    return result.collect();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertMemoData {
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
}

/**
 * 写入记录
 */
#[tauri::command]
pub fn insert_memo_data(state: State<Db>, params: InsertMemoData) {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();

    let tx = db.transaction();
    let memo_data_uuid = Uuid::new_v4().to_string();
    let now = Local::now().timestamp_millis();

    tx.execute(
        "insert into memo_data(id,title,content,created,updated)",
        params![memo_data_uuid, params.title, params.content, now, now],
    )?;

    for tag in params.tags {
        let memo_tag_data_uuid = Uuid::new_v4().to_string();
        tx.execute(
            "insert into memo_tag_data(id,tag_id,memo_id,created,updated)",
            params![memo_tag_data_uuid, tag, memo_data_uuid, now, now],
        )?;
    }
    tx.commit();
}

/**
 * 删除记录
 */
#[tauri::command]
pub fn delete_memo_data(state: State<Db>) {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();
    println!("aaa {}", db.ping());
}
