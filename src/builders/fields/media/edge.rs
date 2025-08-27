use crate::builders::fields::{
    CharacterField, JoinFields, MediaField, StaffField, StaffRoleTypeField,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MediaEdgeField<'a> {
    Node(&'a [MediaField<'a>]),
    Id,
    RelationType,
    IsMainStudio,
    Characters(&'a [CharacterField<'a>]),
    CharacterRole,
    CharacterName,
    RoleNotes,
    DubGroup,
    StaffRole,
    VoiceActors(&'a [StaffField<'a>]),
    VoiceActorRoles(&'a [StaffRoleTypeField<'a>]),
    FavouriteOrder,
}

impl MediaEdgeField<'_> {
    pub const fn all() -> &'static [MediaEdgeField<'static>] {
        const ALL: &[MediaEdgeField<'static>] = &[
            MediaEdgeField::Node(&[MediaField::Id]),
            MediaEdgeField::Id,
            MediaEdgeField::RelationType,
            MediaEdgeField::IsMainStudio,
            MediaEdgeField::Characters(&[CharacterField::Id]),
            MediaEdgeField::CharacterRole,
            MediaEdgeField::CharacterName,
            MediaEdgeField::RoleNotes,
            MediaEdgeField::DubGroup,
            MediaEdgeField::StaffRole,
            MediaEdgeField::FavouriteOrder,
        ];

        ALL
    }
}

impl From<MediaEdgeField<'_>> for String {
    fn from(value: MediaEdgeField) -> Self {
        match value {
            MediaEdgeField::Node(fields) => format!("node {{ {} }}", fields.join_fields()),
            MediaEdgeField::Id => "id".to_owned(),
            MediaEdgeField::RelationType => "relationType".to_owned(),
            MediaEdgeField::IsMainStudio => "isMainStudio".to_owned(),
            MediaEdgeField::Characters(fields) => {
                format!("characters {{ {} }}", fields.join_fields())
            }
            MediaEdgeField::CharacterRole => "characterRole".to_owned(),
            MediaEdgeField::CharacterName => "characterName".to_owned(),
            MediaEdgeField::RoleNotes => "roleNotes".to_owned(),
            MediaEdgeField::DubGroup => "dubGroup".to_owned(),
            MediaEdgeField::StaffRole => "staffRole".to_owned(),
            MediaEdgeField::VoiceActors(fields) => {
                format!("voiceActors {{ {} }}", fields.join_fields())
            }
            MediaEdgeField::VoiceActorRoles(fields) => {
                format!("voiceActorRoles {{ {} }}", fields.join_fields())
            }
            MediaEdgeField::FavouriteOrder => "favouriteOrder".to_owned(),
        }
    }
}
