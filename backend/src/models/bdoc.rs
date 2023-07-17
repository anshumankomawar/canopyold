use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct BdocTable {
    pub id: i64,
    pub name: String,
    pub directory_id: i64,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct BdocCreatePayload {
    pub name: String,
    pub bdoc_dir_id: i64,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct BdocGetPayload {
    pub id: i64,
}
