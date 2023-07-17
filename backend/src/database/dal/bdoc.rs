use crate::app_state::AppState;
use crate::error::{Error, Result};
use crate::models::bdoc::{BdocCreatePayload, BdocSavePayload, BdocTable};

use sqlx::{query, query_as};

impl BdocTable {
    pub async fn create(app_state: AppState, payload: &BdocCreatePayload) -> Result<String> {
        let pool = &app_state.pg_pool;
        match query!(
            r#"
                INSERT INTO BDocuments (name, directory_id, content) 
                VALUES ($1, 1, $2)
            "#,
            payload.name,
            //payload.bdoc_dir_id,
            payload.content
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok("Business document created successfully".to_string()),
            Err(e) => {
                tracing::error!("Error creating business document: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }

    pub async fn get(app_state: AppState, id: i64) -> Result<BdocTable> {
        let pool = &app_state.pg_pool;
        match query_as!(
            BdocTable,
            r#"
                SELECT id, name, directory_id, content, created_at, updated_at
                FROM BDocuments
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        {
            Ok(result) => {
                tracing::info!("Business document retrieved successfully");
                Ok(result)
            }
            Err(e) => {
                tracing::error!("Error retrieving business document: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }

    pub async fn save(app_state: AppState, id: i64, payload: &BdocSavePayload) -> Result<String> {
        let pool = &app_state.pg_pool;
        match query!(
            r#"
                UPDATE BDocuments
                SET content = $1, updated_at = NOW()
                WHERE id = $2
            "#,
            payload.content,
            id
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok("Business document saved successfully".to_string()),
            Err(e) => {
                tracing::error!("Error saving business document: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }
}
