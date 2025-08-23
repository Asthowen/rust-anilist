use crate::builders::fields::{CharacterEdgeField, JoinFields, PageInfoField};

#[derive(Copy, Clone)]
pub enum CharacterConnectionField<'a> {
    Edges(&'a [CharacterEdgeField<'a>]),
    PageInfo(&'a [PageInfoField]),
}

impl CharacterConnectionField<'_> {
    pub const fn all() -> &'static [CharacterConnectionField<'static>] {
        const ALL: &[CharacterConnectionField<'static>] = &[
            CharacterConnectionField::Edges(CharacterEdgeField::all()),
            CharacterConnectionField::PageInfo(PageInfoField::all()),
        ];
        ALL
    }
}

impl From<CharacterConnectionField<'_>> for String {
    fn from(value: CharacterConnectionField) -> Self {
        match value {
            CharacterConnectionField::Edges(fields) => {
                format!("edges {{ {} }}", fields.join_fields())
            }
            CharacterConnectionField::PageInfo(fields) => {
                format!("pageInfo {{ {} }}", fields.join_fields())
            }
        }
    }
}
