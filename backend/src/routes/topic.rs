use crate::{
    app_state::AppState,
    error::Result,
    models::topic::{TopicCreatePayload, TopicResourceGetPayload, TopicTable},
};
use axum::{extract::State, response::Json};
use serde_json::{json, Value};

#[axum_macros::debug_handler]
pub async fn create(
    State(state): State<AppState>,
    payload: Json<TopicCreatePayload>,
) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "users/create");
    match TopicTable::create(state.pg_pool, &payload).await {
        Ok(result) => {
            let response = Json(json!({ "result": result }));
            Ok(response)
        }
        Err(e) => Err(e),
    }
}

#[axum_macros::debug_handler]
pub async fn get_resources(
    State(state): State<AppState>,
    payload: Json<TopicResourceGetPayload>,
) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "users/get_resources");
    match TopicTable::get_resources(state.pg_pool, &payload).await {
        Ok((api, online)) => {
            let response = Json(json!({ "api": api, "online": online }));
            Ok(response)
        }
        Err(e) => Err(e),
    }
}
