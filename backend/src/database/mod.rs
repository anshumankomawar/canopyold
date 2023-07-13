pub mod dal;

use dotenvy::dotenv;
use sqlx::postgres::{PgPoolOptions, PgPool};

pub async fn get_postgres_pool() -> PgPool {
    tracing::debug!("Connecting to postgres");
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    tracing::debug!("Connected");
    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Unable to connect to database")
}
