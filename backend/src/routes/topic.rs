use axum::{
    response::Json,
    extract::State,
};
use serde_json::{Value, json};
use crate::{models::topic::{TopicCreatePayload, TopicTable}, error::Result, app_state::AppState};

#[axum_macros::debug_handler]
pub async fn create(State(state): State<AppState>, payload: Json<TopicCreatePayload>) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "users/create");
    match TopicTable::create(state.pg_pool, &payload).await {
        Ok(result) => {
            let response = Json(json!({"result": result}));
            Ok(response)
        },
        Err(e) => Err(e)
    }
}
