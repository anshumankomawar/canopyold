use sqlx::{query, postgres::PgPool};

use crate::error::{Result, Error};
use crate::models::topic::{TopicTable, TopicCreatePayload};

impl TopicTable {
    pub async fn create(pool: PgPool, payload: &TopicCreatePayload) -> Result<String> {
        match query!(
            r#"
                INSERT INTO topics (name)
                VALUES ($1)
            "#,
            payload.name
        ).execute(&pool).await {
            Ok(_) => Ok("Topic created successfully".to_string()),
            Err(e) => {
                tracing::error!("Error creating topic: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }
}
