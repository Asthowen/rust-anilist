use serde::{Deserialize, Serialize};

use crate::models::Staff;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffRoleType {
    pub voice_actor: Option<Staff>,
    pub role_notes: Option<String>,
    pub dub_group: Option<String>,
}
