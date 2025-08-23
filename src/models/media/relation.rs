use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaRelation {
    #[serde(alias = "ADAPTATION")]
    Adaptation,
    #[serde(alias = "PREQUEL")]
    Prequel,
    #[serde(alias = "SEQUEL")]
    Sequel,
    #[serde(alias = "PARENT")]
    Parent,
    #[serde(alias = "SIDE_STORY")]
    SideStory,
    #[serde(alias = "CHARACTER")]
    Character,
    #[serde(alias = "SUMMARY")]
    Summary,
    #[serde(alias = "ALTERNATIVE")]
    Alternative,
    #[serde(alias = "SPIN_OFF")]
    SpinOff,
    #[serde(alias = "OTHER")]
    Other,
    #[serde(alias = "SOURCE")]
    Source,
    #[serde(alias = "COMPILATION")]
    Compilation,
    #[serde(alias = "CONTAINS")]
    Contains,
}
