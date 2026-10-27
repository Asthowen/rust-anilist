use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct FuzzyDate {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

impl From<FuzzyDate> for Value {
    fn from(value: FuzzyDate) -> Self {
        serde_json::to_value(value).unwrap_or_default()
    }
}
