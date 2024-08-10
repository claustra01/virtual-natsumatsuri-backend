use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::{self};

use crate::model::{
    query::QueryParams,
    sender::{MySender, PeerMap, PeerMapTrait},
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(peer_map): State<PeerMap>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    println!("{:?} connected with params: {:?}", addr, params);
    ws.on_upgrade(move |socket| handle_socket(socket, addr, peer_map, params))
}

async fn handle_socket(
    socket: WebSocket,
    addr: SocketAddr,
    peer_map: PeerMap,
    params: QueryParams,
) {
    println!("Handling socket for {:?} with params: {:?}", addr, params);

    let (tx, mut rx) = mpsc::unbounded_channel();
    {
        let mut peer_map = peer_map.lock().unwrap();
        let peers = peer_map.entry(params.params()).or_default();
        peers.insert(MySender(Arc::new(tx.clone())));
    }

    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                println!("{:?} disconnected.", addr);
                return;
            }
        }
    });

    // メッセージを受信して処理する
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            let msg_txt = msg.to_text();
            if let Ok(msg_txt) = msg_txt {
                let result = serde_json::from_str::<crate::model::schema::Schema>(msg_txt);
                if let Ok(data) = result {
                    match data.event_type {
                        crate::model::schema::EventType::Shooter => {
                            println!("Shooter");
                            print!("{:?}", params.params());
                            // ここでusecaseを呼び出す
                            peer_map
                                .broadcast_message(
                                    &params.params(),
                                    Message::Text("hoge".to_string()),
                                )
                                .await;
                        }
                        crate::model::schema::EventType::RingToss => {
                            println!("Wanage");
                        }
                        crate::model::schema::EventType::FireFlower => {
                            println!("Hanabi");
                        }
                    }
                } else {
                    println!("{:?} sent invalid JSON.", addr);
                    return;
                }
            } else {
                println!("{:?} sent invalid UTF-8.", addr);
                return;
            }
        } else {
            println!("{:?} disconnected.", addr);
            return;
        }
    }

    // 送信タスクを待機
    send_task.await.unwrap();
}
