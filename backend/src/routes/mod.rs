pub mod topic;

use axum::{
    body::Body, 
    Router, 
    routing::{get, post},
    http::StatusCode,
    response::{Response, IntoResponse},
    middleware::map_response
};
use crate::app_state;

pub fn create_routes(state: app_state::AppState) -> Router<(), Body> {
    let unprotected_routes = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/topic/create", post(topic::create));

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
