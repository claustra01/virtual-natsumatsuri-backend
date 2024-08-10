use std::net::SocketAddr;

use axum::{
    extract::{connect_info::ConnectInfo, ws::{Message, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
};

pub async fn shooter_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("{:?} connected.", addr);
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, addr: SocketAddr) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {

            let msg_txt = msg.to_text();
            if let Ok(msg_txt) = msg_txt {
                
                let result = serde_json::from_str::<crate::model::shooter::ShooterData>(msg_txt);
                if let Ok(data) = result {
                    println!("{:?}", data.id);
                    if socket.send(Message::Text(data.id)).await.is_err() {
                        // client disconnected
                        println!("{:?} disconnected.", addr);
                        return;
                    }

                } else {
                    println!("{:?} sent invalid JSON.", addr);
                    return;
                }

            } else {
                // client sent invalid UTF-8
                println!("{:?} sent invalid UTF-8.", addr);
                return;
            }

        } else {
            // client disconnected
            println!("{:?} disconnected.", addr);
            return;
        };
    }
}
