// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Notification {}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NotificationOption {
    notification_type: Type,
    enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
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
