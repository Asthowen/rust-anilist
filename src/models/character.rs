// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Date;
use crate::models::Gender;
use crate::models::Image;
use crate::models::Name;
use crate::models::Person;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub id: i64,
    pub name: Name,
    pub role: Option<Role>,
    pub image: Image,
    pub description: String,
    pub gender: Option<Gender>,
    pub date_of_birth: Option<Date>,
    pub age: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub url: String,
    pub favourites: Option<i64>,
    pub voice_actors: Option<Vec<Person>>,
    pub mod_notes: Option<String>,
    pub(crate) is_full_loaded: bool,
}

impl Character {
    pub(crate) fn parse(data: &serde_json::Value) -> Self {
        Self {
            id: data["id"].as_i64().unwrap_or_default(),
            name: data["name"]
                .as_object()
                .map(|object| Name {
                    first: object["first"].as_str().unwrap_or_default().to_owned(),
                    middle: object["middle"].as_str().map(String::from),
                    last: object["last"].as_str().map(String::from),
                    full: object["full"].as_str().unwrap_or_default().to_owned(),
                    native: object["native"].as_str().map(String::from),
                    alternative: object["alternative"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_str().unwrap_or_default().to_owned())
                        .collect::<Vec<String>>(),
                    alternative_spoiler: object["alternativeSpoiler"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_str().unwrap_or_default().to_owned())
                        .collect::<Vec<String>>(),
                    user_preferred: object["userPreferred"].as_str().map(String::from),
                })
                .unwrap_or_default(),
            role: data["role"]
                .as_str()
                .map(|role| match role.to_ascii_lowercase().as_str() {
                    "main" => Role::Main,
                    "supporting" => Role::Supporting,
                    _ => Role::default(),
                }),
            image: data["image"]
                .as_object()
                .map(|object| Image {
                    large: object["large"].as_str().unwrap_or_default().to_owned(),
                    medium: object["medium"].as_str().unwrap_or_default().to_owned(),
                })
                .unwrap_or_default(),
            description: data["description"].as_str().unwrap_or_default().to_owned(),
            gender: data["gender"].as_str().map(|gender| {
                match gender.to_ascii_lowercase().as_str() {
                    "male" => Gender::Male,
                    "female" => Gender::Female,
                    "nonbinary" => Gender::NonBinary,
                    _ => Gender::Other(gender.to_owned()),
                }
            }),
            date_of_birth: data["dateOfBirth"].as_object().map(|object| {
                Date {
                    year: object["year"].as_i64(),   // TODO: Use u64
                    month: object["month"].as_i64(), // Same as above
                    day: object["day"].as_i64(),     // Same as above
                }
            }),
            age: data["age"].as_str().map(String::from),
            blood_type: data["bloodType"].as_str().map(String::from),
            is_favourite: data["isFavourite"].as_bool(),
            is_favourite_blocked: data["isFavouriteBlocked"].as_bool(),
            url: data["siteUrl"].as_str().unwrap_or_default().to_owned(),
            favourites: data["favourites"].as_i64(),
            mod_notes: data["modNotes"].as_str().map(String::from),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Role {
    #[default]
    Background,
    Main,
    Supporting,
}
