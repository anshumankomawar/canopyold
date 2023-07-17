use crate::app_state::{AppState};
use crate::models::bdoc::{BdocTable, BdocCreatePayload, BdocGetPayload };
use crate::error::{Result, Error};

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
            Ok(_) => {Ok("Business document created successfully".to_string())}
            Err(e) => {
                tracing::error!("Error creating business document: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }

    }

    pub async fn get(app_state: AppState, payload: &BdocGetPayload) -> Result<BdocTable> {
        let pool = &app_state.pg_pool;
        match query_as!(BdocTable,
            r#"
                SELECT id, name, directory_id, content, created_at, updated_at
                FROM BDocuments
                WHERE id = $1
            "#,
            payload.id
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
}
