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

impl From<MediaTitleField> for &'static str {
    fn from(value: MediaTitleField) -> Self {
        match value {
            MediaTitleField::Romaji => "romaji",
            MediaTitleField::English => "english",
            MediaTitleField::Native => "native",
            MediaTitleField::UserPreferred => "userPreferred",
        }
    }
}
