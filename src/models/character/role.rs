use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CharacterRole {
    #[serde(rename = "MAIN")]
    Main,
    #[serde(rename = "SUPPORTING")]
    Supporting,
    #[serde(rename = "BACKGROUND")]
    Background,
}
