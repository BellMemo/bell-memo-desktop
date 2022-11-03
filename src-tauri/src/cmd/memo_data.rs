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

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchTagValue {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchMemoData {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<SearchTagValue>,
}

/**
 * 查询记录
 */
#[tauri::command]
pub fn select_memo_data(state: State<Db>, params: SearchValue) -> Vec<SearchMemoData> {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();

    let mut stmt = db
        .prepare(
            "SELECT DISTINCT memo_data.id, memo_data.content, memo_data.title, memo_data.created, memo_data.updated
            FROM memo_data, memo_tag
            WHERE name LIKE ?1
                OR title LIKE ?2
                OR content LIKE ?3
            ORDER BY memo_data.updated DESC;",
        )
        .unwrap();

    let sql = format!("%{}%", params.content);

    let result = stmt
        .query_map(params![sql, sql, sql], |record| {
            Ok(MemoData {
                id: record.get("id")?,
                title: record.get("title")?,
                content: record.get("content")?,
                created: record.get("created")?,
                updated: record.get("updated")?,
            })
        })
        .unwrap()
        .filter_map(|record| record.ok());

    let memo_list: Vec<MemoData> = result.collect();

    let mut return_value: Vec<SearchMemoData> = Vec::new();

    for row in memo_list {
        let mut search_tag_stmt = db
            .prepare(
                "SELECT memo_tag_data.tag_id as id, memo_tag.name
        FROM memo_tag_data
            LEFT JOIN memo_tag ON memo_tag_data.tag_id = memo_tag.id
        WHERE memo_tag_data.memo_id = ?1",
            )
            .unwrap();

        let tags: Vec<SearchTagValue> = search_tag_stmt
            .query_map(params![row.id], |record| {
                Ok(SearchTagValue {
                    id: record.get("id")?,
                    name: record.get("name")?,
                })
            })
            .unwrap()
            .filter_map(|record| record.ok())
            .collect();

        let record_item = SearchMemoData {
            id: row.id,
            title: row.title,
            content: row.content,
            tags: tags,
        };
        return_value.push(record_item);
    }

    return return_value;
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
pub fn insert_memo_data(state: State<Db>, params: InsertMemoData) -> bool {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();

    let tx = db.transaction();
    let memo_data_uuid = Uuid::new_v4().to_string();
    let now = Local::now().timestamp_millis();

    tx.execute(
        "insert into memo_data(id,title,content,created,updated) values(?1,?2,?3,?4,?5)",
        params![memo_data_uuid, params.title, params.content, now, now],
    )
    .unwrap();

    for tag in params.tags {
        let memo_tag_data_uuid = Uuid::new_v4().to_string();
        tx.execute(
            "insert into memo_tag_data(id,tag_id,memo_id,created,updated) values(?1,?2,?3,?4,?5)",
            params![memo_tag_data_uuid, tag, memo_data_uuid, now, now],
        )
        .unwrap();
    }
    let success = tx.commit().is_ok();
    return success;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteMemoData {
    pub id: String,
}

/**
 * 删除记录
 */
#[tauri::command]
pub fn delete_memo_data(state: State<Db>, params: DeleteMemoData) -> bool {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();

    let tx = db.transaction();
    tx.execute("delete from memo_data where id = ?1", params![params.id])
        .unwrap();
    tx.execute(
        "delete from memo_tag_data where memo_id = ?1",
        params![params.id],
    )
    .unwrap();

    let success = tx.commit().is_ok();
    return success;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditMemoData {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
}

#[tauri::command]
pub fn edit_memo_data(state: State<Db>, params: EditMemoData) -> bool {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();

    let tx = db.transaction();
    let now = Local::now().timestamp_millis();

    tx.execute(
        "update memo_data set title = ?1, content= ?2, updated = ?3 where id = ?4",
        params![params.title, params.content, now, params.id],
    )
    .unwrap();

    for t in params.tags {
        tx.execute(
            "delete from memo_tag_data where memo_id = ?1",
            params![params.id],
        )
        .unwrap();
        let memo_tag_data_uuid = Uuid::new_v4().to_string();
        tx.execute(
            "insert into memo_tag_data(id,tag_id,memo_id,created,updated) values(?1,?2,?3,?4,?5)",
            params![memo_tag_data_uuid, t, params.id, now, now],
        )
        .unwrap();
    }
    let success = tx.commit().is_ok();
    return success;
}
