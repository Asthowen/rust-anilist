use crate::models::Staff;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffRoleType {
    pub voice_actor: Option<Staff>,
    pub role_notes: Option<String>,
    pub dub_group: Option<String>,
}
