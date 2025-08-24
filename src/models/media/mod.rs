use crate::models::airing::schedule::AiringSchedule;
use crate::models::character::connection::CharacterConnection;
use crate::models::graphql::country_code::CountryCode;
use crate::models::graphql::fuzzy_date::FuzzyDate;
use crate::models::media::connection::MediaConnection;
use crate::models::media::cover_image::MediaCoverImage;
use crate::models::media::mtype::MediaType;
use crate::models::staff::connection::StaffConnection;
use crate::models::{
    MediaExternalLink, MediaFormat, MediaSeason, MediaSource, MediaStatus, MediaTag, MediaTitle,
    MediaTrailer,
};
use serde::{Deserialize, Serialize};

pub(crate) mod connection;
pub(crate) mod cover_image;
pub(crate) mod edge;
pub(crate) mod external_link;
pub(crate) mod format;
pub(crate) mod list;
pub(crate) mod list_status;
pub(crate) mod mtype;
pub(crate) mod rank;
pub(crate) mod rank_type;
pub(crate) mod relation;
pub(crate) mod season;
pub(crate) mod source;
pub(crate) mod status;
pub(crate) mod tag;
pub(crate) mod title;
pub(crate) mod trailer;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: i32,
    pub id_mal: Option<i32>,
    pub title: Option<MediaTitle>,
    pub r#type: Option<MediaType>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    pub description: Option<String>,
    pub start_date: Option<FuzzyDate>,
    pub end_date: Option<FuzzyDate>,
    pub season: Option<MediaSeason>,
    pub season_year: Option<i32>,
    pub season_int: Option<i32>,
    pub episodes: Option<i32>,
    pub duration: Option<i32>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub country_of_origin: Option<CountryCode>,
    pub is_licensed: Option<bool>,
    pub source: Option<MediaSource>,
    pub hashtag: Option<String>,
    pub trailer: Option<MediaTrailer>,
    pub updated_at: Option<i32>,
    pub cover_image: Option<MediaCoverImage>,
    pub banner_image: Option<String>,
    pub genres: Option<Vec<String>>,
    pub synonyms: Option<Vec<String>>,
    pub average_score: Option<i32>,
    pub mean_score: Option<i32>,
    pub popularity: Option<i32>,
    pub is_locked: Option<bool>,
    pub trending: Option<i32>,
    pub favourites: Option<i32>,
    pub tags: Option<Vec<MediaTag>>,
    pub relations: Option<MediaConnection>,
    pub characters: Option<CharacterConnection>,
    pub staff: Option<StaffConnection>,
    // pub studios: Option<StudioConnection>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub is_adult: Option<bool>,
    pub next_airing_episode: Option<AiringSchedule>,
    // pub airingSchedule: Option<AiringScheduleConnection>,
    // pub trends: Option<MediaTrendConnection>,
    pub external_links: Option<Vec<MediaExternalLink>>,
    pub site_url: Option<String>,
    pub auto_create_forum_thread: Option<bool>,
    pub is_recommendation_blocked: Option<bool>,
    pub is_review_blocked: Option<bool>,
    pub mod_notes: Option<bool>,
}
