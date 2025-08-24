use crate::models::{FuzzyDate, Media, MediaListStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaList {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub media_id: Option<i32>,
    pub status: Option<MediaListStatus>,
    pub score: Option<f64>,
    pub progress: Option<i32>,
    pub progress_volumes: Option<i32>,
    pub repeat: Option<i32>,
    pub priority: Option<i32>,
    pub private: Option<bool>,
    pub notes: Option<String>,
    pub hidden_from_status_lists: Option<bool>,
    pub custom_lists: Option<Value>,
    pub advanced_scores: Option<Value>,
    pub started_at: Option<FuzzyDate>,
    pub completed_at: Option<FuzzyDate>,
    pub updated_at: Option<i32>,
    pub created_at: Option<i32>,
    pub media: Option<Media>,
    // pub user: Option<User>
}
