#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CharacterNameField {
    First,
    Middle,
    Last,
    Full,
    Native,
    Alternative,
    UserPreferred,
}

impl CharacterNameField {
    pub const fn all() -> &'static [CharacterNameField] {
        &[
            CharacterNameField::First,
            CharacterNameField::Middle,
            CharacterNameField::Last,
            CharacterNameField::Full,
            CharacterNameField::Native,
            CharacterNameField::Alternative,
            CharacterNameField::UserPreferred,
        ]
    }
}

impl From<CharacterNameField> for String {
    fn from(value: CharacterNameField) -> Self {
        match value {
            CharacterNameField::First => "first".to_owned(),
            CharacterNameField::Middle => "middle".to_owned(),
            CharacterNameField::Last => "last".to_owned(),
            CharacterNameField::Full => "full".to_owned(),
            CharacterNameField::Native => "native".to_owned(),
            CharacterNameField::Alternative => "alternative".to_owned(),
            CharacterNameField::UserPreferred => "user_preferred".to_owned(),
        }
    }
}
