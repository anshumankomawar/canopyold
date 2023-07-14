use crate::{app_state::AppState, error::Result, models::search::SearchTopicPayload};
use axum::{extract::State, response::Json};
use serde_json::{json, Value};

#[axum_macros::debug_handler]
pub async fn search_topic(
    State(state): State<AppState>,
    payload: Json<SearchTopicPayload>,
) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "search/topic");
    let search_engine = &state.search_engine;
    let query = &payload.query;
    match search_engine.search(query) {
        Ok(docs) => {
            let results = search_engine.convert_to_json(docs);
            Ok(Json(json!({ "results": results })))
        }
        Err(e) => Err(e),
    }
}
