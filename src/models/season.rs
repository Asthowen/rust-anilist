// SPDX-License-Identifier: MIT↴↴
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>↴↴

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
}
