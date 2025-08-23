use crate::builders::fields::{JoinFields, MediaEdgeField, PageInfoField};

#[derive(Copy, Clone)]
pub enum MediaConnectionField<'a> {
    Edges(&'a [MediaEdgeField<'a>]),
    PageInfo(&'a [PageInfoField]),
}

impl MediaConnectionField<'_> {
    pub const fn all() -> &'static [MediaConnectionField<'static>] {
        const ALL: &[MediaConnectionField<'static>] = &[
            MediaConnectionField::Edges(MediaEdgeField::all()),
            MediaConnectionField::PageInfo(PageInfoField::all()),
        ];
        ALL
    }
}

impl From<MediaConnectionField<'_>> for String {
    fn from(value: MediaConnectionField) -> Self {
        match value {
            MediaConnectionField::Edges(fields) => format!("edges {{ {} }}", fields.join_fields()),
            MediaConnectionField::PageInfo(fields) => {
                format!("pageInfo {{ {} }}", fields.join_fields())
            }
        }
    }
}
