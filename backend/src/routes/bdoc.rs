use crate::{
    app_state::AppState,
    error::Result,
    models::bdoc::{BdocCreatePayload, BdocSavePayload, BdocTable},
};
use axum::{
    extract::{Path, State},
    response::Json,
};
use serde_json::{json, Value};

#[axum_macros::debug_handler]
pub async fn create_bdoc(
    State(state): State<AppState>,
    payload: Json<BdocCreatePayload>,
) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "bdoc/add");
    match BdocTable::create(state, &payload).await {
        Ok(result) => {
            let response = Json(json!({ "result": result }));
            Ok(response)
        }
        Err(e) => Err(e),
    }
}

#[axum_macros::debug_handler]
pub async fn get_bdoc(Path(id): Path<i64>, State(state): State<AppState>) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "bdoc/get");
    match BdocTable::get(state, id).await {
        Ok(result) => {
            let response = Json(json!({ "result": result }));
            Ok(response)
        }
        Err(e) => Err(e),
    }
}

#[axum_macros::debug_handler]
pub async fn save_bdoc(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    payload: Json<BdocSavePayload>,
) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "bdoc/save");
    match BdocTable::save(state, id, &payload).await {
        Ok(result) => {
            let response = Json(json!({ "result": result }));
            Ok(response)
        }
        Err(e) => Err(e),
    }
}
