use serde::{Deserialize, Serialize};

use crate::models::ExternalLinkType;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaExternalLink {
    pub id: Option<i32>,
    pub url: Option<String>,
    pub site: Option<String>,
    pub site_id: Option<i32>,
    pub r#type: Option<ExternalLinkType>,
    pub language: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub notes: Option<String>,
    pub is_disabled: Option<bool>,
}
