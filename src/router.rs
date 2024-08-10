use axum::{routing::get, Router};
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet};
use crate::handler::ws::MySender;
type Tx = MySender;

pub fn create_router(peer_map:Arc<Mutex<HashMap<String, HashSet<Tx>>>>) -> Router {
    Router::new()
        .route("/", get(crate::handler::hello::hello_handler))
        .route("/ws", get(crate::handler::ws::ws_handler))
        .with_state(peer_map)
}
