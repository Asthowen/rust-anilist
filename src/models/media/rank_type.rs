use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaRankType {
    #[serde(alias = "RATED")]
    Rated,
    #[serde(alias = "POPULAR")]
    Popular,
}
