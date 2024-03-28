// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Other(String),
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Other(String::from("Neutral"))
    }
}
