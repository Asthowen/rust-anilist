use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringSchedule {
    id: Option<i32>,
    airing_at: Option<i32>,
    time_until_airing: Option<i32>,
    episode: Option<i32>,
    media_id: Option<i32>,
}
