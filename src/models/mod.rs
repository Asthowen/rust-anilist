pub mod anime;
pub mod character;
pub mod color;
pub mod cover;
pub mod date;
pub mod format;
pub mod gender;
pub mod image;
pub mod language;
pub mod link;
pub mod manga;
pub mod name;
pub mod notification;
pub mod occupations;
pub mod person;
pub mod relation;
pub mod score;
pub mod season;
pub mod source;
pub mod status;
pub mod studio;
pub mod tag;
pub mod title;
pub mod user;

pub use anime::Anime;
pub use character::{Character, Role as CharacterRole};
pub use color::Color;
pub use cover::Cover;
pub use date::Date;
pub use format::Format;
pub use gender::Gender;
pub use image::Image;
pub use language::Language;
pub use link::{Link, Type as LinkType};
pub use manga::Manga;
pub use name::Name;
pub use notification::{Notification, NotificationOption, Type as NotificationType};
pub use person::Person;
pub use relation::{Relation, Type as RelationType};
pub use score::{Format as ScoreFormat, Score};
pub use season::Season;
use serde::{Deserialize, Serialize};
pub use source::Source;
pub use status::Status;
pub use studio::Studio;
pub use tag::Tag;
pub use title::Title;
pub use user::User;

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum MediaType {
    Anime,
    Manga,
    #[default]
    Unknown,
}
