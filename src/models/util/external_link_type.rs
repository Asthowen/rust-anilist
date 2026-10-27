use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ExternalLinkType {
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "STREAMING")]
    Streaming,
    #[serde(rename = "SOCIAL")]
    Social,
}
