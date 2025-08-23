#[derive(Copy, Clone)]
pub enum MediaTrailerField {
    Id,
    Site,
    Thumbnail,
}

impl MediaTrailerField {
    pub const fn all() -> &'static [MediaTrailerField] {
        &[
            MediaTrailerField::Id,
            MediaTrailerField::Site,
            MediaTrailerField::Thumbnail,
        ]
    }
}

impl From<MediaTrailerField> for &'static str {
    fn from(value: MediaTrailerField) -> Self {
        match value {
            MediaTrailerField::Id => "id",
            MediaTrailerField::Site => "site",
            MediaTrailerField::Thumbnail => "thumbnail",
        }
    }
}
