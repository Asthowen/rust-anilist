use crate::models::Character;
use crate::models::character::edge::CharacterEdge;
use crate::models::util::page_info::PageInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterConnection {
    pub edges: Option<Vec<CharacterEdge>>,
    pub nodes: Option<Vec<Character>>,
    pub page_info: Option<PageInfo>,
}
