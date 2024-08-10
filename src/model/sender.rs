use axum::extract::ws::Message;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct MySender(pub Arc<UnboundedSender<Message>>);

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

impl Deref for MySender {
    type Target = UnboundedSender<Message>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type PeerMap = Arc<Mutex<HashMap<String, HashSet<MySender>>>>;

pub trait PeerMapTrait {
    async fn broadcast_message(&self, room_id: &str, message: Message);
}

impl PeerMapTrait for PeerMap {
    async fn broadcast_message(&self, room_id: &str, message: Message) {
        // ロックはここで取得
        let peers = {
            let rooms = self.lock().unwrap();
            rooms.get(room_id).cloned() // Cloneしてロックを外す
        };

        if let Some(peers) = peers {
            for peer in peers {
                peer.send(message.clone())
                    .expect("Failed to send message to peer");
            }
        }
    }
}
