pub(crate) mod connection;
pub(crate) mod edge;
pub(crate) mod image;
pub(crate) mod name;
pub(crate) mod role_type;

use crate::models::graphql::fuzzy_date::FuzzyDate;
use crate::models::media::connection::MediaConnection;
use crate::models::{CharacterConnection, StaffImage, StaffName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Staff {
    pub id: Option<i32>,
    pub name: Option<StaffName>,
    pub language_v2: Option<String>,
    pub image: Option<StaffImage>,
    pub description: Option<String>,
    pub primary_occupations: Option<Vec<String>>,
    pub gender: Option<String>,
    pub date_of_birth: Option<FuzzyDate>,
    pub date_of_death: Option<FuzzyDate>,
    pub age: Option<i32>,
    pub years_active: Option<Vec<i32>>,
    pub home_town: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub site_url: Option<String>,
    pub staff_media: Option<MediaConnection>,
    pub characters: Option<CharacterConnection>,
    pub character_media: Option<MediaConnection>,
    // pub submitter: Option<User>,
    pub submission_status: Option<i32>,
    pub submission_notes: Option<String>,
    pub favourites: Option<i32>,
    pub mod_notes: Option<String>,
}
