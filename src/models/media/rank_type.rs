use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaRankType {
    #[serde(rename = "RATED")]
    Rated,
    #[serde(rename = "POPULAR")]
    Popular,
}
