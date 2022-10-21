use serde::{Deserialize, Serialize};
use tauri::State;

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
        .prepare("select * from memo_data where content=?;")
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

/**
 * 写入记录
 */
#[tauri::command]
pub fn insert_memo_data(state: State<Db>) {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();
    println!("aaa {}", db.ping());
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
