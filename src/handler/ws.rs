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
    schema,
    sender::{MySender, PeerMap, PeerMapTrait},
};
use crate::usecase::shooting;

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
    let (tx, mut rx) = mpsc::unbounded_channel();
    {
        let mut peer_map = peer_map.lock().unwrap();
        let peers = peer_map.entry(params.params()).or_default();
        peers.insert(MySender(Arc::new(tx.clone())));
    }

    let (mut sender, mut receiver) = socket.split();

    let params_clone = params.clone();
    let peer_map_clone = peer_map.clone();
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                println!("{:?} disconnected.", addr);
                let mut peer_map = peer_map_clone.lock().unwrap();
                let room_id = params_clone.params().clone();
                if let Some(peers) = peer_map.get_mut(&room_id) {
                    peers.retain(|peer| !Arc::ptr_eq(&peer.0, &Arc::new(tx.clone())));
                    if peers.is_empty() {
                        peer_map.remove(&room_id);
                    }
                }

                return;
            }
        }
    });

    // メッセージを受信して処理する
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            let msg_txt = msg.to_text();
            if let Ok(msg_txt) = msg_txt {
                let result = serde_json::from_str::<schema::Schema>(msg_txt);
                if let Ok(data) = result {
                    match data.event_type {
                        schema::EventType::Shooter => {
                            println!("Shooter event received.");
                            println!("{:?}", &params.params().clone());
                            // usecase
                            match data.message_type {
                                schema::MessageType::Status => {
                                    let pointer_schema = shooting::build_pointer_schema(data);
                                    peer_map
                                        .broadcast_message(
                                            &params.params(),
                                            Message::Text(
                                                serde_json::to_string(&pointer_schema).unwrap(),
                                            ),
                                        )
                                        .await;
                                }
                                schema::MessageType::Action => {
                                    let action_schema = shooting::build_action_schema(data);
                                    peer_map
                                        .broadcast_message(
                                            &params.params(),
                                            Message::Text(
                                                serde_json::to_string(&action_schema).unwrap(),
                                            ),
                                        )
                                        .await;
                                }
                            }
                        }
                        schema::EventType::RingToss => {
                            println!("RingToss event received.");
                        }
                        schema::EventType::FireFlower => {
                            println!("FireFlower event received.");
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
