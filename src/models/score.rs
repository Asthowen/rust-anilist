// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub average: i64,
    pub mean: i64,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Format {
    Point100,
    #[default]
    Point10Decimal,
    Point10,
    Point5,
    Point3,
}
