pub mod resource;
pub mod search;
pub mod topic;

use crate::app_state;
use axum::middleware::map_response;
use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};

pub fn create_routes(state: app_state::AppState) -> Router<(), Body> {
    let unprotected_routes = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/topic/create", post(topic::create))
        .route("/topic/getresources", get(topic::get_resources))
        .route("/resource/api/create", post(resource::create_api_resource))
        .route(
            "/resource/online/create",
            post(resource::create_online_resource),
        )
        .route("/search/topic", post(search::search_topic));

    Router::new()
        .merge(unprotected_routes)
        .layer(map_response(main_response_mapper))
        .with_state(state)
        .fallback(handler_404)
}

async fn main_response_mapper(res: Response) -> Response {
    tracing::debug!("{:<12} [main_response_mapper]\n", "RES_MAPPER");
    res
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "No route found")
}
