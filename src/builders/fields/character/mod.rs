pub(crate) mod connection;
pub(crate) mod edge;
pub(crate) mod image;
pub(crate) mod name;

use crate::builders::fields::media::connection::MediaConnectionField;
use crate::builders::fields::{CharacterImageField, CharacterNameField};

#[derive(Copy, Clone)]
pub enum CharacterField<'a> {
    Id,
    Name(&'a [CharacterNameField]),
    Image(&'a [CharacterImageField]),
    Description,
    Gender,
    DateOfBirth,
    Age,
    BloodType,
    IsFavourite,
    IsFavouriteBlocked,
    SiteUrl,
    Media(&'a [MediaConnectionField<'a>]),
    Favourites,
    ModNotes,
}

impl CharacterField<'_> {
    pub const fn all() -> &'static [CharacterField<'static>] {
        const ALL: &[CharacterField<'static>] = &[
            CharacterField::Id,
            CharacterField::Name(CharacterNameField::all()),
            CharacterField::Image(CharacterImageField::all()),
            CharacterField::Description,
            CharacterField::Gender,
            CharacterField::DateOfBirth,
            CharacterField::Age,
            CharacterField::BloodType,
            CharacterField::IsFavourite,
            CharacterField::IsFavouriteBlocked,
            CharacterField::SiteUrl,
            CharacterField::Media(MediaConnectionField::all()),
            CharacterField::Favourites,
            CharacterField::ModNotes,
        ];

        ALL
    }
}

impl From<CharacterField<'_>> for String {
    fn from(value: CharacterField) -> Self {
        match value {
            CharacterField::Id => "id".to_owned(),
            CharacterField::Name(fields) => format!(
                "name {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            CharacterField::Image(fields) => format!(
                "image {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            CharacterField::Description => "description".to_owned(),
            CharacterField::Gender => "gender".to_owned(),
            CharacterField::DateOfBirth => "dateOfBirth { year month day }".to_owned(),
            CharacterField::Age => "age".to_owned(),
            CharacterField::BloodType => "bloodType".to_owned(),
            CharacterField::IsFavourite => "isFavourite".to_owned(),
            CharacterField::IsFavouriteBlocked => "isFavouriteBlocked".to_owned(),
            CharacterField::SiteUrl => "siteUrl".to_owned(),
            CharacterField::Media(fields) => format!(
                "media {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            CharacterField::Favourites => "favourites".to_owned(),
            CharacterField::ModNotes => "modNotes".to_owned(),
        }
    }
}
