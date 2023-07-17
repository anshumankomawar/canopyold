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
pub struct BdocSavePayload {
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct BdocDirTable {
    pub id: i64,
    pub name: String,
    pub parent_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Debug, sqlx::Type)]
#[repr(i32)]
pub enum DirentType {
    File = 1,
    Directory = 2,
}

#[derive(Serialize, Debug)]
pub struct BdocDirent {
    pub id: i64,
    pub name: String,
    pub dirent_type: Option<DirentType>,
}

#[derive(Deserialize, Debug)]
pub struct BdocDirCreatePayload {
    pub name: String,
    pub parent_id: i64,
}
