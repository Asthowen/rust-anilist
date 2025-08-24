use reqwest::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Request<'a> {
    pub query: &'a str,
    pub variables: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AniListResponseErrorLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AniListResponseError {
    pub message: String,
    #[serde(deserialize_with = "deserialize_status_code")]
    pub status: StatusCode,
    pub locations: Option<Vec<AniListResponseErrorLocation>>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AniListResponseInternal<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<AniListResponseError>>,
}

pub fn deserialize_status_code<'de, D>(d: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(StatusCode::from_u16(u16::deserialize(d)?).unwrap_or(StatusCode::NOT_FOUND))
}
