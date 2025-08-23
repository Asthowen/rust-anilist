use crate::builders::fields::{PageInfoField, StaffEdgeField};

#[derive(Copy, Clone)]
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
            StaffConnectionField::Edges(fields) => format!(
                "edges {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            StaffConnectionField::PageInfo(fields) => format!(
                "pageInfo {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
        }
    }
}
