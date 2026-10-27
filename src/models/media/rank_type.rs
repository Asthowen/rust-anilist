use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum MediaRankType {
    #[serde(rename = "RATED")]
    Rated,
    #[serde(rename = "POPULAR")]
    Popular,
}
