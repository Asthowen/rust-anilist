// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Source {
    #[default]
    Original,
    Manga,
    LightNovel,
    VisualNovel,
    VideoGame,
    Other,
    Novel,
    Doujinshi,
    Anime,
    WebNovel,
    LiveAction,
    Game,
    Comic,
    MultimediaProject,
    PictureBook,
}
