use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CharacterImage {
    pub large: Option<String>,
    pub medium: Option<String>,
}
