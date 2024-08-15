use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MessageType {
    #[serde(rename = "pointer")]
    Pointer,
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "hit")]
    Hit,
}

#[derive(Debug, Serialize)]
pub struct Target {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize)]
pub struct PointerSchema {
    pub id: String,
    pub message_type: MessageType,
    pub target: Target,
}

#[derive(Debug, Serialize)]
pub struct ActionSchema {
    pub id: String,
    pub message_type: MessageType,
    pub target: Target,
    pub vector: Vector,
}

#[derive(Debug, Serialize)]
pub struct HitCountSchema {
    pub id: String,
    pub message_type: MessageType,
}
