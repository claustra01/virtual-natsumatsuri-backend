use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn hello_handler() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}
