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

impl From<MediaTagField> for String {
    fn from(value: MediaTagField) -> Self {
        match value {
            MediaTagField::Id => "id".to_owned(),
            MediaTagField::Name => "name".to_owned(),
            MediaTagField::Description => "description".to_owned(),
            MediaTagField::Category => "category".to_owned(),
            MediaTagField::Rank => "rank".to_owned(),
            MediaTagField::IsGeneralSpoiler => "isGeneralSpoiler".to_owned(),
            MediaTagField::IsMediaSpoiler => "isMediaSpoiler".to_owned(),
            MediaTagField::IsAdult => "isAdult".to_owned(),
            MediaTagField::UserId => "userId".to_owned(),
        }
    }
}
