#[derive(Copy, Clone)]
pub enum MediaRankField {
    Id,
    Rank,
    Type,
    Format,
    Year,
    Season,
    AllTime,
    Context,
}

impl MediaRankField {
    pub const fn all() -> &'static [MediaRankField] {
        &[
            MediaRankField::Id,
            MediaRankField::Rank,
            MediaRankField::Type,
            MediaRankField::Format,
            MediaRankField::Year,
            MediaRankField::Season,
            MediaRankField::AllTime,
            MediaRankField::Context,
        ]
    }
}

impl From<MediaRankField> for &'static str {
    fn from(value: MediaRankField) -> Self {
        match value {
            MediaRankField::Id => "id",
            MediaRankField::Rank => "rank",
            MediaRankField::Type => "type",
            MediaRankField::Format => "format",
            MediaRankField::Year => "year",
            MediaRankField::Season => "season",
            MediaRankField::AllTime => "allTime",
            MediaRankField::Context => "context",
        }
    }
}
