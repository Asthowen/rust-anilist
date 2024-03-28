// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationOption {
    notification_type: Type,
    enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Type {
    #[default]
    ActivityMessage,
    ActivityReply,
    Following,
    ActivityMention,
    ThreadCommentMention,
    Airing,
    ActivityLike,
    ActivityReplyLike,
    ThreadLike,
    ActivityReplySubscribed,
    RelatedMediaAddition,
    MediaDataChange,
    MediaMerge,
    MediaDeletion,
}
