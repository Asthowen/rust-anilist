use serde::{Deserialize, Serialize};

use crate::models::Staff;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffEdge {
    pub node: Option<Staff>,
    pub id: Option<i32>,
    pub role: Option<String>,
    pub favourite_order: Option<i32>,
}
