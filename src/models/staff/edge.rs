use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffEdge {
    pub id: Option<i32>,
    pub role: Option<String>,
    pub favourite_order: Option<i32>,
}
