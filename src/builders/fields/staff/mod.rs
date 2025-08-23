pub(crate) mod connection;
pub(crate) mod edge;
pub(crate) mod image;
pub(crate) mod name;
pub(crate) mod role_type;

use crate::builders::fields::{
    CharacterConnectionField, CharacterEdgeField, JoinFields, MediaConnectionField, PageInfoField,
    StaffImageField, StaffNameField,
};

#[derive(Copy, Clone)]
pub enum StaffField<'a> {
    Id,
    Name(&'a [StaffNameField]),
    LanguageV2,
    Image(&'a [StaffImageField]),
    Description,
    PrimaryOccupations,
    Gender,
    DateOfBirth,
    DateOfDeath,
    Age,
    YearsActive,
    HomeTown,
    BloodType,
    IsFavourite,
    IsFavouriteBlocked,
    SiteUrl,
    StaffMedia(&'a [MediaConnectionField<'a>]),
    Characters(&'a [CharacterConnectionField<'a>]),
    CharacterMedia(&'a [MediaConnectionField<'a>]),
    SubmissionStatus,
    SubmissionNotes,
    Favourites,
    ModNotes,
}

impl StaffField<'_> {
    pub const fn all() -> &'static [StaffField<'static>] {
        const ALL: &[StaffField<'static>] = &[
            StaffField::Id,
            StaffField::Name(StaffNameField::all()),
            StaffField::LanguageV2,
            StaffField::Image(StaffImageField::all()),
            StaffField::Description,
            StaffField::PrimaryOccupations,
            StaffField::Gender,
            StaffField::DateOfBirth,
            StaffField::DateOfDeath,
            StaffField::Age,
            StaffField::YearsActive,
            StaffField::HomeTown,
            StaffField::BloodType,
            StaffField::IsFavourite,
            StaffField::IsFavouriteBlocked,
            StaffField::SiteUrl,
            StaffField::StaffMedia(MediaConnectionField::all()),
            StaffField::Characters(&[
                CharacterConnectionField::Edges(&[
                    CharacterEdgeField::Id,
                    CharacterEdgeField::Role,
                    CharacterEdgeField::Name,
                    CharacterEdgeField::FavouriteOrder,
                ]),
                CharacterConnectionField::PageInfo(PageInfoField::all()),
            ]),
            StaffField::CharacterMedia(MediaConnectionField::all()),
            StaffField::SubmissionStatus,
            StaffField::SubmissionNotes,
            StaffField::Favourites,
            StaffField::ModNotes,
        ];

        ALL
    }
}

impl From<StaffField<'_>> for String {
    fn from(value: StaffField) -> Self {
        match value {
            StaffField::Id => "id".to_owned(),
            StaffField::Name(fields) => format!("name {{ {} }}", fields.join_fields()),
            StaffField::LanguageV2 => "languageV2".to_owned(),
            StaffField::Image(fields) => format!("image {{ {} }}", fields.join_fields()),
            StaffField::Description => "description".to_owned(),
            StaffField::PrimaryOccupations => "primaryOccupations".to_owned(),
            StaffField::Gender => "gender".to_owned(),
            StaffField::DateOfBirth => "dateOfBirth { year month day }".to_owned(),
            StaffField::DateOfDeath => "dateOfDeath { year month day }".to_owned(),
            StaffField::Age => "age".to_owned(),
            StaffField::YearsActive => "yearsActive".to_owned(),
            StaffField::HomeTown => "homeTown".to_owned(),
            StaffField::BloodType => "bloodType".to_owned(),
            StaffField::IsFavourite => "isFavourite".to_owned(),
            StaffField::IsFavouriteBlocked => "isFavouriteBlocked".to_owned(),
            StaffField::SiteUrl => "siteUrl".to_owned(),
            StaffField::StaffMedia(fields) => format!("staffMedia {{ {} }}", fields.join_fields()),
            StaffField::Characters(fields) => format!("characters {{ {} }}", fields.join_fields()),
            StaffField::CharacterMedia(fields) => {
                format!("characterMedia {{ {} }}", fields.join_fields())
            }
            StaffField::SubmissionStatus => "submissionStatus".to_owned(),
            StaffField::SubmissionNotes => "submissionNotes".to_owned(),
            StaffField::Favourites => "favourites".to_owned(),
            StaffField::ModNotes => "modNotes".to_owned(),
        }
    }
}
