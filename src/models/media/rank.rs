use crate::models::{MediaFormat, MediaRankType, MediaSeason};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaRank {
    pub id: Option<i32>,
    pub rank: Option<i32>,
    pub r#type: Option<MediaRankType>,
    pub format: Option<MediaFormat>,
    pub year: Option<i32>,
    pub season: Option<MediaSeason>,
    pub all_time: Option<bool>,
    pub context: Option<String>,
}
