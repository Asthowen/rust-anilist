#[derive(Copy, Clone)]
pub enum StaffNameField {
    First,
    Middle,
    Last,
    Full,
    Native,
    Alternative,
    UserPreferred,
}

impl StaffNameField {
    pub const fn all() -> &'static [StaffNameField] {
        &[
            StaffNameField::First,
            StaffNameField::Middle,
            StaffNameField::Last,
            StaffNameField::Full,
            StaffNameField::Native,
            StaffNameField::Alternative,
            StaffNameField::UserPreferred,
        ]
    }
}

impl From<StaffNameField> for &'static str {
    fn from(value: StaffNameField) -> Self {
        match value {
            StaffNameField::First => "first",
            StaffNameField::Middle => "middle",
            StaffNameField::Last => "last",
            StaffNameField::Full => "full",
            StaffNameField::Native => "native",
            StaffNameField::Alternative => "alternative",
            StaffNameField::UserPreferred => "userPreferred",
        }
    }
}
