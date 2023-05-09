use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{Connection, Sqlite};
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
pub async fn search_memo_tag(
    state: State<'_, Db>,
    params: SearchTagValue,
) -> Result<Vec<MemoTag>, ()> {
    let db = state.connection.lock().await;
    let mut conn = db.get_connection().await;

    let result = sqlx::query_as::<Sqlite, MemoTag>("select * from memo_tag where name like ?1")
        .bind(format!("%{}%", params.content))
        .fetch_all(&mut conn)
        .await
        .unwrap();

    Ok(result)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertMemoTag {
    content: String,
}
/**
 * 写标签
 */
#[tauri::command]
pub async fn insert_memo_tag(state: State<'_, Db>, params: InsertMemoTag) -> Result<MemoTag, ()> {
    let db = state.connection.lock().await;
    let mut conn = db.get_connection().await;
    let mut tx = conn.begin().await.unwrap();

    let _is_tag_exist = sqlx::query("select id from memo_tag where name=?")
        .bind(params.content.clone())
        .fetch_one(&mut tx)
        .await
        .unwrap();

    let uuid = Uuid::new_v4().to_string();
    let now = Local::now().timestamp_millis();

    let _result = sqlx::query("insert into memo_tag(id,name,created,updated) values(?1,?2,?3,?4)")
        .bind(uuid.clone())
        .bind(params.content.clone())
        .bind(now)
        .bind(now)
        .execute(&mut tx)
        .await
        .unwrap();

    let tag = MemoTag {
        id: uuid,
        name: params.content,
        created: now as u64,
        updated: now as u64,
    };

    Ok(tag)
}
