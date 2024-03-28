// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::occupations::Occupation;
use crate::models::Character;
use crate::models::Date;
use crate::models::Gender;
use crate::models::Image;
use crate::models::Language;
use crate::models::Name;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: i64,
    pub name: Name,
    pub language: Language,
    pub image: Option<Image>,
    pub description: Option<String>,
    pub primary_occupations: Option<Vec<Occupation>>,
    pub gender: Gender,
    pub date_of_birth: Option<Date>,
    pub date_of_death: Option<Date>,
    pub age: Option<i64>,
    pub years_active: Option<(u64, u64)>,
    pub home_town: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub url: String,
    pub characters: Option<Vec<Character>>,
    pub favourites: i64,
    pub mod_notes: Option<String>,
    pub(crate) is_full_loaded: bool,
}

impl Person {
    pub(crate) fn parse(data: &serde_json::Value) -> Self {
        let mut person: Person = Person {
            id: data["id"].as_i64().unwrap_or_default(),
            ..Default::default()
        };

        if let Some(name_object) = data["name"].as_object() {
            let name = Name {
                first: name_object["first"].as_str().unwrap_or_default().to_owned(),
                middle: name_object["middle"].as_str().map(String::from),
                last: name_object["last"].as_str().map(String::from),
                full: name_object["full"].as_str().unwrap_or_default().to_owned(),
                native: name_object["native"].as_str().map(String::from),
                alternative: name_object["alternative"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .map(|n| n.as_str().unwrap_or_default().to_owned())
                            .collect()
                    })
                    .unwrap_or_default(),
                alternative_spoiler: Vec::default(),
                user_preferred: name_object["userPreferred"].as_str().map(String::from),
            };

            person.name = name;
        }

        if let Some(language) = data["languageV2"].as_str() {
            person.language = match language.to_uppercase().as_str() {
                "ENGLISH" => Language::English,
                "KOREAN" => Language::Korean,
                "ITALIAN" => Language::Italian,
                "SPANISH" => Language::Spanish,
                "PORTUGUESE" => Language::Portuguese,
                "FRENCH" => Language::French,
                "GERMAN" => Language::German,
                "HEBREW" => Language::Hebrew,
                "HUNGARIAN" => Language::Hungarian,
                "CHINESE" => Language::Chinese,
                "ARABIC" => Language::Arabic,
                "FILIPINO" => Language::Filipino,
                "CATALAN" => Language::Catalan,
                "FINNISH" => Language::Finnish,
                "TURKISH" => Language::Turkish,
                "DUTCH" => Language::Dutch,
                "SWEDISH" => Language::Swedish,
                "THAI" => Language::Thai,
                "TAGALOG" => Language::Tagalog,
                "MALAYSIAN" => Language::Malaysian,
                "INDONESIAN" => Language::Indonesian,
                "VIETNAMESE" => Language::Vietnamese,
                "NEPALI" => Language::Nepali,
                "HINDI" => Language::Hindi,
                "URDU" => Language::Urdu,
                _ => Language::default(),
            };
        }

        if let Some(image_object) = data["image"].as_object() {
            person.image = Some(Image {
                large: image_object["large"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                medium: image_object["medium"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            });
        }

        if let Some(date_of_birth) = data["dateOfBirth"].as_object() {
            let date = Date {
                year: date_of_birth["year"].as_i64(),
                month: date_of_birth["month"].as_i64(),
                day: date_of_birth["day"].as_i64(),
            };

            person.date_of_birth = Some(date);
        }

        if let Some(date_of_death) = data["dateOfDeath"].as_object() {
            let date = Date {
                year: date_of_death["year"].as_i64(),
                month: date_of_death["month"].as_i64(),
                day: date_of_death["day"].as_i64(),
            };

            person.date_of_death = Some(date);
        }

        person.age = data["age"].as_i64();

        if let Some(years_active) = data["yearsActive"].as_array() {
            person.years_active = match years_active.len() {
                2 => Some((
                    years_active[0].as_u64().unwrap_or_default(),
                    years_active[1].as_u64().unwrap_or_default(),
                )),
                1 => Some((years_active[0].as_u64().unwrap_or_default(), 0)),
                _ => None,
            };
        }

        person.home_town = data["homeTown"].as_str().map(String::from);
        person.blood_type = data["bloodType"].as_str().map(String::from);
        person.is_favourite = data["isFavourite"].as_bool();
        person.is_favourite_blocked = data["isFavouriteBlocked"].as_bool();
        person.url = data["siteUrl"].as_str().unwrap_or_default().to_owned();

        if let Some(characters) = data["characters"]
            .as_object()
            .and_then(|obj| obj["nodes"].as_array())
        {
            let characters: Vec<Character> = characters.iter().map(Character::parse).collect();
            person.characters = Some(characters);
        }

        person.favourites = data["favourites"].as_i64().unwrap_or_default();

        if let Some(primary_occupations) = data["primaryOccupations"].as_array() {
            let occupations = primary_occupations
                .iter()
                .filter_map(|occupation| {
                    occupation.as_str().and_then(|occ| {
                        match occ.to_uppercase().replace(' ', "").as_str() {
                            "ANIMATOR" => Some(Occupation::Animator),
                            "ARRANGER" => Some(Occupation::Arranger),
                            "ARTIST" => Some(Occupation::Artist),
                            "AUDIOENGINEER" => Some(Occupation::AudioEngineer),
                            "BACKGROUNDARTIST" => Some(Occupation::BackgroundArtist),
                            "BAND" => Some(Occupation::Band),
                            "CGARTIST" => Some(Occupation::CGArtist),
                            "COMPOSER" => Some(Occupation::Composer),
                            "COMPOSITEARTIST" => Some(Occupation::CompositeArtist),
                            "DESIGNER" => Some(Occupation::Designer),
                            "DIRECTOR" => Some(Occupation::Director),
                            "EDITOR" => Some(Occupation::Editor),
                            "ILLUSTRATOR" => Some(Occupation::Illustrator),
                            "LYRICIST" => Some(Occupation::Lyricist),
                            "MANGAKA" => Some(Occupation::Mangaka),
                            "MUSICIAN" => Some(Occupation::Musician),
                            "PAINTER" => Some(Occupation::Painter),
                            "PRODUCER" => Some(Occupation::Producer),
                            "PRODUCTIONMANAGER" => Some(Occupation::ProductionManager),
                            "SCRIPTWRITER" => Some(Occupation::ScriptWriter),
                            "STORYBOARDARTIST" => Some(Occupation::StoryBoardArtist),
                            "TRANSLATOR" => Some(Occupation::Translator),
                            "VOCALIST" => Some(Occupation::Vocalist),
                            "VOICEACTOR" => Some(Occupation::VoiceActor),
                            "WRITER" => Some(Occupation::Writer),
                            _ => None,
                        }
                    })
                })
                .collect::<Vec<_>>();

            if !occupations.is_empty() {
                person.primary_occupations = Some(occupations);
            }
        }

        person.mod_notes = data["modNotes"].as_str().map(String::from);

        person
    }
}
