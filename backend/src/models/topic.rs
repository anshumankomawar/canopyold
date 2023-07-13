use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

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
