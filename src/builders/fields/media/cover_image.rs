#[derive(Copy, Clone)]
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

impl From<MediaCoverImageField> for &'static str {
    fn from(value: MediaCoverImageField) -> Self {
        match value {
            MediaCoverImageField::ExtraLarge => "extraLarge",
            MediaCoverImageField::Large => "large",
            MediaCoverImageField::Medium => "medium",
            MediaCoverImageField::Color => "color",
        }
    }
}
