use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct angle {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
struct acceleration {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize)]
struct distance {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize)]
struct shooter_data {
    pub id: String,
    pub angle: angle,
    pub acceleration: acceleration,
    pub distance: distance,
    pub interval: f64,
}
