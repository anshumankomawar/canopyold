pub mod search;
pub mod topic;
pub mod bdoc;

use crate::app_state;
use crate::routes::search::search_topic;
use crate::routes::topic::create_topic;
use crate::routes::bdoc::{create_bdoc, get_bdoc};
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
        .route("/topic/create", post(create_topic))
        .route("/search/topic", post(search_topic))
        .route("/bdoc/create", post(create_bdoc))
        .route("/bdoc/get", post(get_bdoc));

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
