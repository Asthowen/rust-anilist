use crate::models::media::Media;
use crate::models::media::edge::MediaEdge;
use crate::models::util::page_info::PageInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaConnection {
    pub edges: Option<Vec<MediaEdge>>,
    pub nodes: Option<Vec<Media>>,
    pub page_info: Option<PageInfo>,
}
