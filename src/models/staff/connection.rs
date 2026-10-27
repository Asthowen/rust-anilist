use serde::{Deserialize, Serialize};

use crate::models::{PageInfo, Staff, StaffEdge};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffConnection {
    pub edges: Option<Vec<StaffEdge>>,
    pub nodes: Option<Vec<Staff>>,
    pub page_info: Option<PageInfo>,
}
