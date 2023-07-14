/*
 * `AppState` represents the application state, holding the database connection pool and an AWS S3 client.
 *
 * - `pool` [PgPool]: The PostgreSQL connection pool for database operations.
 * - `s3`   [aws_sdk_s3::Client]: The AWS S3 client for interacting with S3 services.
 */
use sqlx::postgres::PgPool;

use crate::search::SearchEngine;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub search_engine: SearchEngine,
}

impl AppState {
    pub fn new(pg_pool: PgPool, search_engine: SearchEngine) -> Self {
        Self {
            pg_pool,
            search_engine,
        }
    }
}
