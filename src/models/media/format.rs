use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaFormat {
    #[serde(alias = "TV")]
    Tv,
    #[serde(alias = "TV_SHORT")]
    TvShort,
    #[serde(alias = "MOVIE")]
    Movie,
    #[serde(alias = "SPECIAL")]
    Special,
    #[serde(alias = "OVA")]
    Ova,
    #[serde(alias = "ONA")]
    Ona,
    #[serde(alias = "MUSIC")]
    Music,
    #[serde(alias = "MANGA")]
    Manga,
    #[serde(alias = "NOVEL")]
    Novel,
    #[serde(alias = "ONE_SHOT")]
    OneShot,
}
