#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MediaCoverImageField {
    ExtraLarge,
    Large,
    Medium,
    Color,
}

impl MediaCoverImageField {
    pub const fn all() -> &'static [MediaCoverImageField] {
        &[
            MediaCoverImageField::ExtraLarge,
            MediaCoverImageField::Large,
            MediaCoverImageField::Medium,
            MediaCoverImageField::Color,
        ]
    }
}

impl From<MediaCoverImageField> for String {
    fn from(value: MediaCoverImageField) -> Self {
        match value {
            MediaCoverImageField::ExtraLarge => "extraLarge".to_owned(),
            MediaCoverImageField::Large => "large".to_owned(),
            MediaCoverImageField::Medium => "medium".to_owned(),
            MediaCoverImageField::Color => "color".to_owned(),
        }
    }
}
