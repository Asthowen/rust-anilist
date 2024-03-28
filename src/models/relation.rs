// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Anime;
use crate::models::Manga;
use crate::models::MediaType;
use serde::{Deserialize, Serialize};

// TODO: Use generic type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relation {
    pub media_type: MediaType,
    pub anime: Option<Anime>,
    pub manga: Option<Manga>,
    pub id: i64,
    pub relation_type: Type,
    pub is_main_studio: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Adaptation,
    Prequel,
    Sequel,
    Parent,
    SideStory,
    Character,
    Summary,
    Alternative,
    SpinOff,
    Other,
    Source,
    Compilation,
    Contains,
}
