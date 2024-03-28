// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Studio {
    pub id: i64,
    pub name: String,
    pub is_animation_studio: bool,
    pub url: String,
    pub is_favourite: Option<bool>,
    pub favourites: i64,
}

impl Studio {
    pub(crate) fn parse(data: &serde_json::Value, studio: Option<Studio>) -> Self {
        let mut studio = studio.unwrap_or_default();

        studio.id = data["id"].as_i64().unwrap_or_default();
        studio.name = data["name"].as_str().unwrap_or_default().to_owned();
        studio.is_animation_studio = data["isAnimationStudio"].as_bool().unwrap_or_default();
        studio.url = data["siteUrl"].as_str().unwrap_or_default().to_owned();
        studio.is_favourite = data["isFavourite"].as_bool();
        studio.favourites = data["favourites"].as_i64().unwrap_or_default();

        studio
    }

    // pub async fn get_medias<T>() -> Result<T> {
    //     todo!()
    // }
}
