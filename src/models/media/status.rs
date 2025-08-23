use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaStatus {
    #[serde(alias = "FINISHED")]
    Finished,
    #[serde(alias = "RELEASING")]
    Releasing,
    #[serde(alias = "NOT_YET_RELEASED")]
    NotYetReleased,
    #[serde(alias = "CANCELLED")]
    Cancelled,
    #[serde(alias = "HIATUS")]
    Hiatus,
}
