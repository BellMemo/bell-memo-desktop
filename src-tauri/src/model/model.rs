use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct MemoData {
    id: String,
    title: String,
    content: String,
    created: u64,
    updated: u64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct MemoTag {
    id: String,
    name: String,
    created: u64,
    updated: u64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct MemoTagData {
    id: String,
    tag_id: String,
    memo_id:String,
    created: u64,
    updated: u64,
}