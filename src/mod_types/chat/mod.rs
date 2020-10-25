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

#![allow(unused_imports)]

pub mod scheduled_messages_types;

use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct MeMessageRequest {
    /// Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name.
    pub channel: Option<String>,
    /// Text of the message to send.
    pub text: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MeMessageResponse {
    pub callstack: Option<String>,
    pub channel: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub ts: Option<String>,
}

impl<E: Error> Into<Result<MeMessageResponse, MeMessageError<E>>> for MeMessageResponse {
    fn into(self) -> Result<MeMessageResponse, MeMessageError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum MeMessageError<E: Error> {
    ChannelNotFound,
    NotInChannel,
    IsArchived,
    MsgTooLong,
    NoText,
    RateLimited,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    FatalError,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for MeMessageError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => MeMessageError::ChannelNotFound,
            "not_in_channel" => MeMessageError::NotInChannel,
            "is_archived" => MeMessageError::IsArchived,
            "msg_too_long" => MeMessageError::MsgTooLong,
            "no_text" => MeMessageError::NoText,
            "rate_limited" => MeMessageError::RateLimited,
            "not_authed" => MeMessageError::NotAuthed,
            "invalid_auth" => MeMessageError::InvalidAuth,
            "account_inactive" => MeMessageError::AccountInactive,
            "token_revoked" => MeMessageError::TokenRevoked,
            "no_permission" => MeMessageError::NoPermission,
            "org_login_required" => MeMessageError::OrgLoginRequired,
            "invalid_arg_name" => MeMessageError::InvalidArgName,
            "invalid_array_arg" => MeMessageError::InvalidArrayArg,
            "invalid_charset" => MeMessageError::InvalidCharset,
            "invalid_form_data" => MeMessageError::InvalidFormData,
            "invalid_post_type" => MeMessageError::InvalidPostType,
            "missing_post_type" => MeMessageError::MissingPostType,
            "team_added_to_org" => MeMessageError::TeamAddedToOrg,
            "invalid_json" => MeMessageError::InvalidJson,
            "json_not_object" => MeMessageError::JsonNotObject,
            "request_timeout" => MeMessageError::RequestTimeout,
            "upgrade_required" => MeMessageError::UpgradeRequired,
            "fatal_error" => MeMessageError::FatalError,
            _ => MeMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MeMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MeMessageError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            MeMessageError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            MeMessageError::IsArchived => write!(f, "Server returned error is_archived"),
            MeMessageError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            MeMessageError::NoText => write!(f, "Server returned error no_text"),
            MeMessageError::RateLimited => write!(f, "Server returned error rate_limited"),
            MeMessageError::NotAuthed => write!(f, "Server returned error not_authed"),
            MeMessageError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            MeMessageError::AccountInactive => write!(f, "Server returned error account_inactive"),
            MeMessageError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            MeMessageError::NoPermission => write!(f, "Server returned error no_permission"),
            MeMessageError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            MeMessageError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            MeMessageError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            MeMessageError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            MeMessageError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            MeMessageError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            MeMessageError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            MeMessageError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            MeMessageError::InvalidJson => write!(f, "Server returned error invalid_json"),
            MeMessageError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            MeMessageError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            MeMessageError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            MeMessageError::FatalError => write!(f, "Server returned error fatal_error"),
            MeMessageError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            MeMessageError::Unknown(ref s) => write!(f, "{}", s),
            MeMessageError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for MeMessageError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            MeMessageError::MalformedResponse(_, ref e) => Some(e),
            MeMessageError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct PostMessageRequest {
    /// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [authorship](#authorship) below.
    pub as_user: Option<String>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    pub attachments: Option<String>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<String>,
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details.
    pub channel: String,
    /// Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below.
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub icon_url: Option<String>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Disable Slack markup parsing by setting to `false`. Enabled by default.
    pub mrkdwn: Option<bool>,
    /// Change how messages are treated. Defaults to `none`. See [below](#formatting).
    pub parse: Option<String>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    pub reply_broadcast: Option<bool>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    pub text: Option<String>,
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    pub thread_ts: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
    /// Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: PostMessageIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<PostMessageReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageFileInner {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<u64>,
    pub created: Option<u64>,
    pub date_delete: Option<u64>,
    pub display_as_bot: Option<bool>,
    pub editable: Option<bool>,
    pub editor: Option<String>,
    pub external_id: Option<String>,
    pub external_type: Option<String>,
    pub external_url: Option<String>,
    pub filetype: Option<String>,
    pub groups: Option<Vec<String>>,
    pub has_rich_preview: Option<bool>,
    pub id: Option<String>,
    pub image_exif_rotation: Option<u64>,
    pub ims: Option<Vec<String>>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub is_starred: Option<bool>,
    pub is_tombstoned: Option<bool>,
    pub last_editor: Option<String>,
    pub mimetype: Option<String>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub non_owner_editable: Option<bool>,
    pub num_stars: Option<u64>,
    pub original_h: Option<u64>,
    pub original_w: Option<u64>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub pretty_type: Option<String>,
    pub preview: Option<String>,
    pub public_url_shared: Option<bool>,
    pub reactions: Option<Vec<PostMessageReactions1Inner>>,
    pub shares: Option<PostMessageSharesInner>,
    pub size: Option<u64>,
    pub source_team: Option<String>,
    pub state: Option<String>,
    pub thumb_1024: Option<String>,
    pub thumb_1024_h: Option<u64>,
    pub thumb_1024_w: Option<u64>,
    pub thumb_160: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_h: Option<u64>,
    pub thumb_360_w: Option<u64>,
    pub thumb_480: Option<String>,
    pub thumb_480_h: Option<u64>,
    pub thumb_480_w: Option<u64>,
    pub thumb_64: Option<String>,
    pub thumb_720: Option<String>,
    pub thumb_720_h: Option<u64>,
    pub thumb_720_w: Option<u64>,
    pub thumb_80: Option<String>,
    pub thumb_800: Option<String>,
    pub thumb_800_h: Option<u64>,
    pub thumb_800_w: Option<u64>,
    pub thumb_960: Option<String>,
    pub thumb_960_h: Option<u64>,
    pub thumb_960_w: Option<u64>,
    pub thumb_tiny: Option<String>,
    pub timestamp: Option<u64>,
    pub title: Option<String>,
    pub updated: Option<u64>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub user: Option<String>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageFilesInner {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<u64>,
    pub created: Option<u64>,
    pub date_delete: Option<u64>,
    pub display_as_bot: Option<bool>,
    pub editable: Option<bool>,
    pub editor: Option<String>,
    pub external_id: Option<String>,
    pub external_type: Option<String>,
    pub external_url: Option<String>,
    pub filetype: Option<String>,
    pub groups: Option<Vec<String>>,
    pub has_rich_preview: Option<bool>,
    pub id: Option<String>,
    pub image_exif_rotation: Option<u64>,
    pub ims: Option<Vec<String>>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub is_starred: Option<bool>,
    pub is_tombstoned: Option<bool>,
    pub last_editor: Option<String>,
    pub mimetype: Option<String>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub non_owner_editable: Option<bool>,
    pub num_stars: Option<u64>,
    pub original_h: Option<u64>,
    pub original_w: Option<u64>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub pretty_type: Option<String>,
    pub preview: Option<String>,
    pub public_url_shared: Option<bool>,
    pub reactions: Option<Vec<PostMessageReactions2Inner>>,
    pub shares: Option<PostMessageShares1Inner>,
    pub size: Option<u64>,
    pub source_team: Option<String>,
    pub state: Option<String>,
    pub thumb_1024: Option<String>,
    pub thumb_1024_h: Option<u64>,
    pub thumb_1024_w: Option<u64>,
    pub thumb_160: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_h: Option<u64>,
    pub thumb_360_w: Option<u64>,
    pub thumb_480: Option<String>,
    pub thumb_480_h: Option<u64>,
    pub thumb_480_w: Option<u64>,
    pub thumb_64: Option<String>,
    pub thumb_720: Option<String>,
    pub thumb_720_h: Option<u64>,
    pub thumb_720_w: Option<u64>,
    pub thumb_80: Option<String>,
    pub thumb_800: Option<String>,
    pub thumb_800_h: Option<u64>,
    pub thumb_800_w: Option<u64>,
    pub thumb_960: Option<String>,
    pub thumb_960_h: Option<u64>,
    pub thumb_960_w: Option<u64>,
    pub thumb_tiny: Option<String>,
    pub timestamp: Option<u64>,
    pub title: Option<String>,
    pub updated: Option<u64>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub user: Option<String>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageUserProfileInner {
    pub avatar_hash: String,
    pub display_name: String,
    pub display_name_normalized: Option<String>,
    pub first_name: String,
    pub image_72: String,
    pub is_restricted: bool,
    pub is_ultra_restricted: bool,
    pub name: String,
    pub real_name: String,
    pub real_name_normalized: Option<String>,
    pub team: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageMessageInner {
    pub attachments: Option<Vec<PostMessageAttachmentsInner>>,
    pub blocks: Option<Vec<PostMessageBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<PostMessageBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<PostMessageCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<PostMessageFileInner>,
    pub files: Option<Vec<PostMessageFilesInner>>,
    pub icons: Option<PostMessageIcons1Inner>,
    pub inviter: Option<String>,
    pub is_delayed_message: Option<bool>,
    pub is_intro: Option<bool>,
    pub is_starred: Option<bool>,
    pub last_read: Option<String>,
    pub latest_reply: Option<String>,
    pub name: Option<String>,
    pub old_name: Option<String>,
    pub parent_user_id: Option<String>,
    pub permalink: Option<String>,
    pub pinned_to: Option<Vec<String>>,
    pub purpose: Option<String>,
    pub reactions: Option<Vec<PostMessageReactions3Inner>>,
    pub reply_count: Option<u64>,
    pub reply_users: Option<Vec<String>>,
    pub reply_users_count: Option<u64>,
    pub source_team: Option<String>,
    pub subscribed: Option<bool>,
    pub subtype: Option<String>,
    pub team: Option<String>,
    pub text: String,
    pub thread_ts: Option<String>,
    pub topic: Option<String>,
    pub ts: String,
    pub r#type: String,
    pub unread_count: Option<u64>,
    pub upload: Option<bool>,
    pub user: Option<String>,
    pub user_profile: Option<PostMessageUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageResponse {
    pub callstack: Option<String>,
    pub channel: String,
    error: Option<String>,
    pub message: PostMessageMessageInner,
    #[serde(default)]
    ok: bool,
    pub ts: String,
}

impl<E: Error> Into<Result<PostMessageResponse, PostMessageError<E>>> for PostMessageResponse {
    fn into(self) -> Result<PostMessageResponse, PostMessageError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum PostMessageError<E: Error> {
    ChannelNotFound,
    NotInChannel,
    IsArchived,
    MsgTooLong,
    NoText,
    TooManyAttachments,
    RateLimited,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for PostMessageError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => PostMessageError::ChannelNotFound,
            "not_in_channel" => PostMessageError::NotInChannel,
            "is_archived" => PostMessageError::IsArchived,
            "msg_too_long" => PostMessageError::MsgTooLong,
            "no_text" => PostMessageError::NoText,
            "too_many_attachments" => PostMessageError::TooManyAttachments,
            "rate_limited" => PostMessageError::RateLimited,
            "not_authed" => PostMessageError::NotAuthed,
            "invalid_auth" => PostMessageError::InvalidAuth,
            "account_inactive" => PostMessageError::AccountInactive,
            "invalid_arg_name" => PostMessageError::InvalidArgName,
            "invalid_array_arg" => PostMessageError::InvalidArrayArg,
            "invalid_charset" => PostMessageError::InvalidCharset,
            "invalid_form_data" => PostMessageError::InvalidFormData,
            "invalid_post_type" => PostMessageError::InvalidPostType,
            "missing_post_type" => PostMessageError::MissingPostType,
            _ => PostMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for PostMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PostMessageError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            PostMessageError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            PostMessageError::IsArchived => write!(f, "Server returned error is_archived"),
            PostMessageError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            PostMessageError::NoText => write!(f, "Server returned error no_text"),
            PostMessageError::TooManyAttachments => {
                write!(f, "Server returned error too_many_attachments")
            }
            PostMessageError::RateLimited => write!(f, "Server returned error rate_limited"),
            PostMessageError::NotAuthed => write!(f, "Server returned error not_authed"),
            PostMessageError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            PostMessageError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            PostMessageError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            PostMessageError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            PostMessageError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            PostMessageError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            PostMessageError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            PostMessageError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            PostMessageError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            PostMessageError::Unknown(ref s) => write!(f, "{}", s),
            PostMessageError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for PostMessageError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            PostMessageError::MalformedResponse(_, ref e) => Some(e),
            PostMessageError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct PostEphemeralRequest {
    /// Pass true to post the message as the authed user. Defaults to true if the chat:write:bot scope is not included. Otherwise, defaults to false.
    pub as_user: Option<bool>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    pub attachments: Option<String>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<String>,
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name.
    pub channel: String,
    /// Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below.
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub icon_url: Option<String>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Change how messages are treated. Defaults to `none`. See [below](#formatting).
    pub parse: Option<String>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    pub text: Option<String>,
    /// Provide another message's `ts` value to post this message in a thread. Avoid using a reply's `ts` value; use its parent's value instead. Ephemeral messages in threads are only shown if there is already an active thread.
    pub thread_ts: Option<String>,
    /// `id` of the user who will receive the ephemeral message. The user should be in the channel specified by the `channel` argument.
    pub user: String,
    /// Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostEphemeralResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub message_ts: String,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<PostEphemeralResponse, PostEphemeralError<E>>>
    for PostEphemeralResponse
{
    fn into(self) -> Result<PostEphemeralResponse, PostEphemeralError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum PostEphemeralError<E: Error> {
    ChannelNotFound,
    IsArchived,
    MsgTooLong,
    NoText,
    RestrictedAction,
    TooManyAttachments,
    UserNotInChannel,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    FatalError,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for PostEphemeralError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => PostEphemeralError::ChannelNotFound,
            "is_archived" => PostEphemeralError::IsArchived,
            "msg_too_long" => PostEphemeralError::MsgTooLong,
            "no_text" => PostEphemeralError::NoText,
            "restricted_action" => PostEphemeralError::RestrictedAction,
            "too_many_attachments" => PostEphemeralError::TooManyAttachments,
            "user_not_in_channel" => PostEphemeralError::UserNotInChannel,
            "not_authed" => PostEphemeralError::NotAuthed,
            "invalid_auth" => PostEphemeralError::InvalidAuth,
            "account_inactive" => PostEphemeralError::AccountInactive,
            "token_revoked" => PostEphemeralError::TokenRevoked,
            "no_permission" => PostEphemeralError::NoPermission,
            "org_login_required" => PostEphemeralError::OrgLoginRequired,
            "invalid_arg_name" => PostEphemeralError::InvalidArgName,
            "invalid_array_arg" => PostEphemeralError::InvalidArrayArg,
            "invalid_charset" => PostEphemeralError::InvalidCharset,
            "invalid_form_data" => PostEphemeralError::InvalidFormData,
            "invalid_post_type" => PostEphemeralError::InvalidPostType,
            "missing_post_type" => PostEphemeralError::MissingPostType,
            "team_added_to_org" => PostEphemeralError::TeamAddedToOrg,
            "invalid_json" => PostEphemeralError::InvalidJson,
            "json_not_object" => PostEphemeralError::JsonNotObject,
            "request_timeout" => PostEphemeralError::RequestTimeout,
            "upgrade_required" => PostEphemeralError::UpgradeRequired,
            "fatal_error" => PostEphemeralError::FatalError,
            _ => PostEphemeralError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for PostEphemeralError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PostEphemeralError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            PostEphemeralError::IsArchived => write!(f, "Server returned error is_archived"),
            PostEphemeralError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            PostEphemeralError::NoText => write!(f, "Server returned error no_text"),
            PostEphemeralError::RestrictedAction => {
                write!(f, "Server returned error restricted_action")
            }
            PostEphemeralError::TooManyAttachments => {
                write!(f, "Server returned error too_many_attachments")
            }
            PostEphemeralError::UserNotInChannel => {
                write!(f, "Server returned error user_not_in_channel")
            }
            PostEphemeralError::NotAuthed => write!(f, "Server returned error not_authed"),
            PostEphemeralError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            PostEphemeralError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            PostEphemeralError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            PostEphemeralError::NoPermission => write!(f, "Server returned error no_permission"),
            PostEphemeralError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            PostEphemeralError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            PostEphemeralError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            PostEphemeralError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            PostEphemeralError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            PostEphemeralError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            PostEphemeralError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            PostEphemeralError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            PostEphemeralError::InvalidJson => write!(f, "Server returned error invalid_json"),
            PostEphemeralError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            PostEphemeralError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            PostEphemeralError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            PostEphemeralError::FatalError => write!(f, "Server returned error fatal_error"),
            PostEphemeralError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            PostEphemeralError::Unknown(ref s) => write!(f, "{}", s),
            PostEphemeralError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for PostEphemeralError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            PostEphemeralError::MalformedResponse(_, ref e) => Some(e),
            PostEphemeralError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GetPermalinkRequest {
    /// The ID of the conversation or channel containing the message
    pub channel: String,
    /// A message's `ts` value, uniquely identifying it within a channel
    pub message_ts: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetPermalinkResponse {
    pub callstack: Option<String>,
    pub channel: String,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub permalink: String,
}

impl<E: Error> Into<Result<GetPermalinkResponse, GetPermalinkError<E>>> for GetPermalinkResponse {
    fn into(self) -> Result<GetPermalinkResponse, GetPermalinkError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum GetPermalinkError<E: Error> {
    ChannelNotFound,
    MessageNotFound,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    FatalError,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for GetPermalinkError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => GetPermalinkError::ChannelNotFound,
            "message_not_found" => GetPermalinkError::MessageNotFound,
            "not_authed" => GetPermalinkError::NotAuthed,
            "invalid_auth" => GetPermalinkError::InvalidAuth,
            "account_inactive" => GetPermalinkError::AccountInactive,
            "token_revoked" => GetPermalinkError::TokenRevoked,
            "no_permission" => GetPermalinkError::NoPermission,
            "org_login_required" => GetPermalinkError::OrgLoginRequired,
            "invalid_arg_name" => GetPermalinkError::InvalidArgName,
            "invalid_array_arg" => GetPermalinkError::InvalidArrayArg,
            "invalid_charset" => GetPermalinkError::InvalidCharset,
            "invalid_form_data" => GetPermalinkError::InvalidFormData,
            "invalid_post_type" => GetPermalinkError::InvalidPostType,
            "missing_post_type" => GetPermalinkError::MissingPostType,
            "team_added_to_org" => GetPermalinkError::TeamAddedToOrg,
            "invalid_json" => GetPermalinkError::InvalidJson,
            "json_not_object" => GetPermalinkError::JsonNotObject,
            "request_timeout" => GetPermalinkError::RequestTimeout,
            "upgrade_required" => GetPermalinkError::UpgradeRequired,
            "fatal_error" => GetPermalinkError::FatalError,
            _ => GetPermalinkError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetPermalinkError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetPermalinkError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            GetPermalinkError::MessageNotFound => {
                write!(f, "Server returned error message_not_found")
            }
            GetPermalinkError::NotAuthed => write!(f, "Server returned error not_authed"),
            GetPermalinkError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            GetPermalinkError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            GetPermalinkError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            GetPermalinkError::NoPermission => write!(f, "Server returned error no_permission"),
            GetPermalinkError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            GetPermalinkError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            GetPermalinkError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            GetPermalinkError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            GetPermalinkError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            GetPermalinkError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            GetPermalinkError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            GetPermalinkError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            GetPermalinkError::InvalidJson => write!(f, "Server returned error invalid_json"),
            GetPermalinkError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            GetPermalinkError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            GetPermalinkError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            GetPermalinkError::FatalError => write!(f, "Server returned error fatal_error"),
            GetPermalinkError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            GetPermalinkError::Unknown(ref s) => write!(f, "{}", s),
            GetPermalinkError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for GetPermalinkError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            GetPermalinkError::MalformedResponse(_, ref e) => Some(e),
            GetPermalinkError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest {
    /// Timestamp of the message to be deleted.
    pub ts: Option<u64>,
    /// Channel containing the message to be deleted.
    pub channel: Option<String>,
    /// Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope.
    pub as_user: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteResponse {
    pub callstack: Option<String>,
    pub channel: String,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub ts: String,
}

impl<E: Error> Into<Result<DeleteResponse, DeleteError<E>>> for DeleteResponse {
    fn into(self) -> Result<DeleteResponse, DeleteError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum DeleteError<E: Error> {
    MessageNotFound,
    ChannelNotFound,
    CantDeleteMessage,
    ComplianceExportsPreventDeletion,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    FatalError,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for DeleteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "message_not_found" => DeleteError::MessageNotFound,
            "channel_not_found" => DeleteError::ChannelNotFound,
            "cant_delete_message" => DeleteError::CantDeleteMessage,
            "compliance_exports_prevent_deletion" => DeleteError::ComplianceExportsPreventDeletion,
            "not_authed" => DeleteError::NotAuthed,
            "invalid_auth" => DeleteError::InvalidAuth,
            "account_inactive" => DeleteError::AccountInactive,
            "token_revoked" => DeleteError::TokenRevoked,
            "no_permission" => DeleteError::NoPermission,
            "invalid_arg_name" => DeleteError::InvalidArgName,
            "invalid_array_arg" => DeleteError::InvalidArrayArg,
            "invalid_charset" => DeleteError::InvalidCharset,
            "invalid_form_data" => DeleteError::InvalidFormData,
            "invalid_post_type" => DeleteError::InvalidPostType,
            "missing_post_type" => DeleteError::MissingPostType,
            "team_added_to_org" => DeleteError::TeamAddedToOrg,
            "invalid_json" => DeleteError::InvalidJson,
            "json_not_object" => DeleteError::JsonNotObject,
            "request_timeout" => DeleteError::RequestTimeout,
            "upgrade_required" => DeleteError::UpgradeRequired,
            "fatal_error" => DeleteError::FatalError,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeleteError::MessageNotFound => write!(f, "Server returned error message_not_found"),
            DeleteError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            DeleteError::CantDeleteMessage => {
                write!(f, "Server returned error cant_delete_message")
            }
            DeleteError::ComplianceExportsPreventDeletion => write!(
                f,
                "Server returned error compliance_exports_prevent_deletion"
            ),
            DeleteError::NotAuthed => write!(f, "Server returned error not_authed"),
            DeleteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            DeleteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            DeleteError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            DeleteError::NoPermission => write!(f, "Server returned error no_permission"),
            DeleteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            DeleteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            DeleteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            DeleteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            DeleteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            DeleteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            DeleteError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            DeleteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            DeleteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            DeleteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            DeleteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            DeleteError::FatalError => write!(f, "Server returned error fatal_error"),
            DeleteError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            DeleteError::Unknown(ref s) => write!(f, "{}", s),
            DeleteError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for DeleteError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DeleteError::MalformedResponse(_, ref e) => Some(e),
            DeleteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ScheduleMessageRequest {
    /// Channel, private group, or DM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details.
    pub channel: Option<String>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    pub text: Option<String>,
    /// Unix EPOCH timestamp of time in future to send the message.
    pub post_at: Option<String>,
    /// Change how messages are treated. Defaults to `none`. See [chat.postMessage](chat.postMessage#formatting).
    pub parse: Option<String>,
    /// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [chat.postMessage](chat.postMessage#authorship).
    pub as_user: Option<bool>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    pub attachments: Option<String>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    pub thread_ts: Option<u64>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    pub reply_broadcast: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScheduleMessageIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScheduleMessageBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: ScheduleMessageIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScheduleMessageMessageInner {
    pub bot_id: String,
    pub bot_profile: Option<ScheduleMessageBotProfileInner>,
    pub team: String,
    pub text: String,
    pub r#type: String,
    pub user: String,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScheduleMessageResponse {
    pub callstack: Option<String>,
    pub channel: String,
    error: Option<String>,
    pub message: ScheduleMessageMessageInner,
    #[serde(default)]
    ok: bool,
    pub post_at: u64,
    pub scheduled_message_id: String,
}

impl<E: Error> Into<Result<ScheduleMessageResponse, ScheduleMessageError<E>>>
    for ScheduleMessageResponse
{
    fn into(self) -> Result<ScheduleMessageResponse, ScheduleMessageError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum ScheduleMessageError<E: Error> {
    InvalidTime,
    TimeInPast,
    TimeTooFar,
    ChannelNotFound,
    NotInChannel,
    IsArchived,
    MsgTooLong,
    NoText,
    RestrictedAction,
    RestrictedActionReadOnlyChannel,
    RestrictedActionThreadOnlyChannel,
    RestrictedActionNonThreadableChannel,
    TooManyAttachments,
    RateLimited,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    EkmAccessDenied,
    MissingScope,
    InvalidArguments,
    InvalidArgName,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    FatalError,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ScheduleMessageError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "invalid_time" => ScheduleMessageError::InvalidTime,
            "time_in_past" => ScheduleMessageError::TimeInPast,
            "time_too_far" => ScheduleMessageError::TimeTooFar,
            "channel_not_found" => ScheduleMessageError::ChannelNotFound,
            "not_in_channel" => ScheduleMessageError::NotInChannel,
            "is_archived" => ScheduleMessageError::IsArchived,
            "msg_too_long" => ScheduleMessageError::MsgTooLong,
            "no_text" => ScheduleMessageError::NoText,
            "restricted_action" => ScheduleMessageError::RestrictedAction,
            "restricted_action_read_only_channel" => {
                ScheduleMessageError::RestrictedActionReadOnlyChannel
            }
            "restricted_action_thread_only_channel" => {
                ScheduleMessageError::RestrictedActionThreadOnlyChannel
            }
            "restricted_action_non_threadable_channel" => {
                ScheduleMessageError::RestrictedActionNonThreadableChannel
            }
            "too_many_attachments" => ScheduleMessageError::TooManyAttachments,
            "rate_limited" => ScheduleMessageError::RateLimited,
            "not_authed" => ScheduleMessageError::NotAuthed,
            "invalid_auth" => ScheduleMessageError::InvalidAuth,
            "account_inactive" => ScheduleMessageError::AccountInactive,
            "token_revoked" => ScheduleMessageError::TokenRevoked,
            "no_permission" => ScheduleMessageError::NoPermission,
            "org_login_required" => ScheduleMessageError::OrgLoginRequired,
            "ekm_access_denied" => ScheduleMessageError::EkmAccessDenied,
            "missing_scope" => ScheduleMessageError::MissingScope,
            "invalid_arguments" => ScheduleMessageError::InvalidArguments,
            "invalid_arg_name" => ScheduleMessageError::InvalidArgName,
            "invalid_charset" => ScheduleMessageError::InvalidCharset,
            "invalid_form_data" => ScheduleMessageError::InvalidFormData,
            "invalid_post_type" => ScheduleMessageError::InvalidPostType,
            "missing_post_type" => ScheduleMessageError::MissingPostType,
            "team_added_to_org" => ScheduleMessageError::TeamAddedToOrg,
            "invalid_json" => ScheduleMessageError::InvalidJson,
            "json_not_object" => ScheduleMessageError::JsonNotObject,
            "request_timeout" => ScheduleMessageError::RequestTimeout,
            "upgrade_required" => ScheduleMessageError::UpgradeRequired,
            "fatal_error" => ScheduleMessageError::FatalError,
            _ => ScheduleMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ScheduleMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ScheduleMessageError::InvalidTime => write!(f, "Server returned error invalid_time"),
            ScheduleMessageError::TimeInPast => write!(f, "Server returned error time_in_past"),
            ScheduleMessageError::TimeTooFar => write!(f, "Server returned error time_too_far"),
            ScheduleMessageError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            ScheduleMessageError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            ScheduleMessageError::IsArchived => write!(f, "Server returned error is_archived"),
            ScheduleMessageError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            ScheduleMessageError::NoText => write!(f, "Server returned error no_text"),
            ScheduleMessageError::RestrictedAction => {
                write!(f, "Server returned error restricted_action")
            }
            ScheduleMessageError::RestrictedActionReadOnlyChannel => write!(
                f,
                "Server returned error restricted_action_read_only_channel"
            ),
            ScheduleMessageError::RestrictedActionThreadOnlyChannel => write!(
                f,
                "Server returned error restricted_action_thread_only_channel"
            ),
            ScheduleMessageError::RestrictedActionNonThreadableChannel => write!(
                f,
                "Server returned error restricted_action_non_threadable_channel"
            ),
            ScheduleMessageError::TooManyAttachments => {
                write!(f, "Server returned error too_many_attachments")
            }
            ScheduleMessageError::RateLimited => write!(f, "Server returned error rate_limited"),
            ScheduleMessageError::NotAuthed => write!(f, "Server returned error not_authed"),
            ScheduleMessageError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ScheduleMessageError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            ScheduleMessageError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ScheduleMessageError::NoPermission => write!(f, "Server returned error no_permission"),
            ScheduleMessageError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            ScheduleMessageError::EkmAccessDenied => {
                write!(f, "Server returned error ekm_access_denied")
            }
            ScheduleMessageError::MissingScope => write!(f, "Server returned error missing_scope"),
            ScheduleMessageError::InvalidArguments => {
                write!(f, "Server returned error invalid_arguments")
            }
            ScheduleMessageError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            ScheduleMessageError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            ScheduleMessageError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            ScheduleMessageError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            ScheduleMessageError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            ScheduleMessageError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            ScheduleMessageError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ScheduleMessageError::JsonNotObject => {
                write!(f, "Server returned error json_not_object")
            }
            ScheduleMessageError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            ScheduleMessageError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            ScheduleMessageError::FatalError => write!(f, "Server returned error fatal_error"),
            ScheduleMessageError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ScheduleMessageError::Unknown(ref s) => write!(f, "{}", s),
            ScheduleMessageError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ScheduleMessageError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ScheduleMessageError::MalformedResponse(_, ref e) => Some(e),
            ScheduleMessageError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct DeleteScheduledMessageRequest {
    /// Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope.
    pub as_user: Option<bool>,
    /// The channel the scheduled_message is posting to
    pub channel: String,
    /// `scheduled_message_id` returned from call to chat.scheduleMessage
    pub scheduled_message_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteScheduledMessageResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<DeleteScheduledMessageResponse, DeleteScheduledMessageError<E>>>
    for DeleteScheduledMessageResponse
{
    fn into(self) -> Result<DeleteScheduledMessageResponse, DeleteScheduledMessageError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum DeleteScheduledMessageError<E: Error> {
    InvalidScheduledMessageId,
    ChannelNotFound,
    BadToken,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    EkmAccessDenied,
    MissingScope,
    InvalidArguments,
    InvalidArgName,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    FatalError,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for DeleteScheduledMessageError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "invalid_scheduled_message_id" => {
                DeleteScheduledMessageError::InvalidScheduledMessageId
            }
            "channel_not_found" => DeleteScheduledMessageError::ChannelNotFound,
            "bad_token" => DeleteScheduledMessageError::BadToken,
            "not_authed" => DeleteScheduledMessageError::NotAuthed,
            "invalid_auth" => DeleteScheduledMessageError::InvalidAuth,
            "account_inactive" => DeleteScheduledMessageError::AccountInactive,
            "token_revoked" => DeleteScheduledMessageError::TokenRevoked,
            "no_permission" => DeleteScheduledMessageError::NoPermission,
            "org_login_required" => DeleteScheduledMessageError::OrgLoginRequired,
            "ekm_access_denied" => DeleteScheduledMessageError::EkmAccessDenied,
            "missing_scope" => DeleteScheduledMessageError::MissingScope,
            "invalid_arguments" => DeleteScheduledMessageError::InvalidArguments,
            "invalid_arg_name" => DeleteScheduledMessageError::InvalidArgName,
            "invalid_charset" => DeleteScheduledMessageError::InvalidCharset,
            "invalid_form_data" => DeleteScheduledMessageError::InvalidFormData,
            "invalid_post_type" => DeleteScheduledMessageError::InvalidPostType,
            "missing_post_type" => DeleteScheduledMessageError::MissingPostType,
            "team_added_to_org" => DeleteScheduledMessageError::TeamAddedToOrg,
            "invalid_json" => DeleteScheduledMessageError::InvalidJson,
            "json_not_object" => DeleteScheduledMessageError::JsonNotObject,
            "request_timeout" => DeleteScheduledMessageError::RequestTimeout,
            "upgrade_required" => DeleteScheduledMessageError::UpgradeRequired,
            "fatal_error" => DeleteScheduledMessageError::FatalError,
            _ => DeleteScheduledMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteScheduledMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeleteScheduledMessageError::InvalidScheduledMessageId => {
                write!(f, "Server returned error invalid_scheduled_message_id")
            }
            DeleteScheduledMessageError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            DeleteScheduledMessageError::BadToken => write!(f, "Server returned error bad_token"),
            DeleteScheduledMessageError::NotAuthed => write!(f, "Server returned error not_authed"),
            DeleteScheduledMessageError::InvalidAuth => {
                write!(f, "Server returned error invalid_auth")
            }
            DeleteScheduledMessageError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            DeleteScheduledMessageError::TokenRevoked => {
                write!(f, "Server returned error token_revoked")
            }
            DeleteScheduledMessageError::NoPermission => {
                write!(f, "Server returned error no_permission")
            }
            DeleteScheduledMessageError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            DeleteScheduledMessageError::EkmAccessDenied => {
                write!(f, "Server returned error ekm_access_denied")
            }
            DeleteScheduledMessageError::MissingScope => {
                write!(f, "Server returned error missing_scope")
            }
            DeleteScheduledMessageError::InvalidArguments => {
                write!(f, "Server returned error invalid_arguments")
            }
            DeleteScheduledMessageError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            DeleteScheduledMessageError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            DeleteScheduledMessageError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            DeleteScheduledMessageError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            DeleteScheduledMessageError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            DeleteScheduledMessageError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            DeleteScheduledMessageError::InvalidJson => {
                write!(f, "Server returned error invalid_json")
            }
            DeleteScheduledMessageError::JsonNotObject => {
                write!(f, "Server returned error json_not_object")
            }
            DeleteScheduledMessageError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            DeleteScheduledMessageError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            DeleteScheduledMessageError::FatalError => {
                write!(f, "Server returned error fatal_error")
            }
            DeleteScheduledMessageError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            DeleteScheduledMessageError::Unknown(ref s) => write!(f, "{}", s),
            DeleteScheduledMessageError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for DeleteScheduledMessageError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DeleteScheduledMessageError::MalformedResponse(_, ref e) => Some(e),
            DeleteScheduledMessageError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct UnfurlRequest {
    /// Channel ID of the message
    pub channel: String,
    /// Timestamp of the message to add unfurl behavior to.
    pub ts: String,
    /// URL-encoded JSON map with keys set to URLs featured in the the message, pointing to their unfurl blocks or message attachments.
    pub unfurls: Option<String>,
    /// Provide a simply-formatted string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior
    pub user_auth_message: Option<String>,
    /// Set to `true` or `1` to indicate the user must install your Slack app to trigger unfurls for this domain
    pub user_auth_required: Option<bool>,
    /// Send users to this custom URL where they will complete authentication in your app to fully trigger unfurling. Value should be properly URL-encoded.
    pub user_auth_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UnfurlResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<UnfurlResponse, UnfurlError<E>>> for UnfurlResponse {
    fn into(self) -> Result<UnfurlResponse, UnfurlError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum UnfurlError<E: Error> {
    CannotUnfurlUrl,
    CannotFindService,
    MissingUnfurls,
    CannotPrompt,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    UserIsBot,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    FatalError,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for UnfurlError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "cannot_unfurl_url" => UnfurlError::CannotUnfurlUrl,
            "cannot_find_service" => UnfurlError::CannotFindService,
            "missing_unfurls" => UnfurlError::MissingUnfurls,
            "cannot_prompt" => UnfurlError::CannotPrompt,
            "not_authed" => UnfurlError::NotAuthed,
            "invalid_auth" => UnfurlError::InvalidAuth,
            "account_inactive" => UnfurlError::AccountInactive,
            "token_revoked" => UnfurlError::TokenRevoked,
            "no_permission" => UnfurlError::NoPermission,
            "org_login_required" => UnfurlError::OrgLoginRequired,
            "user_is_bot" => UnfurlError::UserIsBot,
            "invalid_arg_name" => UnfurlError::InvalidArgName,
            "invalid_array_arg" => UnfurlError::InvalidArrayArg,
            "invalid_charset" => UnfurlError::InvalidCharset,
            "invalid_form_data" => UnfurlError::InvalidFormData,
            "invalid_post_type" => UnfurlError::InvalidPostType,
            "missing_post_type" => UnfurlError::MissingPostType,
            "team_added_to_org" => UnfurlError::TeamAddedToOrg,
            "invalid_json" => UnfurlError::InvalidJson,
            "json_not_object" => UnfurlError::JsonNotObject,
            "request_timeout" => UnfurlError::RequestTimeout,
            "upgrade_required" => UnfurlError::UpgradeRequired,
            "fatal_error" => UnfurlError::FatalError,
            _ => UnfurlError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UnfurlError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UnfurlError::CannotUnfurlUrl => write!(f, "Server returned error cannot_unfurl_url"),
            UnfurlError::CannotFindService => {
                write!(f, "Server returned error cannot_find_service")
            }
            UnfurlError::MissingUnfurls => write!(f, "Server returned error missing_unfurls"),
            UnfurlError::CannotPrompt => write!(f, "Server returned error cannot_prompt"),
            UnfurlError::NotAuthed => write!(f, "Server returned error not_authed"),
            UnfurlError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UnfurlError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UnfurlError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            UnfurlError::NoPermission => write!(f, "Server returned error no_permission"),
            UnfurlError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            UnfurlError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            UnfurlError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UnfurlError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UnfurlError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UnfurlError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UnfurlError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UnfurlError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UnfurlError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            UnfurlError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UnfurlError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UnfurlError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UnfurlError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            UnfurlError::FatalError => write!(f, "Server returned error fatal_error"),
            UnfurlError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            UnfurlError::Unknown(ref s) => write!(f, "{}", s),
            UnfurlError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for UnfurlError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UnfurlError::MalformedResponse(_, ref e) => Some(e),
            UnfurlError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct UpdateRequest {
    /// Pass true to update the message as the authed user. [Bot users](/bot-users) in this context are considered authed users.
    pub as_user: Option<String>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string. This field is required when not presenting `text`. If you don't include this field, the message's previous `attachments` will be retained. To remove previous `attachments`, include an empty array for this field.
    pub attachments: Option<String>,
    /// A JSON-based array of [structured blocks](/block-kit/building), presented as a URL-encoded string. If you don't include this field, the message's previous `blocks` will be retained. To remove previous `blocks`, include an empty array for this field.
    pub blocks: Option<String>,
    /// Channel containing the message to be updated.
    pub channel: String,
    /// Find and link channel names and usernames. Defaults to `none`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `none`.
    pub link_names: Option<String>,
    /// Change how messages are treated. Defaults to `client`, unlike `chat.postMessage`. Accepts either `none` or `full`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `client`.
    pub parse: Option<String>,
    /// New text for the message, using the [default formatting rules](/reference/surfaces/formatting). It's not required when presenting `blocks` or `attachments`.
    pub text: Option<String>,
    /// Timestamp of the message to be updated.
    pub ts: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateMessageInner {
    pub attachments: Option<Vec<serde_json::Value>>,
    pub blocks: Option<serde_json::Value>,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateResponse {
    pub callstack: Option<String>,
    pub channel: String,
    error: Option<String>,
    pub message: UpdateMessageInner,
    #[serde(default)]
    ok: bool,
    pub text: String,
    pub ts: String,
}

impl<E: Error> Into<Result<UpdateResponse, UpdateError<E>>> for UpdateResponse {
    fn into(self) -> Result<UpdateResponse, UpdateError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum UpdateError<E: Error> {
    MessageNotFound,
    CantUpdateMessage,
    ChannelNotFound,
    EditWindowClosed,
    MsgTooLong,
    TooManyAttachments,
    RateLimited,
    NoText,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    RequestTimeout,
    InvalidJson,
    JsonNotObject,
    UpgradeRequired,
    FatalError,
    IsInactive,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for UpdateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "message_not_found" => UpdateError::MessageNotFound,
            "cant_update_message" => UpdateError::CantUpdateMessage,
            "channel_not_found" => UpdateError::ChannelNotFound,
            "edit_window_closed" => UpdateError::EditWindowClosed,
            "msg_too_long" => UpdateError::MsgTooLong,
            "too_many_attachments" => UpdateError::TooManyAttachments,
            "rate_limited" => UpdateError::RateLimited,
            "no_text" => UpdateError::NoText,
            "not_authed" => UpdateError::NotAuthed,
            "invalid_auth" => UpdateError::InvalidAuth,
            "account_inactive" => UpdateError::AccountInactive,
            "token_revoked" => UpdateError::TokenRevoked,
            "no_permission" => UpdateError::NoPermission,
            "invalid_arg_name" => UpdateError::InvalidArgName,
            "invalid_array_arg" => UpdateError::InvalidArrayArg,
            "invalid_charset" => UpdateError::InvalidCharset,
            "invalid_form_data" => UpdateError::InvalidFormData,
            "invalid_post_type" => UpdateError::InvalidPostType,
            "missing_post_type" => UpdateError::MissingPostType,
            "request_timeout" => UpdateError::RequestTimeout,
            "invalid_json" => UpdateError::InvalidJson,
            "json_not_object" => UpdateError::JsonNotObject,
            "upgrade_required" => UpdateError::UpgradeRequired,
            "fatal_error" => UpdateError::FatalError,
            "is_inactive" => UpdateError::IsInactive,
            _ => UpdateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UpdateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UpdateError::MessageNotFound => write!(f, "Server returned error message_not_found"),
            UpdateError::CantUpdateMessage => {
                write!(f, "Server returned error cant_update_message")
            }
            UpdateError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            UpdateError::EditWindowClosed => write!(f, "Server returned error edit_window_closed"),
            UpdateError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            UpdateError::TooManyAttachments => {
                write!(f, "Server returned error too_many_attachments")
            }
            UpdateError::RateLimited => write!(f, "Server returned error rate_limited"),
            UpdateError::NoText => write!(f, "Server returned error no_text"),
            UpdateError::NotAuthed => write!(f, "Server returned error not_authed"),
            UpdateError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UpdateError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UpdateError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            UpdateError::NoPermission => write!(f, "Server returned error no_permission"),
            UpdateError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UpdateError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UpdateError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UpdateError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UpdateError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UpdateError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UpdateError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UpdateError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UpdateError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UpdateError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            UpdateError::FatalError => write!(f, "Server returned error fatal_error"),
            UpdateError::IsInactive => write!(f, "Server returned error is_inactive"),
            UpdateError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            UpdateError::Unknown(ref s) => write!(f, "{}", s),
            UpdateError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for UpdateError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UpdateError::MalformedResponse(_, ref e) => Some(e),
            UpdateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
