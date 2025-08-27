use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaFormat {
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TV_SHORT")]
    TvShort,
    #[serde(rename = "MOVIE")]
    Movie,
    #[serde(rename = "SPECIAL")]
    Special,
    #[serde(rename = "OVA")]
    Ova,
    #[serde(rename = "ONA")]
    Ona,
    #[serde(rename = "MUSIC")]
    Music,
    #[serde(rename = "MANGA")]
    Manga,
    #[serde(rename = "NOVEL")]
    Novel,
    #[serde(rename = "ONE_SHOT")]
    OneShot,
}
