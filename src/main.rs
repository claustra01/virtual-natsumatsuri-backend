use std::net::SocketAddr;

mod handler;
mod model;
mod router;

#[tokio::main]
async fn main() {
    let app = router::create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
