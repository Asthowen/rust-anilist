use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StaffImage {
    pub large: Option<String>,
    pub medium: Option<String>,
}
