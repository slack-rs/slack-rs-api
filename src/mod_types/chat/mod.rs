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
#![allow(clippy::match_single_binding)]
#![allow(clippy::blacklisted_name)]

pub mod scheduled_messages_types;

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest<'a> {
    /// Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope.
    pub as_user: Option<bool>,
    /// Channel containing the message to be deleted.
    pub channel: Cow<'a, str>,
    /// Timestamp of the message to be deleted.
    pub ts: f64,
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
    AccountInactive,
    CantDeleteMessage,
    ChannelNotFound,
    ComplianceExportsPreventDeletion,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MessageNotFound,
    MissingPostType,
    NoPermission,
    NotAuthed,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
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
            "account_inactive" => DeleteError::AccountInactive,
            "cant_delete_message" => DeleteError::CantDeleteMessage,
            "channel_not_found" => DeleteError::ChannelNotFound,
            "compliance_exports_prevent_deletion" => DeleteError::ComplianceExportsPreventDeletion,
            "fatal_error" => DeleteError::FatalError,
            "invalid_arg_name" => DeleteError::InvalidArgName,
            "invalid_array_arg" => DeleteError::InvalidArrayArg,
            "invalid_auth" => DeleteError::InvalidAuth,
            "invalid_charset" => DeleteError::InvalidCharset,
            "invalid_form_data" => DeleteError::InvalidFormData,
            "invalid_json" => DeleteError::InvalidJson,
            "invalid_post_type" => DeleteError::InvalidPostType,
            "json_not_object" => DeleteError::JsonNotObject,
            "message_not_found" => DeleteError::MessageNotFound,
            "missing_post_type" => DeleteError::MissingPostType,
            "no_permission" => DeleteError::NoPermission,
            "not_authed" => DeleteError::NotAuthed,
            "request_timeout" => DeleteError::RequestTimeout,
            "team_added_to_org" => DeleteError::TeamAddedToOrg,
            "token_revoked" => DeleteError::TokenRevoked,
            "upgrade_required" => DeleteError::UpgradeRequired,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeleteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            DeleteError::CantDeleteMessage => {
                write!(f, "Server returned error cant_delete_message")
            }
            DeleteError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            DeleteError::ComplianceExportsPreventDeletion => write!(
                f,
                "Server returned error compliance_exports_prevent_deletion"
            ),
            DeleteError::FatalError => write!(f, "Server returned error fatal_error"),
            DeleteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            DeleteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            DeleteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            DeleteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            DeleteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            DeleteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            DeleteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            DeleteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            DeleteError::MessageNotFound => write!(f, "Server returned error message_not_found"),
            DeleteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            DeleteError::NoPermission => write!(f, "Server returned error no_permission"),
            DeleteError::NotAuthed => write!(f, "Server returned error not_authed"),
            DeleteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            DeleteError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            DeleteError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            DeleteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
