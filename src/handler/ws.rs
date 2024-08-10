use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::{net::SocketAddr, ops::Deref};
use tokio::sync::mpsc::{self, UnboundedSender};

#[derive(Clone)]
pub struct MySender(Arc<UnboundedSender<Message>>);

impl PartialEq for MySender {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for MySender {}

impl Hash for MySender {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state)
    }
}

// Derefトレイトを実装して、UnboundedSender<Message>のメソッドをMySenderを通して使えるようにする
impl Deref for MySender {
    type Target = UnboundedSender<Message>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MySender {
    // MySender独自のメソッドを追加することもできます
    pub fn new(sender: UnboundedSender<Message>) -> Self {
        MySender(Arc::new(sender))
    }
}

type Tx = MySender;
type PeerMap = Arc<Mutex<HashMap<String, HashSet<Tx>>>>;

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct QueryParams {
    #[serde_as(as = "NoneAsEmptyString")]
    room_id: Option<String>,
}

impl QueryParams {
    const DEFAULT_ROOM_ID: &'static str = "";

    pub fn params(&self) -> String {
        self.room_id
            .clone()
            .unwrap_or(Self::DEFAULT_ROOM_ID.to_string())
    }
}

impl Default for QueryParams {
    fn default() -> Self {
        Self {
            room_id: Some(Self::DEFAULT_ROOM_ID.to_string()),
        }
    }
}

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
        let peers = peer_map
            .entry(params.room_id.clone().expect("room_id is required"))
            .or_insert_with(HashSet::new);
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
                            print!("{:?}", params.room_id.clone().expect("room_id is required"));
                            broadcast_message(
                                &peer_map,
                                &params.room_id.clone().expect("room_id is required"),
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
                        _ => {
                            println!("Unknown");
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

async fn broadcast_message(peer_map: &PeerMap, room_id: &str, message: Message) {
    let rooms = peer_map.lock().unwrap();
    if let Some(peers) = rooms.get(room_id) {
        for peer in peers {
            peer.send(message.clone())
                .expect("Failed to send message to peer");
        }
    }
}
