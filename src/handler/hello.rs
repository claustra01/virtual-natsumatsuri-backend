use axum::{http::StatusCode, response::IntoResponse};

pub async fn hello_handler() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}
