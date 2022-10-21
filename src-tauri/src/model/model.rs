use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct MemoData {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created: u64,
    pub updated: u64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct MemoTag {
    pub id: String,
    pub name: String,
    pub created: u64,
    pub updated: u64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct MemoTagData {
    pub id: String,
    pub tag_id: String,
    pub memo_id:String,
    pub created: u64,
    pub updated: u64,
}