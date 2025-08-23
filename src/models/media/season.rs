use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaSeason {
    #[serde(alias = "WINTER")]
    Winter,
    #[serde(alias = "SPRING")]
    Spring,
    #[serde(alias = "SUMMER")]
    Summer,
    #[serde(alias = "FALL")]
    Fall,
}
