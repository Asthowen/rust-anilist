// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Anime;
use crate::models::Character;
use crate::models::Color;
use crate::models::Format;
use crate::models::Image;
use crate::models::Manga;
use crate::models::NotificationOption;
use crate::models::Person;
use crate::models::Status;
use crate::models::Studio;
use crate::models::{Score, ScoreFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    about: Option<String>,
    avatar: Option<Image>,
    banner: Option<String>,
    is_following: Option<bool>,
    is_follower: Option<bool>,
    is_blocked: Option<bool>,
    options: Option<Options>,
    media_list_options: MediaListOptions,
    favourites: Favourites,
    statistics: UserStatisticTypes,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
struct Options {
    title_language: UserTitleLanguage,
    display_adult_content: bool,
    airing_notifications: bool,
    profile_color: Color,
    notifications_options: Vec<NotificationOption>,
    timezone: String,
    activity_merge_time: i32,
    staff_name_language: UserStaffNameLanguage,
    restrict_messages_to_following: bool,
    disabled_list_activity: Vec<ListActivityOption>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum UserTitleLanguage {
    #[default]
    Romaji,
    English,
    Native,
    RomajiStylised,
    EnglishStylised,
    NativeStylised,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum UserStaffNameLanguage {
    RomajiWestern,
    #[default]
    Romaji,
    Native,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListActivityOption {
    status: Status,
    disabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaListOptions {
    score_format: ScoreFormat,
    row_order: String,
    anime_list: MediaListTypeOptions,
    manga_list: MediaListTypeOptions,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaListTypeOptions {
    section_order: Vec<String>,
    split_completed_section_by_format: bool,
    custom_lists: Vec<String>,
    advanced_scoring: Vec<String>,
    advanced_scoring_enabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Favourites {
    anime: Vec<Anime>,
    manga: Vec<Manga>,
    characters: Vec<Character>,
    staff: Vec<Person>,
    studios: Vec<Studio>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserStatisticTypes {
    anime: UserStatistics,
    manga: UserStatistics,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserStatistics {
    count: i32,
    score: Score,
    standard_deviation: f32,
    minutes_watched: Option<i32>,
    episodes_watched: Option<i32>,
    chapters_read: Option<i32>,
    volumes_read: Option<i32>,
    formats: Vec<UserFormatStatistic>,
    statuses: Vec<UserStatusStatistic>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserFormatStatistic {
    count: i32,
    score: Score,
    minutes_watched: Option<i32>,
    chapters_read: Option<i32>,
    media_ids: Vec<i32>,
    format: Format,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserStatusStatistic {
    count: i32,
    score: Score,
    minutes_watched: Option<i32>,
    chapters_read: Option<i32>,
    media_ids: Vec<i32>,
    status: Status,
}
