use sqlx::{postgres::PgPool, query};

use crate::error::{Error, Result};
use crate::models::resource::{
    ApiResourceCreatePayload, ApiResourceTable, OnlineResourceCreatePayload, OnlineResourceTable,
};

impl ApiResourceTable {
    pub async fn create(pool: PgPool, payload: &ApiResourceCreatePayload) -> Result<String> {
        match query!(
            r#"
                INSERT INTO ApiResources (topic_id, method, path, description)
                VALUES ($1, $2, $3, $4)
            "#,
            payload.topic_id,
            payload.method,
            payload.path,
            payload.description
        )
        .execute(&pool)
        .await
        {
            Ok(_) => Ok("Api resource created successfully".to_string()),
            Err(e) => {
                tracing::error!("Error creating api resource: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }
}

impl OnlineResourceTable {
    pub async fn create(pool: PgPool, payload: &OnlineResourceCreatePayload) -> Result<String> {
        match query!(
            r#"
                INSERT INTO OnlineResources (topic_id, url, description)
                VALUES ($1, $2, $3)
            "#,
            payload.topic_id,
            payload.url,
            payload.description
        )
        .execute(&pool)
        .await
        {
            Ok(_) => Ok("Online resource created successfully".to_string()),
            Err(e) => {
                tracing::error!("Error creating online resource: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }
}
