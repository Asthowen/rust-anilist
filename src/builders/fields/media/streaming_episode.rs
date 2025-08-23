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

impl From<MediaStreamingEpisodesField> for &'static str {
    fn from(value: MediaStreamingEpisodesField) -> Self {
        match value {
            MediaStreamingEpisodesField::Title => "title",
            MediaStreamingEpisodesField::Thumbnail => "thumbnail",
            MediaStreamingEpisodesField::Url => "url",
            MediaStreamingEpisodesField::Site => "site",
        }
    }
}
