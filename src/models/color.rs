// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Color {
    Blue,
    #[default]
    Purple,
    Pink,
    Orange,
    Red,
    Green,
    Gray,
    Hex(String),
}
