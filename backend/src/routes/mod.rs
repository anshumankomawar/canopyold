pub mod bdoc;
pub mod search;
pub mod topic;

use crate::app_state;
use crate::routes::bdoc::{create_bdoc, get_bdoc, save_bdoc};
use crate::routes::search::search_topic;
use crate::routes::topic::create_topic;
use axum::http::Method;
use axum::middleware::map_response;
use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes(state: app_state::AppState) -> Router<(), Body> {
    let routes = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/topic/create", post(create_topic))
        .route("/search/topic", post(search_topic))
        .route("/bdoc/create", post(create_bdoc))
        .route("/bdoc/get/:id", get(get_bdoc))
        .route("/bdoc/save/:id", post(save_bdoc));

    Router::new()
        .merge(routes)
        .layer(map_response(main_response_mapper))
        .layer(cors())
        .with_state(state)
        .fallback(handler_404)
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
}

async fn main_response_mapper(res: Response) -> Response {
    tracing::debug!("{:<12} [main_response_mapper]\n", "RES_MAPPER");
    res
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "No route found")
}
