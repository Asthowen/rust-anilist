use serde::{Deserialize, Serialize};

use crate::models::{Media, MediaEdge, PageInfo};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaConnection {
    pub edges: Option<Vec<MediaEdge>>,
    pub nodes: Option<Vec<Media>>,
    pub page_info: Option<PageInfo>,
}
