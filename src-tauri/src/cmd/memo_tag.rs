use std::rc::Rc;

use chrono::Local;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::{model::model::MemoTag, plugin::db::Db};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchTagValue {
    pub content: String,
    pub offset: u64,
    pub limit: u64,
}

/**
 * 查询记录
 */
#[tauri::command]
pub fn search_memo_tag(state: State<Db>, params: SearchTagValue) -> Vec<MemoTag> {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();
    let mut stmt = db
        .prepare("select * from memo_tag where name like ?1")
        .unwrap();

    let result = stmt
        .query_map(params![format!("%{}%", params.content)], |record| {
            Ok(MemoTag {
                id: record.get("id")?,
                name: record.get("name")?,
                created: record.get("created")?,
                updated: record.get("updated")?,
            })
        })
        .unwrap()
        .filter_map(|record| record.ok());

    return result.collect();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertMemoTag {
    content: String,
}
/**
 * 写标签
 */
#[tauri::command]
pub fn insert_memo_tag(state: State<Db>, params: InsertMemoTag) -> MemoTag {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();

    let _is_tag_exist: String = db
        .query(
            "select id from memo_tag where name=?",
            [params.content.clone()],
            |record| record.get(0),
        )
        .unwrap_or_default();

    let uuid = Uuid::new_v4().to_string();
    let now = Local::now().timestamp_millis();

    let _result = db
        .exec(
            "insert into memo_tag(id,name,created,updated) values(?1,?2,?3,?4)",
            params![uuid, params.content, now, now],
        )
        .unwrap();

    return MemoTag {
        id: uuid,
        name: params.content,
        created: now as u64,
        updated: now as u64,
    };
}
