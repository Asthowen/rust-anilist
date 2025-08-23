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

impl From<MediaTrailerField> for String {
    fn from(value: MediaTrailerField) -> Self {
        match value {
            MediaTrailerField::Id => "id".to_owned(),
            MediaTrailerField::Site => "site".to_owned(),
            MediaTrailerField::Thumbnail => "thumbnail".to_owned(),
        }
    }
}
