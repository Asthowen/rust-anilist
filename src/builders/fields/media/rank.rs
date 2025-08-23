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

impl From<MediaRankField> for String {
    fn from(value: MediaRankField) -> Self {
        match value {
            MediaRankField::Id => "id".to_owned(),
            MediaRankField::Rank => "rank".to_owned(),
            MediaRankField::Type => "type".to_owned(),
            MediaRankField::Format => "format".to_owned(),
            MediaRankField::Year => "year".to_owned(),
            MediaRankField::Season => "season".to_owned(),
            MediaRankField::AllTime => "allTime".to_owned(),
            MediaRankField::Context => "context".to_owned(),
        }
    }
}
