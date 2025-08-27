use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "ANIME")]
    Anime,
    #[serde(rename = "MANGA")]
    Manga,
}

impl Display for MediaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaType::Anime => f.write_str("ANIME"),
            MediaType::Manga => f.write_str("MANGA"),
        }
    }
}
