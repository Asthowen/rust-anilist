#[derive(Copy, Clone)]
pub enum MediaStreamingEpisodesField {
    Title,
    Thumbnail,
    Url,
    Site,
}

impl MediaStreamingEpisodesField {
    pub const fn all() -> &'static [MediaStreamingEpisodesField] {
        &[
            MediaStreamingEpisodesField::Title,
            MediaStreamingEpisodesField::Thumbnail,
            MediaStreamingEpisodesField::Url,
            MediaStreamingEpisodesField::Site,
        ]
    }
}

impl From<MediaStreamingEpisodesField> for String {
    fn from(value: MediaStreamingEpisodesField) -> Self {
        match value {
            MediaStreamingEpisodesField::Title => "title".to_owned(),
            MediaStreamingEpisodesField::Thumbnail => "thumbnail".to_owned(),
            MediaStreamingEpisodesField::Url => "url".to_owned(),
            MediaStreamingEpisodesField::Site => "site".to_owned(),
        }
    }
}
