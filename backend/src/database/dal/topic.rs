use sqlx::query_as;
use sqlx::{postgres::PgPool, query};

use crate::error::{Error, Result};
use crate::models::topic::{
    TopicApiResource, TopicCreatePayload, TopicOnlineResource, TopicResourceGetPayload, TopicTable,
};

impl TopicTable {
    pub async fn create(pool: PgPool, payload: &TopicCreatePayload) -> Result<String> {
        match query!(
            r#"
                INSERT INTO topics (name)
                VALUES ($1)
            "#,
            payload.name
        )
        .execute(&pool)
        .await
        {
            Ok(_) => Ok("Topic created successfully".to_string()),
            Err(e) => {
                tracing::error!("Error creating topic: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }

    pub async fn get_resources(
        pool: PgPool,
        payload: &TopicResourceGetPayload,
    ) -> Result<(Vec<TopicApiResource>, Vec<TopicOnlineResource>)> {
        let online_resources: Vec<TopicOnlineResource> = query_as!(
            TopicOnlineResource,
            r#"
                SELECT url, description
                FROM OnlineResources
                WHERE topic_id = $1
            "#,
            payload.id
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        let api_resources: Vec<TopicApiResource> = query_as!(
            TopicApiResource,
            r#"
                SELECT description, method, path
                FROM ApiResources
                WHERE topic_id = $1
            "#,
            payload.id
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        Ok((api_resources, online_resources))
    }
}