pub struct DeleteScheduledMessageRequest<'a> {
    /// Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope.
    pub as_user: Option<bool>,
    /// The channel the scheduled_message is posting to
    pub channel: Cow<'a, str>,
    /// `scheduled_message_id` returned from call to chat.scheduleMessage
    pub scheduled_message_id: Cow<'a, str>,
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
    AccountInactive,
    BadToken,
    ChannelNotFound,
    EkmAccessDenied,
    FatalError,
    InvalidArgName,
    InvalidArguments,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    InvalidScheduledMessageId,
    JsonNotObject,
    MissingPostType,
    MissingScope,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
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
            "account_inactive" => DeleteScheduledMessageError::AccountInactive,
            "bad_token" => DeleteScheduledMessageError::BadToken,
            "channel_not_found" => DeleteScheduledMessageError::ChannelNotFound,
            "ekm_access_denied" => DeleteScheduledMessageError::EkmAccessDenied,
            "fatal_error" => DeleteScheduledMessageError::FatalError,
            "invalid_arg_name" => DeleteScheduledMessageError::InvalidArgName,
            "invalid_arguments" => DeleteScheduledMessageError::InvalidArguments,
            "invalid_auth" => DeleteScheduledMessageError::InvalidAuth,
            "invalid_charset" => DeleteScheduledMessageError::InvalidCharset,
            "invalid_form_data" => DeleteScheduledMessageError::InvalidFormData,
            "invalid_json" => DeleteScheduledMessageError::InvalidJson,
            "invalid_post_type" => DeleteScheduledMessageError::InvalidPostType,
            "invalid_scheduled_message_id" => {
                DeleteScheduledMessageError::InvalidScheduledMessageId
            }
            "json_not_object" => DeleteScheduledMessageError::JsonNotObject,
            "missing_post_type" => DeleteScheduledMessageError::MissingPostType,
            "missing_scope" => DeleteScheduledMessageError::MissingScope,
            "no_permission" => DeleteScheduledMessageError::NoPermission,
            "not_authed" => DeleteScheduledMessageError::NotAuthed,
            "org_login_required" => DeleteScheduledMessageError::OrgLoginRequired,
            "request_timeout" => DeleteScheduledMessageError::RequestTimeout,
            "team_added_to_org" => DeleteScheduledMessageError::TeamAddedToOrg,
            "token_revoked" => DeleteScheduledMessageError::TokenRevoked,
            "upgrade_required" => DeleteScheduledMessageError::UpgradeRequired,
            _ => DeleteScheduledMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteScheduledMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeleteScheduledMessageError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            DeleteScheduledMessageError::BadToken => write!(f, "Server returned error bad_token"),
            DeleteScheduledMessageError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            DeleteScheduledMessageError::EkmAccessDenied => {
                write!(f, "Server returned error ekm_access_denied")
            }
            DeleteScheduledMessageError::FatalError => {
                write!(f, "Server returned error fatal_error")
            }
            DeleteScheduledMessageError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            DeleteScheduledMessageError::InvalidArguments => {
                write!(f, "Server returned error invalid_arguments")
            }
            DeleteScheduledMessageError::InvalidAuth => {
                write!(f, "Server returned error invalid_auth")
            }
            DeleteScheduledMessageError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            DeleteScheduledMessageError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            DeleteScheduledMessageError::InvalidJson => {
                write!(f, "Server returned error invalid_json")
            }
            DeleteScheduledMessageError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            DeleteScheduledMessageError::InvalidScheduledMessageId => {
                write!(f, "Server returned error invalid_scheduled_message_id")
            }
            DeleteScheduledMessageError::JsonNotObject => {
                write!(f, "Server returned error json_not_object")
            }
            DeleteScheduledMessageError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            DeleteScheduledMessageError::MissingScope => {
                write!(f, "Server returned error missing_scope")
            }
            DeleteScheduledMessageError::NoPermission => {
                write!(f, "Server returned error no_permission")
            }
            DeleteScheduledMessageError::NotAuthed => write!(f, "Server returned error not_authed"),
            DeleteScheduledMessageError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            DeleteScheduledMessageError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            DeleteScheduledMessageError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            DeleteScheduledMessageError::TokenRevoked => {
                write!(f, "Server returned error token_revoked")
            }
            DeleteScheduledMessageError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
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
pub struct GetPermalinkRequest<'a> {
    /// The ID of the conversation or channel containing the message
    pub channel: Cow<'a, str>,
    /// A message's `ts` value, uniquely identifying it within a channel
    pub message_ts: Cow<'a, str>,
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
    AccountInactive,
    ChannelNotFound,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MessageNotFound,
    MissingPostType,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
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
            "account_inactive" => GetPermalinkError::AccountInactive,
            "channel_not_found" => GetPermalinkError::ChannelNotFound,
            "fatal_error" => GetPermalinkError::FatalError,
            "invalid_arg_name" => GetPermalinkError::InvalidArgName,
            "invalid_array_arg" => GetPermalinkError::InvalidArrayArg,
            "invalid_auth" => GetPermalinkError::InvalidAuth,
            "invalid_charset" => GetPermalinkError::InvalidCharset,
            "invalid_form_data" => GetPermalinkError::InvalidFormData,
            "invalid_json" => GetPermalinkError::InvalidJson,
            "invalid_post_type" => GetPermalinkError::InvalidPostType,
            "json_not_object" => GetPermalinkError::JsonNotObject,
            "message_not_found" => GetPermalinkError::MessageNotFound,
            "missing_post_type" => GetPermalinkError::MissingPostType,
            "no_permission" => GetPermalinkError::NoPermission,
            "not_authed" => GetPermalinkError::NotAuthed,
            "org_login_required" => GetPermalinkError::OrgLoginRequired,
            "request_timeout" => GetPermalinkError::RequestTimeout,
            "team_added_to_org" => GetPermalinkError::TeamAddedToOrg,
            "token_revoked" => GetPermalinkError::TokenRevoked,
            "upgrade_required" => GetPermalinkError::UpgradeRequired,
            _ => GetPermalinkError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetPermalinkError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetPermalinkError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            GetPermalinkError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            GetPermalinkError::FatalError => write!(f, "Server returned error fatal_error"),
            GetPermalinkError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            GetPermalinkError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            GetPermalinkError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            GetPermalinkError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            GetPermalinkError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            GetPermalinkError::InvalidJson => write!(f, "Server returned error invalid_json"),
            GetPermalinkError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            GetPermalinkError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            GetPermalinkError::MessageNotFound => {
                write!(f, "Server returned error message_not_found")
            }
            GetPermalinkError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            GetPermalinkError::NoPermission => write!(f, "Server returned error no_permission"),
            GetPermalinkError::NotAuthed => write!(f, "Server returned error not_authed"),
            GetPermalinkError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            GetPermalinkError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            GetPermalinkError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            GetPermalinkError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            GetPermalinkError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
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
pub struct MeMessageRequest<'a> {
    /// Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name.
    pub channel: Cow<'a, str>,
    /// Text of the message to send.
    pub text: Cow<'a, str>,
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
    AccountInactive,
    ChannelNotFound,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    IsArchived,
    JsonNotObject,
    MissingPostType,
    MsgTooLong,
    NoPermission,
    NoText,
    NotAuthed,
    NotInChannel,
    OrgLoginRequired,
    RateLimited,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
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
            "account_inactive" => MeMessageError::AccountInactive,
            "channel_not_found" => MeMessageError::ChannelNotFound,
            "fatal_error" => MeMessageError::FatalError,
            "invalid_arg_name" => MeMessageError::InvalidArgName,
            "invalid_array_arg" => MeMessageError::InvalidArrayArg,
            "invalid_auth" => MeMessageError::InvalidAuth,
            "invalid_charset" => MeMessageError::InvalidCharset,
            "invalid_form_data" => MeMessageError::InvalidFormData,
            "invalid_json" => MeMessageError::InvalidJson,
            "invalid_post_type" => MeMessageError::InvalidPostType,
            "is_archived" => MeMessageError::IsArchived,
            "json_not_object" => MeMessageError::JsonNotObject,
            "missing_post_type" => MeMessageError::MissingPostType,
            "msg_too_long" => MeMessageError::MsgTooLong,
            "no_permission" => MeMessageError::NoPermission,
            "no_text" => MeMessageError::NoText,
            "not_authed" => MeMessageError::NotAuthed,
            "not_in_channel" => MeMessageError::NotInChannel,
            "org_login_required" => MeMessageError::OrgLoginRequired,
            "rate_limited" => MeMessageError::RateLimited,
            "request_timeout" => MeMessageError::RequestTimeout,
            "team_added_to_org" => MeMessageError::TeamAddedToOrg,
            "token_revoked" => MeMessageError::TokenRevoked,
            "upgrade_required" => MeMessageError::UpgradeRequired,
            _ => MeMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MeMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MeMessageError::AccountInactive => write!(f, "Server returned error account_inactive"),
            MeMessageError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            MeMessageError::FatalError => write!(f, "Server returned error fatal_error"),
            MeMessageError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            MeMessageError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            MeMessageError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            MeMessageError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            MeMessageError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            MeMessageError::InvalidJson => write!(f, "Server returned error invalid_json"),
            MeMessageError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            MeMessageError::IsArchived => write!(f, "Server returned error is_archived"),
            MeMessageError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            MeMessageError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            MeMessageError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            MeMessageError::NoPermission => write!(f, "Server returned error no_permission"),
            MeMessageError::NoText => write!(f, "Server returned error no_text"),
            MeMessageError::NotAuthed => write!(f, "Server returned error not_authed"),
            MeMessageError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            MeMessageError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            MeMessageError::RateLimited => write!(f, "Server returned error rate_limited"),
            MeMessageError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            MeMessageError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            MeMessageError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            MeMessageError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
