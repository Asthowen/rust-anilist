use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CharacterRole {
    #[serde(alias = "MAIN")]
    Main,
    #[serde(alias = "SUPPORTING")]
    Supporting,
    #[serde(alias = "BACKGROUND")]
    Background,
}
