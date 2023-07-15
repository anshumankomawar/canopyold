use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct TopicTable {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct TopicCreatePayload {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct TopicResourceGetPayload {
    pub id: i64,
}

#[derive(Serialize, Debug)]
pub struct TopicOnlineResource {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct TopicApiResource {
    pub method: String,
    pub path: String,
    pub description: Option<String>,
}
