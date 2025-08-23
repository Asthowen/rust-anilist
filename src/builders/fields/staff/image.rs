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

impl From<StaffImageField> for &'static str {
    fn from(value: StaffImageField) -> Self {
        match value {
            StaffImageField::Large => "large",
            StaffImageField::Medium => "medium",
        }
    }
}
