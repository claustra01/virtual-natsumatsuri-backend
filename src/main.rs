use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use model::sender::PeerMap;

mod handler;
mod model;
mod router;
mod usecase;

#[tokio::main]
async fn main() {
    let rooms: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    let app = router::create_router(rooms.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
