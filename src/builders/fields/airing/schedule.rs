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

impl From<AiringScheduleField> for &'static str {
    fn from(value: AiringScheduleField) -> Self {
        match value {
            AiringScheduleField::Id => "id",
            AiringScheduleField::AiringAt => "airingAt",
            AiringScheduleField::TimeUntilAiring => "timeUntilAiring",
            AiringScheduleField::Episode => "episode",
            AiringScheduleField::MediaId => "mediaId",
        }
    }
}
