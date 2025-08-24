use crate::models::character::image::CharacterImage;
use crate::models::character::name::CharacterName;
use crate::models::graphql::fuzzy_date::FuzzyDate;
use crate::models::media::connection::MediaConnection;
use serde::{Deserialize, Serialize};

pub(crate) mod connection;
pub(crate) mod edge;
pub(crate) mod image;
pub(crate) mod name;
pub(crate) mod role;
pub(crate) mod sort;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: Option<i32>,
    pub name: Option<CharacterName>,
    pub image: Option<CharacterImage>,
    pub description: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<FuzzyDate>,
    pub age: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub site_url: Option<String>,
    pub media: Option<MediaConnection>,
    pub favourites: Option<i32>,
    pub mod_notes: Option<String>,
}
