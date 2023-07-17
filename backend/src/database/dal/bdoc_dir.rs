use crate::app_state::AppState;
use crate::models::bdoc::{BdocDirCreatePayload, BdocDirTable, BdocDirent, DirentType};
use crate::error::{Error, Result};
use sqlx::{query, query_as};

impl BdocDirTable {
    pub async fn create(state: AppState, payload: &BdocDirCreatePayload) -> Result<String> {
        let pool = &state.pg_pool;
        match query!(
            r#"
                INSERT INTO BDocumentDirectories (name, parent_id) 
                VALUES ($1, $2)
            "#,
            payload.name,
            payload.parent_id
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok("Business document directory created successfully".to_string()),
            Err(e) => {
                tracing::error!("Error creating business document directory: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }

    pub async fn get_contents(state: AppState, id: i64) -> Result<Vec<BdocDirent>> {
        let pool = &state.pg_pool;
        let mut contents: Vec<BdocDirent> = Vec::new();
        match query_as!(
            BdocDirent,
            r#"
                SELECT id, name, 2 as "dirent_type: DirentType"
                FROM BDocumentDirectories
                WHERE parent_id = $1 AND id != $1
            "#,
            id
        )
        .fetch_all(pool)
        .await {
            Ok(dir_result) => {
                contents.extend(dir_result);
                match query_as!(
                    BdocDirent,
                    r#"
                        SELECT id, name, 1 as "dirent_type: DirentType"
                        FROM BDocuments
                        WHERE directory_id = $1
                    "#,
                    id
                )
                .fetch_all(pool)
                .await {
                    Ok(file_result) => {
                        contents.extend(file_result);
                        Ok(contents)
                    }
                    Err(e) => {
                        tracing::error!("Error retrieving business document directory contents: {:?}", e);
                        Err(Error::QueryError { error: e })
                    }
                }
            },
            Err(e) => {
                tracing::error!("Error retrieving business document directory contents: {:?}", e);
                Err(Error::QueryError { error: e })
            }
        }
    }
}
