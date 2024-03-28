// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Character;
use crate::models::Color;
use crate::models::Cover;
use crate::models::Date;
use crate::models::Format;
use crate::models::Language;
use crate::models::Manga;
use crate::models::MediaType;
use crate::models::Person;
use crate::models::Score;
use crate::models::Season;
use crate::models::Source;
use crate::models::Status;
use crate::models::Studio;
use crate::models::Tag;
use crate::models::Title;
use crate::models::{Link, LinkType};
use crate::models::{Relation, RelationType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Anime {
    pub id: i64,
    pub id_mal: Option<i64>,
    pub title: Title,
    pub format: Format,
    pub status: Status,
    pub description: String,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub season: Option<Season>,
    pub season_year: Option<i64>,
    pub season_int: Option<i64>,
    pub episodes: Option<i64>,
    pub duration: Option<i64>,
    pub country_of_origin: Option<String>,
    pub is_licensed: Option<bool>,
    pub source: Option<Source>,
    pub hashtag: Option<String>,
    pub updated_at: Option<i64>,
    pub cover: Cover,
    pub banner: Option<String>,
    pub genres: Option<Vec<String>>,
    pub synonyms: Option<Vec<String>>,
    pub score: Score,
    pub popularity: Option<i64>,
    pub is_locked: Option<bool>,
    pub trending: Option<i64>,
    pub favourites: Option<i64>,
    pub tags: Option<Vec<Tag>>,
    pub relations: Option<Vec<Relation>>,
    pub characters: Option<Vec<Character>>,
    pub staff: Option<Vec<Person>>,
    pub studios: Option<Vec<Studio>>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub is_adult: Option<bool>,
    pub next_airing_episode: Option<AiringEpisode>,
    pub external_links: Option<Vec<Link>>,
    pub streaming_episodes: Option<Vec<Link>>,
    pub url: String,
    pub(crate) is_full_loaded: bool,
}

impl Anime {
    pub(crate) fn parse(data: &serde_json::Value) -> Self {
        let mut anime: Anime = Anime {
            id: data["id"].as_i64().unwrap_or_default(),
            id_mal: data["idMal"].as_i64(),
            ..Default::default()
        };

        anime.title = if let Some(title) = data["title"].as_object() {
            Title {
                romaji: title["romaji"].as_str().map(String::from),
                english: title["english"].as_str().map(String::from),
                native: title["native"].as_str().unwrap_or_default().to_owned(),
                user_preferred: title["userPreferred"].as_str().map(String::from),
            }
        } else {
            Title::default()
        };

        let format = data["format"].as_str().unwrap_or_default();
        anime.format = match format {
            "TV_SHORT" => Format::TvShort,
            "MOVIE" => Format::Movie,
            "SPECIAL" => Format::Special,
            "OVA" => Format::Ova,
            "ONA" => Format::Ona,
            "MUSIC" => Format::Music,
            "MANGA" => Format::Manga,
            "NOVEL" => Format::Novel,
            "ONE_SHOT" => Format::OneShot,
            _ => Format::default(),
        };

        let status = data["status"].as_str().unwrap_or_default();
        anime.status = match status {
            "FINISHED" => Status::Finished,
            "RELEASING" => Status::Releasing,
            "CANCELLED" => Status::Cancelled,
            "HIATUS" => Status::Hiatus,
            _ => Status::default(),
        };

        anime.description = data["description"]
            .as_str()
            .map(String::from)
            .unwrap_or_default();

        if let Some(start_date) = data["startDate"].as_object() {
            let date = Date {
                year: start_date["year"].as_i64(),
                month: start_date["month"].as_i64(),
                day: start_date["day"].as_i64(),
            };

            anime.start_date = Some(date);
        }

        if let Some(end_date) = data["endDate"].as_object() {
            let date = Date {
                year: end_date["year"].as_i64(),
                month: end_date["month"].as_i64(),
                day: end_date["day"].as_i64(),
            };

            anime.end_date = Some(date);
        }

        if let Some(season) = data["season"].as_str() {
            anime.season = match season {
                "WINTER" => Some(Season::Winter),
                "SPRING" => Some(Season::Spring),
                "SUMMER" => Some(Season::Summer),
                _ => Some(Season::Fall),
            };
        }

        anime.season_year = data["seasonYear"].as_i64();
        anime.season_int = data["seasonInt"].as_i64();
        anime.episodes = data["episodes"].as_i64();
        anime.duration = data["duration"].as_i64();
        anime.country_of_origin = data["countryOfOrigin"].as_str().map(String::from);
        anime.is_licensed = data["isLicensed"].as_bool();

        if let Some(source) = data["source"].as_str() {
            anime.source = match source {
                "MANGA" => Some(Source::Manga),
                "LIGHT_NOVEL" => Some(Source::LightNovel),
                "VISUAL_NOVEL" => Some(Source::VisualNovel),
                "VIDEO_GAME" => Some(Source::VideoGame),
                "OTHER" => Some(Source::Other),
                "NOVEL" => Some(Source::Novel),
                _ => Some(Source::default()),
            };
        }

        anime.hashtag = data["hashtag"].as_str().map(String::from);
        anime.updated_at = data["updatedAt"].as_i64();

        if let Some(cover_image) = data["coverImage"].as_object() {
            let cover = Cover {
                extra_large: cover_image["extraLarge"].as_str().map(String::from),
                large: cover_image["large"].as_str().map(String::from),
                medium: cover_image["medium"].as_str().map(String::from),
                color: cover_image["color"]
                    .as_str()
                    .map(|c| Color::Hex(c.to_owned())),
            };

            anime.cover = cover;
        }

        anime.banner = data["bannerImage"].as_str().map(String::from);

        if let Some(genres_array) = data["genres"].as_array() {
            let genres = genres_array
                .iter()
                .map(|genre| genre.as_str().unwrap_or_default().to_owned())
                .collect::<Vec<String>>();

            anime.genres = Some(genres);
        }

        if let Some(synonyms_array) = data["synonyms"].as_array() {
            let synonyms = synonyms_array
                .iter()
                .map(|synonym| synonym.as_str().unwrap_or_default().to_owned())
                .collect::<Vec<String>>();

            anime.synonyms = Some(synonyms);
        }

        anime.score = Score {
            average: data["averageScore"].as_i64().unwrap_or_default(),
            mean: data["meanScore"].as_i64().unwrap_or_default(),
        };

        anime.popularity = data["popularity"].as_i64();
        anime.is_locked = data["isLocked"].as_bool();
        anime.trending = data["trendig"].as_i64();
        anime.favourites = data["favourites"].as_i64();

        if let Some(tags_array) = data["tags"].as_array() {
            let tags = tags_array
                .iter()
                .map(|tag| Tag {
                    id: tag["id"].as_i64().unwrap_or_default(),
                    name: tag["name"].as_str().unwrap_or_default().to_owned(),
                    description: tag["description"].as_str().unwrap_or_default().to_owned(),
                    category: tag["category"].as_str().unwrap_or_default().to_owned(),
                    rank: tag["rank"].as_i64().unwrap_or_default(),
                    is_general_spoiler: tag["isGeneralSpoiler"].as_bool().unwrap_or_default(),
                    is_media_spoiler: tag["isMediaSpoiler"].as_bool().unwrap_or_default(),
                    is_adult: tag["isAdult"].as_bool().unwrap_or_default(),
                    user_id: tag["userId"].as_i64(),
                })
                .collect::<Vec<Tag>>();

            anime.tags = Some(tags);
        }

        if let Some(relations) = data["relations"].as_object() {
            if let Some(edges) = relations["edges"].as_array() {
                let mut relations: Vec<Relation> = Vec::with_capacity(edges.len());

                for edge in edges {
                    let node = if let Some(node) = edge.get("node") {
                        node
                    } else {
                        continue;
                    };
                    let media_type = match node["type"].as_str().unwrap_or_default() {
                        "ANIME" => MediaType::Anime,
                        "MANGA" => MediaType::Manga,
                        _ => MediaType::default(),
                    };
                    relations.push(Relation {
                        media_type,
                        anime: match media_type {
                            MediaType::Anime => Some(Anime::parse(node)),
                            _ => None,
                        },
                        manga: match media_type {
                            MediaType::Manga => Some(Manga::parse(node)),
                            _ => None,
                        },
                        id: edge["id"].as_i64().unwrap_or_default(),
                        relation_type: match edge["relationType"].as_str().unwrap_or_default() {
                            "ADAPTATION" => RelationType::Adaptation,
                            "PREQUEL" => RelationType::Prequel,
                            "SEQUEL" => RelationType::Sequel,
                            "PARENT" => RelationType::Parent,
                            "SIDE_STORY" => RelationType::SideStory,
                            "CHARACTER" => RelationType::Character,
                            "SUMMARY" => RelationType::Summary,
                            "ALTERNATIVE" => RelationType::Alternative,
                            "SPIN_OFF" => RelationType::SpinOff,
                            "OTHER" => RelationType::Other,
                            "COMPILATION" => RelationType::Compilation,
                            "CONTAINS" => RelationType::Contains,
                            _ => RelationType::Source,
                        },
                        is_main_studio: edge["isMainStudio"].as_bool().unwrap_or_default(),
                    });
                }

                anime.relations = Some(relations);
            }
        }

        if let Some(characters) = data["characters"].as_object() {
            if let Some(nodes) = characters["nodes"].as_array() {
                let mut characters: Vec<Character> = Vec::with_capacity(nodes.len());

                for node in nodes {
                    characters.push(Character::parse(node));
                }

                anime.characters = Some(characters);
            }
        }

        if let Some(persons) = data["staff"].as_object() {
            if let Some(nodes) = persons["nodes"].as_array() {
                let mut staff: Vec<Person> = Vec::with_capacity(nodes.len());

                for node in nodes {
                    staff.push(Person::parse(node));
                }

                anime.staff = Some(staff);
            }
        }

        if let Some(studios) = data["studios"].as_object() {
            if let Some(nodes) = studios["nodes"].as_array() {
                let mut studios: Vec<Studio> = Vec::with_capacity(nodes.len());

                for node in nodes {
                    studios.push(Studio::parse(node, None));
                }

                anime.studios = Some(studios);
            }
        }

        anime.is_favourite = data["isFavourite"].as_bool();
        anime.is_favourite_blocked = data["isFavouriteBlocked"].as_bool();
        anime.is_adult = data["isAdult"].as_bool();

        if let Some(next_airing_episode) = data["nextAiringEpisode"].as_object() {
            anime.next_airing_episode = Some(AiringEpisode {
                id: next_airing_episode["id"].as_i64().unwrap_or_default(),
                at: next_airing_episode["airingAt"].as_i64().unwrap_or_default(),
                time_until: next_airing_episode["timeUntilAiring"]
                    .as_i64()
                    .unwrap_or_default(),
                episode: next_airing_episode["episode"].as_i64().unwrap_or_default(),
            });
        }

        if let Some(external_links_array) = data["externalLinks"].as_array() {
            let mut external_links: Vec<Link> = Vec::with_capacity(external_links_array.len());

            for external_link in external_links_array {
                external_links.push(Link {
                    id: external_link["id"].as_i64(),
                    url: external_link["url"].as_str().unwrap_or_default().to_owned(),
                    site: external_link["site"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                    site_id: external_link["siteId"].as_i64(),
                    link_type: match external_link["type"].as_str().unwrap_or_default() {
                        "STREAMING" => Some(LinkType::Streaming),
                        "SOCIAL" => Some(LinkType::Social),
                        _ => Some(LinkType::default()),
                    },
                    language: match external_link["language"].as_str() {
                        Some(language) => match language.to_uppercase().as_str() {
                            "ENGLISH" => Some(Language::English),
                            "KOREAN" => Some(Language::Korean),
                            "ITALIAN" => Some(Language::Italian),
                            "SPANISH" => Some(Language::Spanish),
                            "PORTUGUESE" => Some(Language::Portuguese),
                            "FRENCH" => Some(Language::French),
                            "GERMAN" => Some(Language::German),
                            "HEBREW" => Some(Language::Hebrew),
                            "HUNGARIAN" => Some(Language::Hungarian),
                            "CHINESE" => Some(Language::Chinese),
                            "ARABIC" => Some(Language::Arabic),
                            "FILIPINO" => Some(Language::Filipino),
                            "CATALAN" => Some(Language::Catalan),
                            "FINNISH" => Some(Language::Finnish),
                            "TURKISH" => Some(Language::Turkish),
                            "DUTCH" => Some(Language::Dutch),
                            "SWEDISH" => Some(Language::Swedish),
                            "THAI" => Some(Language::Thai),
                            "TAGALOG" => Some(Language::Tagalog),
                            "MALAYSIAN" => Some(Language::Malaysian),
                            "INDONESIAN" => Some(Language::Indonesian),
                            "VIETNAMESE" => Some(Language::Vietnamese),
                            "NEPALI" => Some(Language::Nepali),
                            "HINDI" => Some(Language::Hindi),
                            "URDU" => Some(Language::Urdu),
                            _ => Some(Language::default()),
                        },
                        None => None,
                    },
                    color: external_link["color"]
                        .as_str()
                        .map(|hex| Color::Hex(hex.to_owned())),
                    icon: external_link["icon"].as_str().map(|url| url.to_owned()),
                    ..Default::default()
                })
            }

            anime.external_links = Some(external_links);
        }

        if let Some(streaming_episodes_array) = data["streamingEpisodes"].as_array() {
            let streaming_episodes: Vec<Link> = streaming_episodes_array
                .iter()
                .map(|streaming_episode| Link {
                    title: streaming_episode["title"].as_str().map(|s| s.to_owned()),
                    thumbnail: streaming_episode["thumbnail"]
                        .as_str()
                        .map(|s| s.to_owned()),
                    url: streaming_episode["url"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                    site: streaming_episode["site"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                    ..Default::default()
                })
                .collect();

            anime.streaming_episodes = Some(streaming_episodes);
        }

        anime.url = data["siteUrl"].as_str().unwrap_or_default().to_owned();

        anime
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiringEpisode {
    id: i64,
    at: i64,
    time_until: i64,
    episode: i64,
}
