use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString};

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
