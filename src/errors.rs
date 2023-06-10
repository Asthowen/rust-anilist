use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct GenericError(pub String);

impl From<reqwest::Error> for GenericError {
    fn from(error: reqwest::Error) -> Self {
        GenericError(format!("reqwest Error: {}", error))
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidId,
    InvalidMediaType,
}

#[derive(Debug)]
struct InvalidId;

impl StdError for InvalidId {}

impl fmt::Display for InvalidId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The given id is invalid")
    }
}

#[derive(Debug)]
struct InvalidMediaType;

impl StdError for InvalidMediaType {}

impl fmt::Display for InvalidMediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The given media type is invalid")
    }
}
