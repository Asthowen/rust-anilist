#[derive(Copy, Clone)]
pub enum AiringScheduleField {
    Id,
    AiringAt,
    TimeUntilAiring,
    Episode,
    MediaId,
}

impl AiringScheduleField {
    pub const fn all() -> &'static [AiringScheduleField] {
        &[
            AiringScheduleField::Id,
            AiringScheduleField::AiringAt,
            AiringScheduleField::TimeUntilAiring,
            AiringScheduleField::Episode,
            AiringScheduleField::MediaId,
        ]
    }
}

impl From<AiringScheduleField> for String {
    fn from(value: AiringScheduleField) -> Self {
        match value {
            AiringScheduleField::Id => "id".to_owned(),
            AiringScheduleField::AiringAt => "airingAt".to_owned(),
            AiringScheduleField::TimeUntilAiring => "timeUntilAiring".to_owned(),
            AiringScheduleField::Episode => "episode".to_owned(),
            AiringScheduleField::MediaId => "mediaId".to_owned(),
        }
    }
}
