use crate::builders::fields::{JoinFields, StaffField};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StaffEdgeField<'a> {
    Node(&'a [StaffField<'a>]),
    Id,
    Role,
    FavouriteOrder,
}

impl<'a> StaffEdgeField<'a> {
    pub const fn all() -> &'static [StaffEdgeField<'a>] {
        const ALL: &[StaffEdgeField<'static>] = &[
            StaffEdgeField::Node(&[StaffField::Id]),
            StaffEdgeField::Id,
            StaffEdgeField::Role,
            StaffEdgeField::FavouriteOrder,
        ];
        ALL
    }
}

impl From<StaffEdgeField<'_>> for String {
    fn from(value: StaffEdgeField<'_>) -> Self {
        match value {
            StaffEdgeField::Node(fields) => format!("node {{ {} }}", fields.join_fields()),
            StaffEdgeField::Id => "id".to_owned(),
            StaffEdgeField::Role => "role".to_owned(),
            StaffEdgeField::FavouriteOrder => "favouriteOrder".to_owned(),
        }
    }
}
