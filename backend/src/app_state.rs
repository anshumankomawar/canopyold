/*
 * `AppState` represents the application state, holding the database connection pool and an AWS S3 client.
 *
 * - `pool` [PgPool]: The PostgreSQL connection pool for database operations.
 * - `s3`   [aws_sdk_s3::Client]: The AWS S3 client for interacting with S3 services.
 */
#[derive(Debug, Clone)]
pub struct AppState {
}

impl AppState {
    pub fn new() -> Self {
        Self { }
    }
}
