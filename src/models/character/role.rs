use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum CharacterRole {
    #[serde(rename = "MAIN")]
    Main,
    #[serde(rename = "SUPPORTING")]
    Supporting,
    #[serde(rename = "BACKGROUND")]
    Background,
}
