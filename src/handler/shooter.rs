use axum::{http::StatusCode, response::IntoResponse};

pub async fn shooter_handler() -> impl IntoResponse {
    (StatusCode::OK, "Shooter Handler")
}
