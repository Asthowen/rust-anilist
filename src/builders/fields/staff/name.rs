#[derive(Debug, Copy, Clone, PartialEq)]
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

impl From<StaffNameField> for String {
    fn from(value: StaffNameField) -> Self {
        match value {
            StaffNameField::First => "first".to_owned(),
            StaffNameField::Middle => "middle".to_owned(),
            StaffNameField::Last => "last".to_owned(),
            StaffNameField::Full => "full".to_owned(),
            StaffNameField::Native => "native".to_owned(),
            StaffNameField::Alternative => "alternative".to_owned(),
            StaffNameField::UserPreferred => "userPreferred".to_owned(),
        }
    }
}
