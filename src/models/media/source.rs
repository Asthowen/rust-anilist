use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaSource {
    #[serde(rename = "ORIGINAL")]
    Original,
    #[serde(rename = "MANGA")]
    Manga,
    #[serde(rename = "LIGHT_NOVEL")]
    LightNovel,
    #[serde(rename = "VISUAL_NOVEL")]
    VisualNovel,
    #[serde(rename = "VIDEO_GAME")]
    VideoGame,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "NOVEL")]
    Novel,
    #[serde(rename = "DOUJINSHI")]
    Doujinshi,
    #[serde(rename = "ANIME")]
    Anime,
    #[serde(rename = "WEB_NOVEL")]
    WebNovel,
    #[serde(rename = "LIVE_ACTION")]
    LiveAction,
    #[serde(rename = "GAME")]
    Game,
    #[serde(rename = "COMIC")]
    Comic,
    #[serde(rename = "MULTIMEDIA_PROJECT")]
    MultimediaProject,
    #[serde(rename = "PICTURE_BOOK")]
    PictureBook,
}
