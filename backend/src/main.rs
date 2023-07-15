mod app_state;
mod database;
mod error;
mod models;
mod routes;
mod search;

use app_state::AppState;
use database::get_postgres_pool;
use routes::create_routes;
use search::get_search_engine;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pg_pool = get_postgres_pool().await;
    let search_engine = get_search_engine();
    let _app_state = AppState::new(pg_pool, search_engine);
    let app = create_routes(_app_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
