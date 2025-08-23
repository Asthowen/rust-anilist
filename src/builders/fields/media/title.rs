#[derive(Copy, Clone)]
pub enum MediaTitleField {
    Romaji,
    English,
    Native,
    UserPreferred,
}

impl MediaTitleField {
    pub const fn all() -> &'static [MediaTitleField] {
        &[
            MediaTitleField::Romaji,
            MediaTitleField::English,
            MediaTitleField::Native,
            MediaTitleField::UserPreferred,
        ]
    }
}

impl From<MediaTitleField> for String {
    fn from(value: MediaTitleField) -> Self {
        match value {
            MediaTitleField::Romaji => "romaji".to_owned(),
            MediaTitleField::English => "english".to_owned(),
            MediaTitleField::Native => "native".to_owned(),
            MediaTitleField::UserPreferred => "userPreferred".to_owned(),
        }
    }
}
