use crate::models::media::relation::MediaRelation;
use crate::models::{Character, CharacterRole, Media, Staff, StaffRoleType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEdge {
    pub node: Option<Media>,
    pub id: Option<i32>,
    pub relation_type: Option<MediaRelation>,
    pub is_main_studio: Option<bool>,
    pub characters: Option<Vec<Character>>,
    pub character_role: Option<CharacterRole>,
    pub character_name: Option<String>,
    pub role_notes: Option<String>,
    pub dub_group: Option<String>,
    pub staff_role: Option<String>,
    pub voice_actors: Option<Vec<Staff>>,
    pub voice_actor_roles: Option<Vec<StaffRoleType>>,
    pub favourite_order: Option<i32>,
}
