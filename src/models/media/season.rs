use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
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
