use crate::models::Staff;
use crate::models::staff::edge::StaffEdge;
use crate::models::util::page_info::PageInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffConnection {
    pub edges: Option<Vec<StaffEdge>>,
    pub nodes: Option<Vec<Staff>>,
    pub page_info: Option<PageInfo>,
}
