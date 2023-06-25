// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Status {
    Finished,
    Releasing,
    #[default]
    NotYetReleased,
    Cancelled,
    Hiatus,
    Current,
    Planning,
    Completed,
    Dropped,
    Paused,
    Repeating,
}
