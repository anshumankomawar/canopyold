use axum::{
    response::Json,
    extract::State,
};
use serde_json::{Value, json};
use crate::{models::resource::{ApiResourceCreatePayload, OnlineResourceCreatePayload, ApiResourceTable, OnlineResourceTable}, error::Result, app_state::AppState};

#[axum_macros::debug_handler]
pub async fn create_api_resource(State(state): State<AppState>, payload: Json<ApiResourceCreatePayload>) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "resources/api/create");
    match ApiResourceTable::create(state.pg_pool, &payload).await {
        Ok(result) => {
            let response = Json(json!({"result": result}));
            Ok(response)
        },
        Err(e) => Err(e)
    }
}

#[axum_macros::debug_handler]
pub async fn create_online_resource(State(state): State<AppState>, payload: Json<OnlineResourceCreatePayload>) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "resources/online/create");
    match OnlineResourceTable::create(state.pg_pool, &payload).await {
        Ok(result) => {
            let response = Json(json!({"result": result}));
            Ok(response)
        },
        Err(e) => Err(e)
    }
}
