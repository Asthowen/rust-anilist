#[derive(Copy, Clone)]
pub enum StaffEdgeField {
    Id,
    Role,
    FavouriteOrder,
}

impl StaffEdgeField {
    pub const fn all() -> &'static [StaffEdgeField] {
        &[
            StaffEdgeField::Id,
            StaffEdgeField::Role,
            StaffEdgeField::FavouriteOrder,
        ]
    }
}

impl From<StaffEdgeField> for String {
    fn from(value: StaffEdgeField) -> Self {
        match value {
            StaffEdgeField::Id => "id".to_owned(),
            StaffEdgeField::Role => "role".to_owned(),
            StaffEdgeField::FavouriteOrder => "favouriteOrder".to_owned(),
        }
    }
}
