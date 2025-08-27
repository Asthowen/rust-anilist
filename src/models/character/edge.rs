use crate::models::{Character, CharacterRole, Media, Staff, StaffRoleType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterEdge {
    pub node: Option<Character>,
    pub id: Option<i32>,
    pub role: Option<CharacterRole>,
    pub name: Option<String>,
    pub voice_actors: Option<Vec<Staff>>,
    pub voice_actor_roles: Option<Vec<StaffRoleType>>,
    pub media: Option<Vec<Media>>,
    pub favourite_order: Option<i32>,
}
