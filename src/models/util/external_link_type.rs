use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExternalLinkType {
    #[serde(alias = "INFO")]
    Info,
    #[serde(alias = "STREAMING")]
    Streaming,
    #[serde(alias = "SOCIAL")]
    Social,
}
