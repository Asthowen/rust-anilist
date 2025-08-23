#[derive(Copy, Clone)]
pub enum CharacterImageField {
    Large,
    Medium,
}

impl CharacterImageField {
    pub const fn all() -> &'static [CharacterImageField] {
        &[CharacterImageField::Large, CharacterImageField::Medium]
    }
}

impl From<CharacterImageField> for String {
    fn from(value: CharacterImageField) -> Self {
        match value {
            CharacterImageField::Large => "large".to_owned(),
            CharacterImageField::Medium => "medium".to_owned(),
        }
    }
}
