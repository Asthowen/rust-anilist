use crate::builders::fields::{
    CharacterField, JoinFields, MediaField, StaffField, StaffRoleTypeField,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CharacterEdgeField<'a> {
    Node(&'a [CharacterField<'a>]),
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
            CharacterEdgeField::Node(&[CharacterField::Id]),
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
            CharacterEdgeField::Node(fields) => {
                format!("node {{ {} }}", fields.join_fields())
            }
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
