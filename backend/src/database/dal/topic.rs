use crate::app_state::AppState;
use crate::error::{Error, Result};
use crate::models::topic::{TopicCreatePayload, TopicTable};
use sqlx::query;

use serde_json::json;

impl TopicTable {
    pub async fn create(app_state: AppState, payload: &TopicCreatePayload) -> Result<String> {
        let pool = &app_state.pg_pool;
        match query!(
            r#"
                INSERT INTO topics (name, description, components)
                VALUES ($1, $2, $3)
                RETURNING id, name, description, components, created_at, updated_at
            "#,
            payload.name,
            payload.description,
            json!(payload.components)
        )
        .fetch_one(pool)
        .await
        {
            Ok(result) => {
                // add document to search engine
                let search_engine = &app_state.search_engine;
                let serde_json_value =
                    serde_json::from_str(&json!(result.components).to_string()).unwrap();
                let payload = TopicTable {
                    id: result.id,
                    name: result.name,
                    description: result.description,
                    components: serde_json_value,
                    created_at: result.created_at,
                    updated_at: result.updated_at,
                };
                println!("payload: {:?}", payload);
                match search_engine.add_document(payload) {
                    Ok(_) => {
                        tracing::info!("Topic created successfully");
                        Ok("Topic created successfully".to_string())
                    }
                    Err(e) => {
                        tracing::error!("Error adding document to search engine: {:?}", e);
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Error creating topic: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }
}
