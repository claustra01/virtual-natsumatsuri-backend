use handler::ws::MySender;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
type Tx = MySender;
type PeerMap = Arc<Mutex<HashMap<String, HashSet<Tx>>>>;

mod handler;
mod model;
mod router;

#[tokio::main]
async fn main() {
    let rooms: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    let app = router::create_router(rooms.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
