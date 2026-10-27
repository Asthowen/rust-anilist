use serde::{Deserialize, Serialize};

use crate::models::{Character, CharacterEdge, PageInfo};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterConnection {
    pub edges: Option<Vec<CharacterEdge>>,
    pub nodes: Option<Vec<Character>>,
    pub page_info: Option<PageInfo>,
}
