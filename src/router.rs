use axum::{routing::get, Router};

use crate::model::sender::PeerMap;

pub fn create_router(peer_map: PeerMap) -> Router {
    Router::new()
        .route("/", get(crate::handler::hello::hello_handler))
        .route("/ws", get(crate::handler::ws::ws_handler))
        .with_state(peer_map)
}
