// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Clone, PartialEq, Default)]
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
