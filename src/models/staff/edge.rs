use crate::models::Staff;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffEdge {
    pub node: Option<Staff>,
    pub id: Option<i32>,
    pub role: Option<String>,
    pub favourite_order: Option<i32>,
}
