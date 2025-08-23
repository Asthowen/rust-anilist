#[derive(Copy, Clone)]
pub enum StaffImageField {
    Large,
    Medium,
}

impl StaffImageField {
    pub const fn all() -> &'static [StaffImageField] {
        &[StaffImageField::Large, StaffImageField::Medium]
    }
}

impl From<StaffImageField> for String {
    fn from(value: StaffImageField) -> Self {
        match value {
            StaffImageField::Large => "large".to_owned(),
            StaffImageField::Medium => "medium".to_owned(),
        }
    }
}
