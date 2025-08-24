use crate::builders::fields::{JoinFields, PageInfoField, StaffEdgeField};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StaffConnectionField<'a> {
    Edges(&'a [StaffEdgeField]),
    PageInfo(&'a [PageInfoField]),
}

impl StaffConnectionField<'_> {
    pub const fn all() -> &'static [StaffConnectionField<'static>] {
        const ALL: &[StaffConnectionField<'static>] = &[
            StaffConnectionField::Edges(StaffEdgeField::all()),
            StaffConnectionField::PageInfo(PageInfoField::all()),
        ];
        ALL
    }
}

impl From<StaffConnectionField<'_>> for String {
    fn from(value: StaffConnectionField) -> Self {
        match value {
            StaffConnectionField::Edges(fields) => format!("edges {{ {} }}", fields.join_fields()),
            StaffConnectionField::PageInfo(fields) => {
                format!("pageInfo {{ {} }}", fields.join_fields())
            }
        }
    }
}
