use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaRelation {
    #[serde(rename = "ADAPTATION")]
    Adaptation,
    #[serde(rename = "PREQUEL")]
    Prequel,
    #[serde(rename = "SEQUEL")]
    Sequel,
    #[serde(rename = "PARENT")]
    Parent,
    #[serde(rename = "SIDE_STORY")]
    SideStory,
    #[serde(rename = "CHARACTER")]
    Character,
    #[serde(rename = "SUMMARY")]
    Summary,
    #[serde(rename = "ALTERNATIVE")]
    Alternative,
    #[serde(rename = "SPIN_OFF")]
    SpinOff,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "SOURCE")]
    Source,
    #[serde(rename = "COMPILATION")]
    Compilation,
    #[serde(rename = "CONTAINS")]
    Contains,
}
