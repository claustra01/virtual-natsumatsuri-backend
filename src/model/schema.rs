use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum MessageType {
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "hit")]
    Hit,
}

#[derive(Clone, Debug, Deserialize)]
pub enum EventType {
    #[serde(rename = "shooter")]
    Shooter,
    #[serde(rename = "ring_toss")]
    RingToss,
    #[serde(rename = "fire_flower")]
    FireFlower,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Angle {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Acceleration {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Distance {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Schema {
    pub id: String,
    pub event_type: EventType,
    pub message_type: MessageType,
    pub angle: Angle,
    pub acceleration: Acceleration,
    pub distance: Distance,
    pub interval: f64,
}
