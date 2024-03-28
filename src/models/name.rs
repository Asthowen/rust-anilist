// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub first: String,
    pub middle: Option<String>,
    pub last: Option<String>,
    pub full: String,
    pub native: Option<String>,
    pub alternative: Vec<String>,
    pub alternative_spoiler: Vec<String>,
    pub user_preferred: Option<String>,
}
