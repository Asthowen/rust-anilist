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
