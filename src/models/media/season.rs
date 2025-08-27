use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaSeason {
    #[serde(rename = "WINTER")]
    Winter,
    #[serde(rename = "SPRING")]
    Spring,
    #[serde(rename = "SUMMER")]
    Summer,
    #[serde(rename = "FALL")]
    Fall,
}
