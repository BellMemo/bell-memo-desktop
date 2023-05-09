use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{Connection, FromRow, Sqlite};
use tauri::State;
use uuid::Uuid;

use crate::{model::model::MemoData, plugin::db::Db};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchValue {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
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
pub async fn select_memo_data(
    state: State<'_, Db>,
    params: SearchValue,
) -> Result<Vec<SearchMemoData>, ()> {
    let db = state.connection.lock().await;
    let mut conn = db.get_connection().await;

    let sql = format!("%{}%", params.content);

    let memo_result:Vec<MemoData> = sqlx::query_as::<Sqlite,MemoData>(
            r#"SELECT DISTINCT memo_data.id, memo_data.content, memo_data.title, memo_data.created, memo_data.updated
            FROM memo_data, memo_tag
            WHERE name LIKE ?1
                OR title LIKE ?2
                OR content LIKE ?3
            ORDER BY memo_data.updated DESC;"#,
    )
    .bind(sql.clone())
    .bind(sql.clone())
    .bind(sql.clone())
    .fetch_all(&mut conn)
    .await
    .unwrap();

    let mut return_value: Vec<SearchMemoData> = Vec::new();

    for row in memo_result {
        let search_tag = sqlx::query_as::<Sqlite, SearchTagValue>(
            r#"SELECT memo_tag_data.tag_id as id, memo_tag.name
        FROM memo_tag_data
            LEFT JOIN memo_tag ON memo_tag_data.tag_id = memo_tag.id
        WHERE memo_tag_data.memo_id = ?1"#,
        )
        .bind(row.id.clone())
        .fetch_all(&mut conn)
        .await
        .unwrap();

        let record_item = SearchMemoData {
            id: row.id,
            title: row.title,
            content: row.content,
            tags: search_tag,
        };
        return_value.push(record_item);
    }

    Ok(return_value)
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
pub async fn insert_memo_data(state: State<'_, Db>, params: InsertMemoData) -> Result<bool, ()> {
    let db = state.connection.lock().await;
    let mut conn = db.get_connection().await;
    let mut tx = conn.begin().await.unwrap();

    let memo_data_uuid = Uuid::new_v4().to_string();
    let now = Local::now().timestamp_millis();

    sqlx::query("insert into memo_data(id,title,content,created,updated) values(?1,?2,?3,?4,?5)")
        .bind(memo_data_uuid.clone())
        .bind(params.title)
        .bind(params.content)
        .bind(now)
        .bind(now)
        .execute(&mut tx)
        .await
        .unwrap();

    for tag in params.tags {
        let memo_tag_data_uuid = Uuid::new_v4().to_string();

        sqlx::query(
            "insert into memo_tag_data(id,tag_id,memo_id,created,updated) values(?1,?2,?3,?4,?5)",
        )
        .bind(memo_tag_data_uuid)
        .bind(tag)
        .bind(memo_data_uuid.clone())
        .bind(now)
        .bind(now)
        .execute(&mut tx)
        .await
        .unwrap();
    }
    let success = tx.commit().await.is_ok();
    Ok(success)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteMemoData {
    pub id: String,
}

/**
 * 删除记录
 */
#[tauri::command]
pub async fn delete_memo_data(state: State<'_, Db>, params: DeleteMemoData) -> Result<bool, ()> {
    let db = state.connection.lock().await;

    let mut conn = db.get_connection().await;
    let mut tx = conn.begin().await.unwrap();

    sqlx::query("delete from memo_data where id = ?1")
        .bind(params.id.clone())
        .execute(&mut tx)
        .await
        .unwrap();
    sqlx::query("delete from memo_tag_data where memo_id = ?1")
        .bind(params.id.clone())
        .execute(&mut tx)
        .await
        .unwrap();

    let success = tx.commit().await.is_ok();
    Ok(success)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditMemoData {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
}

#[tauri::command]
pub async fn edit_memo_data(state: State<'_, Db>, params: EditMemoData) -> Result<bool, ()> {
    let db = state.connection.lock().await;

    let mut conn = db.get_connection().await;
    let mut tx = conn.begin().await.unwrap();

    let now = Local::now().timestamp_millis();

    sqlx::query("update memo_data set title = ?1, content= ?2, updated = ?3 where id = ?4")
        .bind(params.title)
        .bind(params.content)
        .bind(now)
        .bind(params.id.clone())
        .execute(&mut tx)
        .await
        .unwrap();

    for t in params.tags {
        sqlx::query("delete from memo_tag_data where memo_id = ?1")
            .bind(params.id.clone())
            .execute(&mut tx)
            .await
            .unwrap();
        let memo_tag_data_uuid = Uuid::new_v4().to_string();
        sqlx::query(
            "insert into memo_tag_data(id,tag_id,memo_id,created,updated) values(?1,?2,?3,?4,?5)",
        )
        .bind(memo_tag_data_uuid)
        .bind(t)
        .bind(params.id.clone())
        .bind(now)
        .bind(now)
        .execute(&mut tx)
        .await
        .unwrap();
    }
    let success = tx.commit().await.is_ok();
    Ok(success)
}
