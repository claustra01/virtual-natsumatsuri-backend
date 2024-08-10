use crate::handler::ws::MySender;
use axum::{routing::get, Router};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
type Tx = MySender;

pub fn create_router(peer_map: Arc<Mutex<HashMap<String, HashSet<Tx>>>>) -> Router {
    Router::new()
        .route("/", get(crate::handler::hello::hello_handler))
        .route("/ws", get(crate::handler::ws::ws_handler))
        .with_state(peer_map)
}
