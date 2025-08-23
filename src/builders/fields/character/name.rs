#[derive(Copy, Clone)]
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

impl From<CharacterNameField> for &'static str {
    fn from(value: CharacterNameField) -> Self {
        match value {
            CharacterNameField::First => "first",
            CharacterNameField::Middle => "middle",
            CharacterNameField::Last => "last",
            CharacterNameField::Full => "full",
            CharacterNameField::Native => "native",
            CharacterNameField::Alternative => "alternative",
            CharacterNameField::UserPreferred => "user_preferred",
        }
    }
}
