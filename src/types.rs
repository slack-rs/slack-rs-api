//=============================================================================
//
//                    WARNING: This file is AUTO-GENERATED
//
// Do not make changes directly to this file.
//
// If you would like to make a change to the library, please update the schema
// definitions at https://github.com/slack-rs/slack-api-schemas
//
// If you would like to make a change how the library was generated,
// please edit https://github.com/slack-rs/slack-rs-api/tree/master/codegen
//
//=============================================================================

#![allow(clippy::large_enum_variant)]

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Bot {
    pub app_id: Option<String>,
    pub deleted: Option<bool>,
    pub icons: Option<BotIcons>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BotIcons {
    pub image_36: Option<String>,
    pub image_48: Option<String>,
    pub image_72: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Channel {
    pub accepted_user: Option<String>,
    pub created: Option<i32>,
    pub creator: Option<String>,
    pub id: Option<String>,
    pub is_archived: Option<bool>,
    pub is_channel: Option<bool>,
    pub is_general: Option<bool>,
    pub is_member: Option<bool>,
    pub is_moved: Option<i32>,
    pub is_mpim: Option<bool>,
    pub is_org_shared: Option<bool>,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: Option<bool>,
    pub is_read_only: Option<bool>,
    pub is_shared: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<crate::Message>,
    pub members: Option<Vec<String>>,
    pub name: Option<String>,
    pub name_normalized: Option<String>,
    pub num_members: Option<i32>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f32>,
    pub purpose: Option<ChannelPurpose>,
    pub topic: Option<ChannelTopic>,
    pub unlinked: Option<i32>,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChannelPurpose {
    pub creator: Option<String>,
    pub last_set: Option<i32>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChannelTopic {
    pub creator: Option<String>,
    pub last_set: Option<i32>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct File {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<i32>,
    pub created: Option<i32>,
    pub display_as_bot: Option<bool>,
    pub edit_link: Option<String>,
    pub editable: Option<bool>,
    pub external_type: Option<String>,
    pub filetype: Option<String>,
    pub groups: Option<Vec<String>>,
    pub id: Option<String>,
    pub ims: Option<Vec<String>>,
    pub initial_comment: Option<crate::FileComment>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub is_starred: Option<bool>,
    pub lines: Option<i32>,
    pub lines_more: Option<i32>,
    pub mimetype: Option<String>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub num_stars: Option<i32>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub pinned_to: Option<Vec<String>>,
    pub pretty_type: Option<String>,
    pub preview: Option<String>,
    pub preview_highlight: Option<String>,
    pub public_url_shared: Option<bool>,
    pub reactions: Option<Vec<crate::Reaction>>,
    pub size: Option<i32>,
    pub thumb_160: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_gif: Option<String>,
    pub thumb_360_h: Option<i32>,
    pub thumb_360_w: Option<i32>,
    pub thumb_480: Option<String>,
    pub thumb_480_h: Option<i32>,
    pub thumb_480_w: Option<i32>,
    pub thumb_64: Option<String>,
    pub thumb_80: Option<String>,
    pub timestamp: Option<crate::Timestamp>,
    pub title: Option<String>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub user: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileComment {
    pub comment: Option<String>,
    pub id: Option<String>,
    pub reactions: Option<Vec<crate::Reaction>>,
    pub timestamp: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Group {
    pub created: Option<i32>,
    pub creator: Option<String>,
    pub id: Option<String>,
    pub is_archived: Option<bool>,
    pub is_group: Option<bool>,
    pub is_mpim: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<crate::Message>,
    pub members: Option<Vec<String>>,
    pub name: Option<String>,
    pub purpose: Option<GroupPurpose>,
    pub topic: Option<GroupTopic>,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GroupPurpose {
    pub creator: Option<String>,
    pub last_set: Option<i32>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GroupTopic {
    pub creator: Option<String>,
    pub last_set: Option<i32>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Im {
    pub created: Option<i32>,
    pub id: Option<String>,
    pub is_im: Option<bool>,
    pub is_user_deleted: Option<bool>,
    pub user: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Standard(MessageStandard),
    BotAdd(MessageBotAdd),
    BotDisable(MessageBotDisable),
    BotEnable(MessageBotEnable),
    BotRemove(MessageBotRemove),
    BotMessage(MessageBotMessage),
    ChannelArchive(MessageChannelArchive),
    ChannelJoin(MessageChannelJoin),
    ChannelLeave(MessageChannelLeave),
    ChannelName(MessageChannelName),
    ChannelPurpose(MessageChannelPurpose),
    ChannelTopic(MessageChannelTopic),
    ChannelUnarchive(MessageChannelUnarchive),
    FileComment(MessageFileComment),
    FileMention(MessageFileMention),
    FileShare(MessageFileShare),
    GroupArchive(MessageGroupArchive),
    GroupJoin(MessageGroupJoin),
    GroupLeave(MessageGroupLeave),
    GroupName(MessageGroupName),
    GroupPurpose(MessageGroupPurpose),
    GroupTopic(MessageGroupTopic),
    GroupUnarchive(MessageGroupUnarchive),
    MeMessage(MessageMeMessage),
    MessageChanged(MessageMessageChanged),
    MessageDeleted(MessageMessageDeleted),
    MessageReplied(MessageMessageReplied),
    PinnedItem(MessagePinnedItem),
    ReminderAdd(MessageReminderAdd),
    ReplyBroadcast(MessageReplyBroadcast),
    ThreadBroadcast(MessageThreadBroadcast),
    UnpinnedItem(MessageUnpinnedItem),
    ShRoomCreated(MessageShRoomCreated),
    SlackbotResponse(MessageSlackbotResponse),
}

impl<'de> ::serde::Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        use ::serde::de::Error as SerdeError;

        const VARIANTS: &[&str] = &[
            "standard",
            "bot_add",
            "bot_disable",
            "bot_enable",
            "bot_remove",
            "bot_message",
            "channel_archive",
            "channel_join",
            "channel_leave",
            "channel_name",
            "channel_purpose",
            "channel_topic",
            "channel_unarchive",
            "file_comment",
            "file_mention",
            "file_share",
            "group_archive",
            "group_join",
            "group_leave",
            "group_name",
            "group_purpose",
            "group_topic",
            "group_unarchive",
            "me_message",
            "message_changed",
            "message_deleted",
            "message_replied",
            "pinned_item",
            "reminder_add",
            "reply_broadcast",
            "thread_broadcast",
            "unpinned_item",
            "sh_room_created",
            "slackbot_response",
        ];

        let value = ::serde_json::Value::deserialize(deserializer)?;
        if let Some(ty_val) = value.get("subtype") {
            if let Some(ty) = ty_val.as_str() {
                match ty {
                    "standard" => ::serde_json::from_value::<MessageStandard>(value.clone())
                        .map(Message::Standard)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "bot_add" => ::serde_json::from_value::<MessageBotAdd>(value.clone())
                        .map(Message::BotAdd)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "bot_disable" => ::serde_json::from_value::<MessageBotDisable>(value.clone())
                        .map(Message::BotDisable)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "bot_enable" => ::serde_json::from_value::<MessageBotEnable>(value.clone())
                        .map(Message::BotEnable)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "bot_remove" => ::serde_json::from_value::<MessageBotRemove>(value.clone())
                        .map(Message::BotRemove)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "bot_message" => ::serde_json::from_value::<MessageBotMessage>(value.clone())
                        .map(Message::BotMessage)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "channel_archive" => {
                        ::serde_json::from_value::<MessageChannelArchive>(value.clone())
                            .map(Message::ChannelArchive)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "channel_join" => ::serde_json::from_value::<MessageChannelJoin>(value.clone())
                        .map(Message::ChannelJoin)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "channel_leave" => {
                        ::serde_json::from_value::<MessageChannelLeave>(value.clone())
                            .map(Message::ChannelLeave)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "channel_name" => ::serde_json::from_value::<MessageChannelName>(value.clone())
                        .map(Message::ChannelName)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "channel_purpose" => {
                        ::serde_json::from_value::<MessageChannelPurpose>(value.clone())
                            .map(Message::ChannelPurpose)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "channel_topic" => {
                        ::serde_json::from_value::<MessageChannelTopic>(value.clone())
                            .map(Message::ChannelTopic)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "channel_unarchive" => {
                        ::serde_json::from_value::<MessageChannelUnarchive>(value.clone())
                            .map(Message::ChannelUnarchive)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "file_comment" => ::serde_json::from_value::<MessageFileComment>(value.clone())
                        .map(Message::FileComment)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "file_mention" => ::serde_json::from_value::<MessageFileMention>(value.clone())
                        .map(Message::FileMention)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "file_share" => ::serde_json::from_value::<MessageFileShare>(value.clone())
                        .map(Message::FileShare)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "group_archive" => {
                        ::serde_json::from_value::<MessageGroupArchive>(value.clone())
                            .map(Message::GroupArchive)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "group_join" => ::serde_json::from_value::<MessageGroupJoin>(value.clone())
                        .map(Message::GroupJoin)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "group_leave" => ::serde_json::from_value::<MessageGroupLeave>(value.clone())
                        .map(Message::GroupLeave)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "group_name" => ::serde_json::from_value::<MessageGroupName>(value.clone())
                        .map(Message::GroupName)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "group_purpose" => {
                        ::serde_json::from_value::<MessageGroupPurpose>(value.clone())
                            .map(Message::GroupPurpose)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "group_topic" => ::serde_json::from_value::<MessageGroupTopic>(value.clone())
                        .map(Message::GroupTopic)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "group_unarchive" => {
                        ::serde_json::from_value::<MessageGroupUnarchive>(value.clone())
                            .map(Message::GroupUnarchive)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "me_message" => ::serde_json::from_value::<MessageMeMessage>(value.clone())
                        .map(Message::MeMessage)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "message_changed" => {
                        ::serde_json::from_value::<MessageMessageChanged>(value.clone())
                            .map(Message::MessageChanged)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "message_deleted" => {
                        ::serde_json::from_value::<MessageMessageDeleted>(value.clone())
                            .map(Message::MessageDeleted)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "message_replied" => {
                        ::serde_json::from_value::<MessageMessageReplied>(value.clone())
                            .map(Message::MessageReplied)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "pinned_item" => ::serde_json::from_value::<MessagePinnedItem>(value.clone())
                        .map(Message::PinnedItem)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "reminder_add" => ::serde_json::from_value::<MessageReminderAdd>(value.clone())
                        .map(Message::ReminderAdd)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "reply_broadcast" => {
                        ::serde_json::from_value::<MessageReplyBroadcast>(value.clone())
                            .map(Message::ReplyBroadcast)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "thread_broadcast" => {
                        ::serde_json::from_value::<MessageThreadBroadcast>(value.clone())
                            .map(Message::ThreadBroadcast)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "unpinned_item" => {
                        ::serde_json::from_value::<MessageUnpinnedItem>(value.clone())
                            .map(Message::UnpinnedItem)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "sh_room_created" => {
                        ::serde_json::from_value::<MessageShRoomCreated>(value.clone())
                            .map(Message::ShRoomCreated)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "slackbot_response" => {
                        ::serde_json::from_value::<MessageSlackbotResponse>(value.clone())
                            .map(Message::SlackbotResponse)
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    _ => Err(D::Error::unknown_variant(ty, VARIANTS)),
                }
            } else {
                Err(D::Error::invalid_type(
                    ::serde::de::Unexpected::Unit,
                    &"a string",
                ))
            }
        } else {
            ::serde_json::from_value::<MessageStandard>(value.clone())
                .map(Message::Standard)
                .map_err(|e| D::Error::custom(&format!("{}", e)))
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotAdd {
    pub bot_id: Option<String>,
    pub bot_link: Option<String>,
    pub channel: Option<String>,
    pub subtype: Option<String>,
    pub team: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotDisable {
    pub bot_id: Option<String>,
    pub bot_link: Option<String>,
    pub channel: Option<String>,
    pub subtype: Option<String>,
    pub team: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotEnable {
    pub bot_id: Option<String>,
    pub bot_link: Option<String>,
    pub channel: Option<String>,
    pub subtype: Option<String>,
    pub team: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotMessage {
    pub attachments: Option<Vec<MessageBotMessageAttachment>>,
    pub bot_id: Option<String>,
    pub channel: Option<String>,
    #[serde(deserialize_with = "crate::optional_struct_or_empty_array")]
    #[serde(default)]
    pub icons: Option<MessageBotMessageIcons>,
    pub subtype: Option<String>,
    pub team: Option<String>,
    pub text: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotMessageAttachment {
    pub author_icon: Option<String>,
    pub author_link: Option<String>,
    pub author_name: Option<String>,
    pub color: Option<String>,
    pub fallback: Option<String>,
    pub fields: Option<Vec<MessageBotMessageAttachmentField>>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub image_url: Option<String>,
    pub pretext: Option<String>,
    pub text: Option<String>,
    pub thumb_url: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub ts: Option<crate::Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotMessageAttachmentField {
    pub short: Option<bool>,
    pub title: Option<String>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotMessageIcons {
    pub emoji: Option<String>,
    pub image_36: Option<String>,
    pub image_48: Option<String>,
    pub image_64: Option<String>,
    pub image_72: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotRemove {
    pub bot_id: Option<String>,
    pub bot_link: Option<String>,
    pub channel: Option<String>,
    pub subtype: Option<String>,
    pub team: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelArchive {
    pub members: Option<Vec<String>>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelJoin {
    pub inviter: Option<String>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelLeave {
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelName {
    pub name: Option<String>,
    pub old_name: Option<String>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelPurpose {
    pub purpose: Option<String>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelTopic {
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub topic: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelUnarchive {
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageFileComment {
    pub comment: Option<crate::FileComment>,
    pub file: Option<crate::File>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageFileMention {
    pub file: Option<crate::File>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageFileShare {
    pub file: Option<crate::File>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub upload: Option<bool>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupArchive {
    pub members: Option<Vec<String>>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupJoin {
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupLeave {
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupName {
    pub name: Option<String>,
    pub old_name: Option<String>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupPurpose {
    pub purpose: Option<String>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupTopic {
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub topic: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupUnarchive {
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMeMessage {
    pub channel: Option<String>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChanged {
    pub channel: Option<String>,
    pub event_ts: Option<crate::Timestamp>,
    pub hidden: Option<bool>,
    pub message: Option<MessageMessageChangedMessage>,
    pub previous_message: Option<MessageMessageChangedPreviousMessage>,
    pub subtype: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedMessage {
    pub bot_id: Option<String>,
    pub edited: Option<MessageMessageChangedMessageEdited>,
    pub last_read: Option<String>,
    pub parent_user_id: Option<String>,
    pub replies: Option<Vec<MessageMessageChangedMessageReply>>,
    pub reply_count: Option<i32>,
    pub subscribed: Option<bool>,
    pub text: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub unread_count: Option<i32>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedMessageEdited {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedMessageReply {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedPreviousMessage {
    pub bot_id: Option<String>,
    pub edited: Option<MessageMessageChangedPreviousMessageEdited>,
    pub last_read: Option<String>,
    pub parent_user_id: Option<String>,
    pub replies: Option<Vec<MessageMessageChangedPreviousMessageReply>>,
    pub reply_count: Option<i32>,
    pub subscribed: Option<bool>,
    pub text: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub unread_count: Option<i32>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedPreviousMessageEdited {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedPreviousMessageReply {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageDeleted {
    pub channel: Option<String>,
    pub deleted_ts: Option<crate::Timestamp>,
    pub event_ts: Option<crate::Timestamp>,
    pub hidden: Option<bool>,
    pub previous_message: Option<MessageMessageDeletedPreviousMessage>,
    pub subtype: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageDeletedPreviousMessage {
    pub bot_id: Option<String>,
    pub edited: Option<MessageMessageDeletedPreviousMessageEdited>,
    pub last_read: Option<String>,
    pub parent_user_id: Option<String>,
    pub replies: Option<Vec<MessageMessageDeletedPreviousMessageReply>>,
    pub reply_count: Option<i32>,
    pub subscribed: Option<bool>,
    pub text: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub unread_count: Option<i32>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageDeletedPreviousMessageEdited {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageDeletedPreviousMessageReply {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageReplied {
    pub channel: Option<String>,
    pub event_ts: Option<crate::Timestamp>,
    pub hidden: Option<bool>,
    pub message: Option<MessageMessageRepliedMessage>,
    pub subtype: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageRepliedMessage {
    pub bot_id: Option<String>,
    pub edited: Option<MessageMessageRepliedMessageEdited>,
    pub last_read: Option<String>,
    pub parent_user_id: Option<String>,
    pub replies: Option<Vec<MessageMessageRepliedMessageReply>>,
    pub reply_count: Option<i32>,
    pub subscribed: Option<bool>,
    pub text: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub unread_count: Option<i32>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageRepliedMessageEdited {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageRepliedMessageReply {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessagePinnedItem {
    pub channel: Option<String>,
    pub item: Option<MessagePinnedItemItem>,
    pub item_type: Option<String>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessagePinnedItemItem {}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageReminderAdd {
    pub channel: Option<String>,
    pub subtype: Option<String>,
    pub team: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageReplyBroadcast {
    pub attachments: Option<Vec<MessageReplyBroadcastAttachment>>,
    pub channel: Option<String>,
    pub event_ts: Option<crate::Timestamp>,
    pub subtype: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageReplyBroadcastAttachment {
    pub author_icon: Option<String>,
    pub author_link: Option<String>,
    pub author_subname: Option<String>,
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
    pub fallback: Option<String>,
    pub footer: Option<String>,
    pub from_url: Option<String>,
    pub id: Option<i32>,
    pub mrkdwn_in: Option<Vec<String>>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageShRoomCreated {
    pub channel: Option<String>,
    pub no_notifications: Option<bool>,
    pub permalink: Option<String>,
    pub root: Option<MessageShRoomCreatedRoot>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageShRoomCreatedRoot {
    pub app_id: Option<String>,
    pub channels: Option<Vec<String>>,
    pub created_by: Option<String>,
    pub date_end: Option<crate::Timestamp>,
    pub date_start: Option<crate::Timestamp>,
    pub display_id: Option<String>,
    pub external_unique_id: Option<String>,
    pub has_ended: Option<bool>,
    pub id: Option<String>,
    pub is_dm_call: Option<bool>,
    pub media_server: Option<String>,
    pub name: Option<String>,
    pub participant_history: Option<Vec<String>>,
    pub participants: Option<Vec<String>>,
    pub participants_camera_off: Option<Vec<String>>,
    pub participants_camera_on: Option<Vec<String>>,
    pub participants_screenshare_off: Option<Vec<String>>,
    pub participants_screenshare_on: Option<Vec<String>>,
    pub was_accepted: Option<bool>,
    pub was_missed: Option<bool>,
    pub was_rejected: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageSlackbotResponse {
    pub channel: Option<String>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageStandard {
    pub attachments: Option<Vec<MessageStandardAttachment>>,
    pub bot_id: Option<String>,
    pub channel: Option<String>,
    pub edited: Option<MessageStandardEdited>,
    pub event_ts: Option<crate::Timestamp>,
    pub reply_broadcast: Option<bool>,
    pub source_team: Option<String>,
    pub team: Option<String>,
    pub text: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageStandardAttachment {
    pub author_icon: Option<String>,
    pub author_link: Option<String>,
    pub author_name: Option<String>,
    pub color: Option<String>,
    pub fallback: Option<String>,
    pub fields: Option<Vec<MessageStandardAttachmentField>>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub image_url: Option<String>,
    pub pretext: Option<String>,
    pub text: Option<String>,
    pub thumb_url: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub ts: Option<crate::Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageStandardAttachmentField {
    pub short: Option<bool>,
    pub title: Option<String>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageStandardEdited {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageThreadBroadcast {
    pub root: Option<MessageThreadBroadcastRoot>,
    pub subtype: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageThreadBroadcastRoot {
    pub latest_reply: Option<crate::Timestamp>,
    pub replies: Option<Vec<MessageThreadBroadcastRootReply>>,
    pub reply_count: Option<i32>,
    pub reply_users: Option<Vec<String>>,
    pub reply_users_count: Option<i32>,
    pub subscriped: Option<bool>,
    pub text: Option<String>,
    pub thread_ts: Option<crate::Timestamp>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageThreadBroadcastRootReply {
    pub ts: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageUnpinnedItem {
    pub channel: Option<String>,
    pub item: Option<MessageUnpinnedItemItem>,
    pub item_type: Option<String>,
    pub subtype: Option<String>,
    pub text: Option<String>,
    pub ts: Option<crate::Timestamp>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageUnpinnedItemItem {}

#[derive(Clone, Debug, Deserialize)]
pub struct Mpim {
    pub created: Option<i32>,
    pub creator: Option<String>,
    pub id: Option<String>,
    pub is_group: Option<bool>,
    pub is_mpim: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<crate::Message>,
    pub members: Option<Vec<String>>,
    pub name: Option<String>,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Paging {
    pub count: Option<i32>,
    pub page: Option<i32>,
    pub pages: Option<i32>,
    pub total: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reaction {
    pub count: Option<i32>,
    pub name: Option<String>,
    pub users: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reminder {
    pub complete_ts: Option<crate::Timestamp>,
    pub creator: Option<String>,
    pub id: Option<String>,
    pub recurring: Option<bool>,
    pub text: Option<String>,
    pub time: Option<crate::Timestamp>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Team {
    pub domain: Option<String>,
    pub email_domain: Option<String>,
    pub icon: Option<TeamIcon>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeamIcon {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ThreadInfo {
    pub complete: Option<bool>,
    pub count: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub color: Option<String>,
    pub deleted: Option<bool>,
    pub has_2fa: Option<bool>,
    pub id: Option<String>,
    pub is_admin: Option<bool>,
    pub is_app_user: Option<bool>,
    pub is_bot: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub profile: Option<crate::UserProfile>,
    pub real_name: Option<String>,
    pub team_id: Option<String>,
    pub two_factor_type: Option<String>,
    pub tz: Option<String>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<f32>,
    pub updated: Option<f32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Usergroup {
    pub auto_type: Option<String>,
    pub created_by: Option<String>,
    pub date_create: Option<i32>,
    pub date_delete: Option<i32>,
    pub date_update: Option<i32>,
    pub deleted_by: Option<String>,
    pub description: Option<String>,
    pub handle: Option<String>,
    pub id: Option<String>,
    pub is_external: Option<bool>,
    pub is_usergroup: Option<bool>,
    pub name: Option<String>,
    pub prefs: Option<UsergroupPrefs>,
    pub team_id: Option<String>,
    pub updated_by: Option<String>,
    pub user_count: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UsergroupPrefs {
    pub channels: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserProfile {
    pub avatar_hash: Option<String>,
    pub display_name: Option<String>,
    pub display_name_normalized: Option<String>,
    pub email: Option<String>,
    #[serde(deserialize_with = "crate::optional_struct_or_empty_array")]
    #[serde(default)]
    pub fields: Option<HashMap<String, UserProfileFields>>,
    pub first_name: Option<String>,
    pub guest_channels: Option<String>,
    pub image_192: Option<String>,
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_512: Option<String>,
    pub image_72: Option<String>,
    pub image_original: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub real_name: Option<String>,
    pub real_name_normalized: Option<String>,
    pub skype: Option<String>,
    pub status_emoji: Option<String>,
    pub status_text: Option<String>,
    pub team: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserProfileFields {
    pub alt: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>,
}
