use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tantivy::query::QueryParserError;

pub type Result<T> = core::result::Result<T, Error>;

/*
 * `Error` represents the custom error type for this module, encompassing various error scenarios.
 *
 * Variants:
 * - `HashError`: Represents an error encountered during hashing operations.
 * - `WrongPassword`: Represents an error indicating that the provided password is incorrect.
 * - `QueryError`: Represents an error encountered during a database query.
 *      - `error` [sqlx::Error]: The underlying SQLx error.
 * - `AuthError`: Represents an authentication-related error.
 *      - `error` [String]: The specific error message describing the authentication error.
 * - `TokenError`: Represents an error related to token processing.
 *      - `error` [jsonwebtoken::errors::Error]: The underlying JSON Web Token (JWT) error.
 */
#[derive(Debug)]
pub enum Error {
    QueryError { error: sqlx::Error },
    QueryParseError { error: QueryParserError },
    SearchError { error: tantivy::TantivyError },
    IndexError { error: tantivy::TantivyError },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("Error: {self:?}");
        match self {
            Error::QueryError { error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("QueryError: {:?}", error),
            )
                .into_response(),
            Error::SearchError { error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("SearchError: {:?}", error),
            )
                .into_response(),
            Error::IndexError { error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IndexError: {:?}", error),
            )
                .into_response(),
            Error::QueryParseError { error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("QueryParseError: {:?}", error),
            )
                .into_response(),
        }
    }
}
