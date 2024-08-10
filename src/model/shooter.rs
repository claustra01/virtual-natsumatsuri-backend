use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum MessageType {
    #[serde(rename = "pointer")]
    Pointer,
    #[serde(rename = "action")]
    Action,
}

#[derive(Debug, Deserialize)]
pub struct Angle {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
pub struct Acceleration {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize)]
pub struct Distance {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize)]
pub struct ShooterData {
    pub id: String,
    pub message_type: MessageType,
    pub angle: Angle,
    pub acceleration: Acceleration,
    pub distance: Distance,
    pub interval: f64,
}
