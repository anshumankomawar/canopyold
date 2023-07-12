mod routes;
mod app_state;

use routes::create_routes;
use app_state::AppState;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt().init();

    let _app_state = AppState::new();
    let app = create_routes(_app_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
