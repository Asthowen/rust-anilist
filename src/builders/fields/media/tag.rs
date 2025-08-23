#[derive(Copy, Clone)]
pub enum MediaTagField {
    Id,
    Name,
    Description,
    Category,
    Rank,
    IsGeneralSpoiler,
    IsMediaSpoiler,
    IsAdult,
    UserId,
}

impl MediaTagField {
    pub const fn all() -> &'static [MediaTagField] {
        &[
            MediaTagField::Id,
            MediaTagField::Name,
            MediaTagField::Description,
            MediaTagField::Category,
            MediaTagField::Rank,
            MediaTagField::IsGeneralSpoiler,
            MediaTagField::IsMediaSpoiler,
            MediaTagField::IsAdult,
            MediaTagField::UserId,
        ]
    }
}

impl From<MediaTagField> for &'static str {
    fn from(value: MediaTagField) -> Self {
        match value {
            MediaTagField::Id => "id",
            MediaTagField::Name => "name",
            MediaTagField::Description => "description",
            MediaTagField::Category => "category",
            MediaTagField::Rank => "rank",
            MediaTagField::IsGeneralSpoiler => "isGeneralSpoiler",
            MediaTagField::IsMediaSpoiler => "isMediaSpoiler",
            MediaTagField::IsAdult => "isAdult",
            MediaTagField::UserId => "userId",
        }
    }
}
