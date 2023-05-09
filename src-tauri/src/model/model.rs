use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MemoData {
    pub id: String,
    pub title: String,
    pub content: String,
    #[sqlx(try_from = "i64")]
    pub created: u64,
    #[sqlx(try_from = "i64")]
    pub updated: u64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MemoTag {
    pub id: String,
    pub name: String,
    #[sqlx(try_from = "i64")]
    pub created: u64,
    #[sqlx(try_from = "i64")]
    pub updated: u64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MemoTagData {
    pub id: String,
    pub tag_id: String,
    pub memo_id: String,
    #[sqlx(try_from = "i64")]
    pub created: u64,
    #[sqlx(try_from = "i64")]
    pub updated: u64,
}
