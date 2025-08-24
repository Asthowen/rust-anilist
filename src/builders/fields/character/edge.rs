use crate::builders::fields::{JoinFields, MediaField, StaffField, StaffRoleTypeField};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CharacterEdgeField<'a> {
    Id,
    Role,
    Name,
    VoiceActors(&'a [StaffField<'a>]),
    VoiceActorRoles(&'a [StaffRoleTypeField<'a>]),
    Media(&'a [MediaField<'a>]),
    FavouriteOrder,
}

impl CharacterEdgeField<'_> {
    pub const fn all() -> &'static [CharacterEdgeField<'static>] {
        const ALL: &[CharacterEdgeField<'static>] = &[
            CharacterEdgeField::Id,
            CharacterEdgeField::Role,
            CharacterEdgeField::Name,
            CharacterEdgeField::VoiceActors(StaffField::all()),
            CharacterEdgeField::VoiceActorRoles(StaffRoleTypeField::all()),
            CharacterEdgeField::Media(MediaField::all()),
            CharacterEdgeField::FavouriteOrder,
        ];
        ALL
    }

    pub const fn all_without_media() -> &'static [CharacterEdgeField<'static>] {
        const ALL: &[CharacterEdgeField<'static>] = &[
            CharacterEdgeField::Id,
            CharacterEdgeField::Role,
            CharacterEdgeField::Name,
            CharacterEdgeField::VoiceActors(StaffField::all()),
            CharacterEdgeField::VoiceActorRoles(StaffRoleTypeField::all()),
            CharacterEdgeField::FavouriteOrder,
        ];
        ALL
    }
}

impl From<CharacterEdgeField<'_>> for String {
    fn from(value: CharacterEdgeField) -> Self {
        match value {
            CharacterEdgeField::Id => "id".to_owned(),
            CharacterEdgeField::Role => "role".to_owned(),
            CharacterEdgeField::Name => "name".to_owned(),
            CharacterEdgeField::VoiceActors(fields) => {
                format!("voiceActors {{ {} }}", fields.join_fields())
            }
            CharacterEdgeField::VoiceActorRoles(fields) => {
                format!("voiceActorRoles {{ {} }}", fields.join_fields())
            }
            CharacterEdgeField::Media(fields) => format!("media {{ {} }}", fields.join_fields()),
            CharacterEdgeField::FavouriteOrder => "favouriteOrder".to_owned(),
        }
    }
}
