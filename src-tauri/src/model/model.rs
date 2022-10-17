use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Record {
    id: String,
    tag: String,
    content: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Tag {
    id: String,
    content: String,
}