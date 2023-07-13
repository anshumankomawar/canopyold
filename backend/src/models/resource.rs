use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Debug)]
pub struct ApiResourceTable {
    pub id: i64,
    pub resource_id: i64,
    pub method: String,
    pub path: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Debug)]
pub struct OnlineResourceTable {
    pub id: i64,
    pub resource_id: i64,
    pub url: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct ApiResourceCreatePayload {
    pub topic_id: i64,
    pub description: String,
    pub method: String,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct OnlineResourceCreatePayload {
    pub topic_id: i64,
    pub description: String,
    pub url: String,
}
