use crate::builders::fields::{JoinFields, MediaField};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MediaListField<'a> {
    Id,
    UserId,
    MediaId,
    Status,
    Score,
    Progress,
    ProgressVolumes,
    Repeat,
    Priority,
    Private,
    Notes,
    HiddenFromStatusLists,
    CustomLists,
    AdvancedScores,
    StartedAt,
    CompletedAt,
    UpdatedAt,
    CreatedAt,
    Media(&'a [MediaField<'a>]),
}

impl<'a> MediaListField<'a> {
    pub const fn all() -> &'static [MediaListField<'static>] {
        const ALL: &[MediaListField<'static>] = &[
            MediaListField::Id,
            MediaListField::UserId,
            MediaListField::MediaId,
            MediaListField::Status,
            MediaListField::Score,
            MediaListField::Progress,
            MediaListField::ProgressVolumes,
            MediaListField::Repeat,
            MediaListField::Priority,
            MediaListField::Private,
            MediaListField::Notes,
            MediaListField::HiddenFromStatusLists,
            MediaListField::CustomLists,
            MediaListField::AdvancedScores,
            MediaListField::StartedAt,
            MediaListField::CompletedAt,
            MediaListField::UpdatedAt,
            MediaListField::CreatedAt,
            MediaListField::Media(&[MediaField::Id]),
        ];
        ALL
    }
}

impl From<MediaListField<'_>> for String {
    fn from(value: MediaListField) -> Self {
        match value {
            MediaListField::Id => "id".to_owned(),
            MediaListField::UserId => "userId".to_owned(),
            MediaListField::MediaId => "mediaId".to_owned(),
            MediaListField::Status => "status".to_owned(),
            MediaListField::Score => "score".to_owned(),
            MediaListField::Progress => "progress".to_owned(),
            MediaListField::ProgressVolumes => "progressVolumes".to_owned(),
            MediaListField::Repeat => "repeat".to_owned(),
            MediaListField::Priority => "priority".to_owned(),
            MediaListField::Private => "private".to_owned(),
            MediaListField::Notes => "notes".to_owned(),
            MediaListField::HiddenFromStatusLists => "hiddenFromStatusLists".to_owned(),
            MediaListField::CustomLists => "customLists".to_owned(),
            MediaListField::AdvancedScores => "advancedScores".to_owned(),
            MediaListField::StartedAt => "startedAt { year month day }".to_owned(),
            MediaListField::CompletedAt => "completedAt { year month day }".to_owned(),
            MediaListField::UpdatedAt => "updatedAt".to_owned(),
            MediaListField::CreatedAt => "createdAt".to_owned(),
            MediaListField::Media(fields) => format!("media {{ {} }}", fields.join_fields()),
        }
    }
}
