use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExternalLinkType {
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "STREAMING")]
    Streaming,
    #[serde(rename = "SOCIAL")]
    Social,
}
