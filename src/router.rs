use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(crate::handler::hello::hello_handler))
        .route("/ws", get(crate::handler::ws::ws_handler))
}
