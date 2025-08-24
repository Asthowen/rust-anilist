use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTag {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub rank: Option<i32>,
    pub is_general_spoiler: Option<bool>,
    pub is_media_spoiler: Option<bool>,
    pub is_adult: Option<bool>,
    pub user_id: Option<i32>,
}