pub struct PostEphemeralRequest<'a> {
    /// Pass true to post the message as the authed user. Defaults to true if the chat:write:bot scope is not included. Otherwise, defaults to false.
    pub as_user: Option<bool>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    pub attachments: Option<Cow<'a, str>>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<Cow<'a, str>>,
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name.
    pub channel: Cow<'a, str>,
    /// Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below.
    pub icon_emoji: Option<Cow<'a, str>>,
    /// URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub icon_url: Option<Cow<'a, str>>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Change how messages are treated. Defaults to `none`. See [below](#formatting).
    pub parse: Option<Cow<'a, str>>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    pub text: Option<Cow<'a, str>>,
    /// Provide another message's `ts` value to post this message in a thread. Avoid using a reply's `ts` value; use its parent's value instead. Ephemeral messages in threads are only shown if there is already an active thread.
    pub thread_ts: Option<Cow<'a, str>>,
    /// `id` of the user who will receive the ephemeral message. The user should be in the channel specified by the `channel` argument.
    pub user: Cow<'a, str>,
    /// Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub username: Option<Cow<'a, str>>,
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
    AccountInactive,
    ChannelNotFound,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    IsArchived,
    JsonNotObject,
    MissingPostType,
    MsgTooLong,
    NoPermission,
    NoText,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    RestrictedAction,
    TeamAddedToOrg,
    TokenRevoked,
    TooManyAttachments,
    UpgradeRequired,
    UserNotInChannel,
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
            "account_inactive" => PostEphemeralError::AccountInactive,
            "channel_not_found" => PostEphemeralError::ChannelNotFound,
            "fatal_error" => PostEphemeralError::FatalError,
            "invalid_arg_name" => PostEphemeralError::InvalidArgName,
            "invalid_array_arg" => PostEphemeralError::InvalidArrayArg,
            "invalid_auth" => PostEphemeralError::InvalidAuth,
            "invalid_charset" => PostEphemeralError::InvalidCharset,
            "invalid_form_data" => PostEphemeralError::InvalidFormData,
            "invalid_json" => PostEphemeralError::InvalidJson,
            "invalid_post_type" => PostEphemeralError::InvalidPostType,
            "is_archived" => PostEphemeralError::IsArchived,
            "json_not_object" => PostEphemeralError::JsonNotObject,
            "missing_post_type" => PostEphemeralError::MissingPostType,
            "msg_too_long" => PostEphemeralError::MsgTooLong,
            "no_permission" => PostEphemeralError::NoPermission,
            "no_text" => PostEphemeralError::NoText,
            "not_authed" => PostEphemeralError::NotAuthed,
            "org_login_required" => PostEphemeralError::OrgLoginRequired,
            "request_timeout" => PostEphemeralError::RequestTimeout,
            "restricted_action" => PostEphemeralError::RestrictedAction,
            "team_added_to_org" => PostEphemeralError::TeamAddedToOrg,
            "token_revoked" => PostEphemeralError::TokenRevoked,
            "too_many_attachments" => PostEphemeralError::TooManyAttachments,
            "upgrade_required" => PostEphemeralError::UpgradeRequired,
            "user_not_in_channel" => PostEphemeralError::UserNotInChannel,
            _ => PostEphemeralError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for PostEphemeralError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PostEphemeralError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            PostEphemeralError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            PostEphemeralError::FatalError => write!(f, "Server returned error fatal_error"),
            PostEphemeralError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            PostEphemeralError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            PostEphemeralError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            PostEphemeralError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            PostEphemeralError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            PostEphemeralError::InvalidJson => write!(f, "Server returned error invalid_json"),
            PostEphemeralError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            PostEphemeralError::IsArchived => write!(f, "Server returned error is_archived"),
            PostEphemeralError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            PostEphemeralError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            PostEphemeralError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            PostEphemeralError::NoPermission => write!(f, "Server returned error no_permission"),
            PostEphemeralError::NoText => write!(f, "Server returned error no_text"),
            PostEphemeralError::NotAuthed => write!(f, "Server returned error not_authed"),
            PostEphemeralError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            PostEphemeralError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            PostEphemeralError::RestrictedAction => {
                write!(f, "Server returned error restricted_action")
            }
            PostEphemeralError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            PostEphemeralError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            PostEphemeralError::TooManyAttachments => {
                write!(f, "Server returned error too_many_attachments")
            }
            PostEphemeralError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            PostEphemeralError::UserNotInChannel => {
                write!(f, "Server returned error user_not_in_channel")
            }
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
pub struct PostMessageRequest<'a> {
    /// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [authorship](#authorship) below.
    pub as_user: Option<Cow<'a, str>>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    pub attachments: Option<Cow<'a, str>>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<Cow<'a, str>>,
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details.
    pub channel: Cow<'a, str>,
    /// Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below.
    pub icon_emoji: Option<Cow<'a, str>>,
    /// URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub icon_url: Option<Cow<'a, str>>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Disable Slack markup parsing by setting to `false`. Enabled by default.
    pub mrkdwn: Option<bool>,
    /// Change how messages are treated. Defaults to `none`. See [below](#formatting).
    pub parse: Option<Cow<'a, str>>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    pub reply_broadcast: Option<bool>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    pub text: Cow<'a, str>,
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    pub thread_ts: Option<Cow<'a, str>>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
    /// Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub username: Option<Cow<'a, str>>,
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
    AccountInactive,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    IsArchived,
    MissingPostType,
    MsgTooLong,
    NoText,
    NotAuthed,
    NotInChannel,
    RateLimited,
    TooManyAttachments,
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
            "account_inactive" => PostMessageError::AccountInactive,
            "channel_not_found" => PostMessageError::ChannelNotFound,
            "invalid_arg_name" => PostMessageError::InvalidArgName,
            "invalid_array_arg" => PostMessageError::InvalidArrayArg,
            "invalid_auth" => PostMessageError::InvalidAuth,
            "invalid_charset" => PostMessageError::InvalidCharset,
            "invalid_form_data" => PostMessageError::InvalidFormData,
            "invalid_post_type" => PostMessageError::InvalidPostType,
            "is_archived" => PostMessageError::IsArchived,
            "missing_post_type" => PostMessageError::MissingPostType,
            "msg_too_long" => PostMessageError::MsgTooLong,
            "no_text" => PostMessageError::NoText,
            "not_authed" => PostMessageError::NotAuthed,
            "not_in_channel" => PostMessageError::NotInChannel,
            "rate_limited" => PostMessageError::RateLimited,
            "too_many_attachments" => PostMessageError::TooManyAttachments,
            _ => PostMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for PostMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PostMessageError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            PostMessageError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            PostMessageError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            PostMessageError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            PostMessageError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            PostMessageError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            PostMessageError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            PostMessageError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            PostMessageError::IsArchived => write!(f, "Server returned error is_archived"),
            PostMessageError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            PostMessageError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            PostMessageError::NoText => write!(f, "Server returned error no_text"),
            PostMessageError::NotAuthed => write!(f, "Server returned error not_authed"),
            PostMessageError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            PostMessageError::RateLimited => write!(f, "Server returned error rate_limited"),
            PostMessageError::TooManyAttachments => {
                write!(f, "Server returned error too_many_attachments")
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
pub struct ScheduleMessageRequest<'a> {
    /// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [chat.postMessage](chat.postMessage#authorship).
    pub as_user: Option<bool>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    pub attachments: Option<Cow<'a, str>>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<Cow<'a, str>>,
    /// Channel, private group, or DM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details.
    pub channel: Option<Cow<'a, str>>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Change how messages are treated. Defaults to `none`. See [chat.postMessage](chat.postMessage#formatting).
    pub parse: Option<Cow<'a, str>>,
    /// Unix EPOCH timestamp of time in future to send the message.
    pub post_at: Option<Cow<'a, str>>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    pub reply_broadcast: Option<bool>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    pub text: Option<Cow<'a, str>>,
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    pub thread_ts: Option<f64>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
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
    AccountInactive,
    ChannelNotFound,
    EkmAccessDenied,
    FatalError,
    InvalidArgName,
    InvalidArguments,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    InvalidTime,
    IsArchived,
    JsonNotObject,
    MissingPostType,
    MissingScope,
    MsgTooLong,
    NoPermission,
    NoText,
    NotAuthed,
    NotInChannel,
    OrgLoginRequired,
    RateLimited,
    RequestTimeout,
    RestrictedAction,
    RestrictedActionNonThreadableChannel,
    RestrictedActionReadOnlyChannel,
    RestrictedActionThreadOnlyChannel,
    TeamAddedToOrg,
    TimeInPast,
    TimeTooFar,
    TokenRevoked,
    TooManyAttachments,
    UpgradeRequired,
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
            "account_inactive" => ScheduleMessageError::AccountInactive,
            "channel_not_found" => ScheduleMessageError::ChannelNotFound,
            "ekm_access_denied" => ScheduleMessageError::EkmAccessDenied,
            "fatal_error" => ScheduleMessageError::FatalError,
            "invalid_arg_name" => ScheduleMessageError::InvalidArgName,
            "invalid_arguments" => ScheduleMessageError::InvalidArguments,
            "invalid_auth" => ScheduleMessageError::InvalidAuth,
            "invalid_charset" => ScheduleMessageError::InvalidCharset,
            "invalid_form_data" => ScheduleMessageError::InvalidFormData,
            "invalid_json" => ScheduleMessageError::InvalidJson,
            "invalid_post_type" => ScheduleMessageError::InvalidPostType,
            "invalid_time" => ScheduleMessageError::InvalidTime,
            "is_archived" => ScheduleMessageError::IsArchived,
            "json_not_object" => ScheduleMessageError::JsonNotObject,
            "missing_post_type" => ScheduleMessageError::MissingPostType,
            "missing_scope" => ScheduleMessageError::MissingScope,
            "msg_too_long" => ScheduleMessageError::MsgTooLong,
            "no_permission" => ScheduleMessageError::NoPermission,
            "no_text" => ScheduleMessageError::NoText,
            "not_authed" => ScheduleMessageError::NotAuthed,
            "not_in_channel" => ScheduleMessageError::NotInChannel,
            "org_login_required" => ScheduleMessageError::OrgLoginRequired,
            "rate_limited" => ScheduleMessageError::RateLimited,
            "request_timeout" => ScheduleMessageError::RequestTimeout,
            "restricted_action" => ScheduleMessageError::RestrictedAction,
            "restricted_action_non_threadable_channel" => {
                ScheduleMessageError::RestrictedActionNonThreadableChannel
            }
            "restricted_action_read_only_channel" => {
                ScheduleMessageError::RestrictedActionReadOnlyChannel
            }
            "restricted_action_thread_only_channel" => {
                ScheduleMessageError::RestrictedActionThreadOnlyChannel
            }
            "team_added_to_org" => ScheduleMessageError::TeamAddedToOrg,
            "time_in_past" => ScheduleMessageError::TimeInPast,
            "time_too_far" => ScheduleMessageError::TimeTooFar,
            "token_revoked" => ScheduleMessageError::TokenRevoked,
            "too_many_attachments" => ScheduleMessageError::TooManyAttachments,
            "upgrade_required" => ScheduleMessageError::UpgradeRequired,
            _ => ScheduleMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ScheduleMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ScheduleMessageError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            ScheduleMessageError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            ScheduleMessageError::EkmAccessDenied => {
                write!(f, "Server returned error ekm_access_denied")
            }
            ScheduleMessageError::FatalError => write!(f, "Server returned error fatal_error"),
            ScheduleMessageError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            ScheduleMessageError::InvalidArguments => {
                write!(f, "Server returned error invalid_arguments")
            }
            ScheduleMessageError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ScheduleMessageError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            ScheduleMessageError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            ScheduleMessageError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ScheduleMessageError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            ScheduleMessageError::InvalidTime => write!(f, "Server returned error invalid_time"),
            ScheduleMessageError::IsArchived => write!(f, "Server returned error is_archived"),
            ScheduleMessageError::JsonNotObject => {
                write!(f, "Server returned error json_not_object")
            }
            ScheduleMessageError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            ScheduleMessageError::MissingScope => write!(f, "Server returned error missing_scope"),
            ScheduleMessageError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            ScheduleMessageError::NoPermission => write!(f, "Server returned error no_permission"),
            ScheduleMessageError::NoText => write!(f, "Server returned error no_text"),
            ScheduleMessageError::NotAuthed => write!(f, "Server returned error not_authed"),
            ScheduleMessageError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            ScheduleMessageError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            ScheduleMessageError::RateLimited => write!(f, "Server returned error rate_limited"),
            ScheduleMessageError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            ScheduleMessageError::RestrictedAction => {
                write!(f, "Server returned error restricted_action")
            }
            ScheduleMessageError::RestrictedActionNonThreadableChannel => write!(
                f,
                "Server returned error restricted_action_non_threadable_channel"
            ),
            ScheduleMessageError::RestrictedActionReadOnlyChannel => write!(
                f,
                "Server returned error restricted_action_read_only_channel"
            ),
            ScheduleMessageError::RestrictedActionThreadOnlyChannel => write!(
                f,
                "Server returned error restricted_action_thread_only_channel"
            ),
            ScheduleMessageError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            ScheduleMessageError::TimeInPast => write!(f, "Server returned error time_in_past"),
            ScheduleMessageError::TimeTooFar => write!(f, "Server returned error time_too_far"),
            ScheduleMessageError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ScheduleMessageError::TooManyAttachments => {
                write!(f, "Server returned error too_many_attachments")
            }
            ScheduleMessageError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
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
pub struct UnfurlRequest<'a> {
    /// Channel ID of the message
    pub channel: Cow<'a, str>,
    /// Timestamp of the message to add unfurl behavior to.
    pub ts: Cow<'a, str>,
    /// URL-encoded JSON map with keys set to URLs featured in the the message, pointing to their unfurl blocks or message attachments.
    pub unfurls: Cow<'a, str>,
    /// Provide a simply-formatted string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior
    pub user_auth_message: Option<Cow<'a, str>>,
    /// Set to `true` or `1` to indicate the user must install your Slack app to trigger unfurls for this domain
    pub user_auth_required: Option<bool>,
    /// Send users to this custom URL where they will complete authentication in your app to fully trigger unfurling. Value should be properly URL-encoded.
    pub user_auth_url: Option<Cow<'a, str>>,
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
    AccountInactive,
    CannotFindService,
    CannotPrompt,
    CannotUnfurlUrl,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    MissingUnfurls,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
    UserIsBot,
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
            "account_inactive" => UnfurlError::AccountInactive,
            "cannot_find_service" => UnfurlError::CannotFindService,
            "cannot_prompt" => UnfurlError::CannotPrompt,
            "cannot_unfurl_url" => UnfurlError::CannotUnfurlUrl,
            "fatal_error" => UnfurlError::FatalError,
            "invalid_arg_name" => UnfurlError::InvalidArgName,
            "invalid_array_arg" => UnfurlError::InvalidArrayArg,
            "invalid_auth" => UnfurlError::InvalidAuth,
            "invalid_charset" => UnfurlError::InvalidCharset,
            "invalid_form_data" => UnfurlError::InvalidFormData,
            "invalid_json" => UnfurlError::InvalidJson,
            "invalid_post_type" => UnfurlError::InvalidPostType,
            "json_not_object" => UnfurlError::JsonNotObject,
            "missing_post_type" => UnfurlError::MissingPostType,
            "missing_unfurls" => UnfurlError::MissingUnfurls,
            "no_permission" => UnfurlError::NoPermission,
            "not_authed" => UnfurlError::NotAuthed,
            "org_login_required" => UnfurlError::OrgLoginRequired,
            "request_timeout" => UnfurlError::RequestTimeout,
            "team_added_to_org" => UnfurlError::TeamAddedToOrg,
            "token_revoked" => UnfurlError::TokenRevoked,
            "upgrade_required" => UnfurlError::UpgradeRequired,
            "user_is_bot" => UnfurlError::UserIsBot,
            _ => UnfurlError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UnfurlError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UnfurlError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UnfurlError::CannotFindService => {
                write!(f, "Server returned error cannot_find_service")
            }
            UnfurlError::CannotPrompt => write!(f, "Server returned error cannot_prompt"),
            UnfurlError::CannotUnfurlUrl => write!(f, "Server returned error cannot_unfurl_url"),
            UnfurlError::FatalError => write!(f, "Server returned error fatal_error"),
            UnfurlError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UnfurlError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UnfurlError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UnfurlError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UnfurlError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UnfurlError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UnfurlError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UnfurlError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UnfurlError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UnfurlError::MissingUnfurls => write!(f, "Server returned error missing_unfurls"),
            UnfurlError::NoPermission => write!(f, "Server returned error no_permission"),
            UnfurlError::NotAuthed => write!(f, "Server returned error not_authed"),
            UnfurlError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            UnfurlError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UnfurlError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            UnfurlError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            UnfurlError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            UnfurlError::UserIsBot => write!(f, "Server returned error user_is_bot"),
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
pub struct UpdateRequest<'a> {
    /// Pass true to update the message as the authed user. [Bot users](/bot-users) in this context are considered authed users.
    pub as_user: Option<Cow<'a, str>>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string. This field is required when not presenting `text`. If you don't include this field, the message's previous `attachments` will be retained. To remove previous `attachments`, include an empty array for this field.
    pub attachments: Option<Cow<'a, str>>,
    /// A JSON-based array of [structured blocks](/block-kit/building), presented as a URL-encoded string. If you don't include this field, the message's previous `blocks` will be retained. To remove previous `blocks`, include an empty array for this field.
    pub blocks: Option<Cow<'a, str>>,
    /// Channel containing the message to be updated.
    pub channel: Cow<'a, str>,
    /// Find and link channel names and usernames. Defaults to `none`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `none`.
    pub link_names: Option<Cow<'a, str>>,
    /// Change how messages are treated. Defaults to `client`, unlike `chat.postMessage`. Accepts either `none` or `full`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `client`.
    pub parse: Option<Cow<'a, str>>,
    /// New text for the message, using the [default formatting rules](/reference/surfaces/formatting). It's not required when presenting `blocks` or `attachments`.
    pub text: Option<Cow<'a, str>>,
    /// Timestamp of the message to be updated.
    pub ts: Cow<'a, str>,
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
    AccountInactive,
    CantUpdateMessage,
    ChannelNotFound,
    EditWindowClosed,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    IsInactive,
    JsonNotObject,
    MessageNotFound,
    MissingPostType,
    MsgTooLong,
    NoPermission,
    NoText,
    NotAuthed,
    RateLimited,
    RequestTimeout,
    TokenRevoked,
    TooManyAttachments,
    UpgradeRequired,
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
            "account_inactive" => UpdateError::AccountInactive,
            "cant_update_message" => UpdateError::CantUpdateMessage,
            "channel_not_found" => UpdateError::ChannelNotFound,
            "edit_window_closed" => UpdateError::EditWindowClosed,
            "fatal_error" => UpdateError::FatalError,
            "invalid_arg_name" => UpdateError::InvalidArgName,
            "invalid_array_arg" => UpdateError::InvalidArrayArg,
            "invalid_auth" => UpdateError::InvalidAuth,
            "invalid_charset" => UpdateError::InvalidCharset,
            "invalid_form_data" => UpdateError::InvalidFormData,
            "invalid_json" => UpdateError::InvalidJson,
            "invalid_post_type" => UpdateError::InvalidPostType,
            "is_inactive" => UpdateError::IsInactive,
            "json_not_object" => UpdateError::JsonNotObject,
            "message_not_found" => UpdateError::MessageNotFound,
            "missing_post_type" => UpdateError::MissingPostType,
            "msg_too_long" => UpdateError::MsgTooLong,
            "no_permission" => UpdateError::NoPermission,
            "no_text" => UpdateError::NoText,
            "not_authed" => UpdateError::NotAuthed,
            "rate_limited" => UpdateError::RateLimited,
            "request_timeout" => UpdateError::RequestTimeout,
            "token_revoked" => UpdateError::TokenRevoked,
            "too_many_attachments" => UpdateError::TooManyAttachments,
            "upgrade_required" => UpdateError::UpgradeRequired,
            _ => UpdateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UpdateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UpdateError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UpdateError::CantUpdateMessage => {
                write!(f, "Server returned error cant_update_message")
            }
            UpdateError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            UpdateError::EditWindowClosed => write!(f, "Server returned error edit_window_closed"),
            UpdateError::FatalError => write!(f, "Server returned error fatal_error"),
            UpdateError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UpdateError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UpdateError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UpdateError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UpdateError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UpdateError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UpdateError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UpdateError::IsInactive => write!(f, "Server returned error is_inactive"),
            UpdateError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UpdateError::MessageNotFound => write!(f, "Server returned error message_not_found"),
            UpdateError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UpdateError::MsgTooLong => write!(f, "Server returned error msg_too_long"),
            UpdateError::NoPermission => write!(f, "Server returned error no_permission"),
            UpdateError::NoText => write!(f, "Server returned error no_text"),
            UpdateError::NotAuthed => write!(f, "Server returned error not_authed"),
            UpdateError::RateLimited => write!(f, "Server returned error rate_limited"),
            UpdateError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UpdateError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            UpdateError::TooManyAttachments => {
                write!(f, "Server returned error too_many_attachments")
            }
            UpdateError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
