use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(crate::handler::hello::hello_handler))
        .route("/shooter", get(crate::handler::shooter::shooter_handler))
}
