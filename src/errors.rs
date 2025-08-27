use crate::client::AniListResponseError;
use reqwest::StatusCode;
use std::fmt;

#[derive(Debug)]
pub enum AniListError {
    MissingBuilderField(&'static str),
    UnknownQuery,
    MixedOperationTypes,
    ApiErrors(Vec<AniListResponseError>),
    ApiRateLimited,
    UnknownApiError(StatusCode),
    HttpRequestError(reqwest::Error),
    JsonParseError(serde_json::Error),
    InvalidHttpHeader(reqwest::header::InvalidHeaderValue),
}

impl From<reqwest::Error> for AniListError {
    fn from(error: reqwest::Error) -> Self {
        Self::HttpRequestError(error)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for AniListError {
    fn from(error: reqwest::header::InvalidHeaderValue) -> Self {
        Self::InvalidHttpHeader(error)
    }
}

impl fmt::Display for AniListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingBuilderField(field) => write!(f, "Missing required field: {field}"),
            Self::UnknownQuery => write!(
                f,
                "The query type entered does not exist or has not yet been implemented."
            ),
            Self::MixedOperationTypes => {
                write!(f, "You cannot mix root operation types in a single query.")
            }
            Self::ApiErrors(errors) => {
                write!(
                    f,
                    "AniList API returned {} error{}: {errors:?}",
                    errors.len(),
                    if errors.len() < 2 { "" } else { "s" }
                )
            }
            Self::ApiRateLimited => write!(
                f,
                "AniList API rate limit has been reached. Please wait before making more requests."
            ),
            Self::UnknownApiError(code) => write!(
                f,
                "An unknown AniList API error occurred (HTTP error code: {code})."
            ),
            Self::HttpRequestError(error) => write!(f, "HTTP request failed: {error}"),
            Self::JsonParseError(error) => write!(f, "JSON parsing failed: {error}"),
            Self::InvalidHttpHeader(error) => write!(f, "Invalid HTTP header value: {error}"),
        }
    }
}
