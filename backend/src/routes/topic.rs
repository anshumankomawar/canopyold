use crate::{
    app_state::AppState,
    error::Result,
    models::topic::{TopicCreatePayload, TopicTable},
};
use axum::{extract::State, response::Json};
use serde_json::{json, Value};

#[axum_macros::debug_handler]
pub async fn create_topic(
    State(state): State<AppState>,
    payload: Json<TopicCreatePayload>,
) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "topics/create");
    match TopicTable::create(state, &payload).await {
        Ok(result) => {
            let response = Json(json!({ "result": result }));
            Ok(response)
        }
        Err(e) => Err(e),
    }
}
