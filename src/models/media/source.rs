use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaSource {
    #[serde(alias = "ORIGINAL")]
    Original,
    #[serde(alias = "MANGA")]
    Manga,
    #[serde(alias = "LIGHT_NOVEL")]
    LightNovel,
    #[serde(alias = "VISUAL_NOVEL")]
    VisualNovel,
    #[serde(alias = "VIDEO_GAME")]
    VideoGame,
    #[serde(alias = "OTHER")]
    Other,
    #[serde(alias = "NOVEL")]
    Novel,
    #[serde(alias = "DOUJINSHI")]
    Doujinshi,
    #[serde(alias = "ANIME")]
    Anime,
    #[serde(alias = "WEB_NOVEL")]
    WebNovel,
    #[serde(alias = "LIVE_ACTION")]
    LiveAction,
    #[serde(alias = "GAME")]
    Game,
    #[serde(alias = "COMIC")]
    Comic,
    #[serde(alias = "MULTIMEDIA_PROJECT")]
    MultimediaProject,
    #[serde(alias = "PICTURE_BOOK")]
    PictureBook,
}
