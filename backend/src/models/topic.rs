use crate::models::components::Component;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sqlx::types::Json;

#[derive(Serialize, Debug)]
pub struct TopicTable {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub components: Json<Vec<Component>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TopicTable {
    pub fn get_components_json(&self) -> Map<String, Value> {
        let mut components_json = Map::new();
        components_json.insert("components".to_string(), json!(self.components));
        components_json
    }
}

#[derive(Deserialize, Debug)]
pub struct TopicCreatePayload {
    pub name: String,
    pub description: String,
    pub components: Vec<Component>,
}
