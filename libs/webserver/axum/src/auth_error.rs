use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
// use axum::{http::Result};
pub enum AuthError {
    InvalidToken,
    BadCredentials,
    TokenCreation(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, Json(&"Invalid token")).into_response()
            }
            AuthError::BadCredentials => {
                (StatusCode::UNAUTHORIZED, Json(&"Bad credentials")).into_response()
            }
            AuthError::TokenCreation(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(&format!("Token creation failed: {}", e)),
            )
                .into_response(),
        }
    }
}
