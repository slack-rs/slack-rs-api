
#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Deletes a message.
///
/// Wraps https://api.slack.com/methods/chat.delete

pub fn delete<R>(client: &R,
                 request: &DeleteRequest)
                 -> Result<DeleteResponse, DeleteError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      Some(("ts", request.ts)),
                      Some(("channel", request.channel)),
                      request.as_user.map(|as_user| ("as_user", if as_user { "1" } else { "0" }))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("chat.delete", &params[..])
        .map_err(|err| DeleteError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|_| DeleteError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest<'a> {
    /// Authentication token.
    /// Requires scope: chat:write:bot or chat:write:user
    pub token: &'a str,
    /// Timestamp of the message to be deleted.
    pub ts: &'a str,
    /// Channel containing the message to be deleted.
    pub channel: &'a str,
    /// Pass true to delete the message as the authed user. Bot users in this context are considered authed users.
    pub as_user: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteResponse {
    pub channel: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub ts: Option<String>,
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
#[derive(Clone, Debug)]
pub enum DeleteError<E: Error> {
    /// No message exists with the requested timestamp.
    MessageNotFound,
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Authenticated user does not have permission to delete this message.
    CantDeleteMessage,
    /// Compliance exports are on, messages can not be deleted
    ComplianceExportsPreventDeletion,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse,
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
            "invalid_arg_name" => DeleteError::InvalidArgName,
            "invalid_array_arg" => DeleteError::InvalidArrayArg,
            "invalid_charset" => DeleteError::InvalidCharset,
            "invalid_form_data" => DeleteError::InvalidFormData,
            "invalid_post_type" => DeleteError::InvalidPostType,
            "missing_post_type" => DeleteError::MissingPostType,
            "request_timeout" => DeleteError::RequestTimeout,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for DeleteError<E> {
    fn description(&self) -> &str {
        match self {
            &DeleteError::MessageNotFound => "message_not_found",
            &DeleteError::ChannelNotFound => "channel_not_found",
            &DeleteError::CantDeleteMessage => "cant_delete_message",
            &DeleteError::ComplianceExportsPreventDeletion => "compliance_exports_prevent_deletion",
            &DeleteError::NotAuthed => "not_authed",
            &DeleteError::InvalidAuth => "invalid_auth",
            &DeleteError::AccountInactive => "account_inactive",
            &DeleteError::InvalidArgName => "invalid_arg_name",
            &DeleteError::InvalidArrayArg => "invalid_array_arg",
            &DeleteError::InvalidCharset => "invalid_charset",
            &DeleteError::InvalidFormData => "invalid_form_data",
            &DeleteError::InvalidPostType => "invalid_post_type",
            &DeleteError::MissingPostType => "missing_post_type",
            &DeleteError::RequestTimeout => "request_timeout",
            &DeleteError::MalformedResponse => "Malformed response data from Slack.",
            &DeleteError::Unknown(ref s) => s,
            &DeleteError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &DeleteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Share a me message into a channel.
///
/// Wraps https://api.slack.com/methods/chat.meMessage

pub fn me_message<R>(client: &R,
                     request: &MeMessageRequest)
                     -> Result<MeMessageResponse, MeMessageError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      Some(("channel", request.channel)),
                      Some(("text", request.text))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("chat.meMessage", &params[..])
        .map_err(|err| MeMessageError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<MeMessageResponse>(&result)
                .map_err(|_| MeMessageError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct MeMessageRequest<'a> {
    /// Authentication token.
    /// Requires scope: chat:write:user
    pub token: &'a str,
    /// Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name.
    pub channel: &'a str,
    /// Text of the message to send.
    pub text: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MeMessageResponse {
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
#[derive(Clone, Debug)]
pub enum MeMessageError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Cannot post user messages to a channel they are not in.
    NotInChannel,
    /// Channel has been archived.
    IsArchived,
    /// Message text is too long
    MsgTooLong,
    /// No message text provided
    NoText,
    /// Application has posted too many messages, read the Rate Limit documentation for more information
    RateLimited,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse,
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
            "invalid_arg_name" => MeMessageError::InvalidArgName,
            "invalid_array_arg" => MeMessageError::InvalidArrayArg,
            "invalid_charset" => MeMessageError::InvalidCharset,
            "invalid_form_data" => MeMessageError::InvalidFormData,
            "invalid_post_type" => MeMessageError::InvalidPostType,
            "missing_post_type" => MeMessageError::MissingPostType,
            "request_timeout" => MeMessageError::RequestTimeout,
            _ => MeMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MeMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for MeMessageError<E> {
    fn description(&self) -> &str {
        match self {
            &MeMessageError::ChannelNotFound => "channel_not_found",
            &MeMessageError::NotInChannel => "not_in_channel",
            &MeMessageError::IsArchived => "is_archived",
            &MeMessageError::MsgTooLong => "msg_too_long",
            &MeMessageError::NoText => "no_text",
            &MeMessageError::RateLimited => "rate_limited",
            &MeMessageError::NotAuthed => "not_authed",
            &MeMessageError::InvalidAuth => "invalid_auth",
            &MeMessageError::AccountInactive => "account_inactive",
            &MeMessageError::InvalidArgName => "invalid_arg_name",
            &MeMessageError::InvalidArrayArg => "invalid_array_arg",
            &MeMessageError::InvalidCharset => "invalid_charset",
            &MeMessageError::InvalidFormData => "invalid_form_data",
            &MeMessageError::InvalidPostType => "invalid_post_type",
            &MeMessageError::MissingPostType => "missing_post_type",
            &MeMessageError::RequestTimeout => "request_timeout",
            &MeMessageError::MalformedResponse => "Malformed response data from Slack.",
            &MeMessageError::Unknown(ref s) => s,
            &MeMessageError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &MeMessageError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Sends a message to a channel.
///
/// Wraps https://api.slack.com/methods/chat.postMessage

pub fn post_message<R>(client: &R,
                       request: &PostMessageRequest)
                       -> Result<PostMessageResponse, PostMessageError<R::Error>>
    where R: SlackWebRequestSender
{

    let params =
        vec![Some(("token", request.token)),
             Some(("channel", request.channel)),
             Some(("text", request.text)),
             request.parse.map(|parse| ("parse", parse)),
             request.link_names
                 .map(|link_names| ("link_names", if link_names { "1" } else { "0" })),
             request.attachments.map(|attachments| ("attachments", attachments)),
             request.unfurl_links
                 .map(|unfurl_links| ("unfurl_links", if unfurl_links { "1" } else { "0" })),
             request.unfurl_media
                 .map(|unfurl_media| ("unfurl_media", if unfurl_media { "1" } else { "0" })),
             request.username.map(|username| ("username", username)),
             request.as_user.map(|as_user| ("as_user", if as_user { "1" } else { "0" })),
             request.icon_url.map(|icon_url| ("icon_url", icon_url)),
             request.icon_emoji.map(|icon_emoji| ("icon_emoji", icon_emoji)),
             request.thread_ts.map(|thread_ts| ("thread_ts", thread_ts)),
             request.reply_broadcast.map(|reply_broadcast| {
                 ("reply_broadcast", if reply_broadcast { "1" } else { "0" })
             })];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("chat.postMessage", &params[..])
        .map_err(|err| PostMessageError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<PostMessageResponse>(&result)
                .map_err(|_| PostMessageError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct PostMessageRequest<'a> {
    /// Authentication token.
    /// Requires scope: chat:write:bot or chat:write:user
    pub token: &'a str,
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See below for more details.
    pub channel: &'a str,
    /// Text of the message to send. See below for an explanation of formatting. This field is usually required, unless you're providing only attachments instead.
    pub text: &'a str,
    /// Change how messages are treated. Defaults to none. See below.
    pub parse: Option<&'a str>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Structured message attachments.
    pub attachments: Option<&'a str>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
    /// Set your bot's user name. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub username: Option<&'a str>,
    /// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See authorship below.
    pub as_user: Option<bool>,
    /// URL to an image to use as the icon for this message. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub icon_url: Option<&'a str>,
    /// Emoji to use as the icon for this message. Overrides icon_url. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub icon_emoji: Option<&'a str>,
    /// Provide another message's ts value to make this message a reply. Avoid using a reply's ts value; use its parent instead.
    pub thread_ts: Option<&'a str>,
    /// Used in conjunction with thread_ts and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to false.
    pub reply_broadcast: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageResponse {
    pub channel: Option<String>,
    error: Option<String>,
    pub message: Option<::Message>,
    #[serde(default)]
    ok: bool,
    pub ts: Option<String>,
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
#[derive(Clone, Debug)]
pub enum PostMessageError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Cannot post user messages to a channel they are not in.
    NotInChannel,
    /// Channel has been archived.
    IsArchived,
    /// Message text is too long
    MsgTooLong,
    /// No message text provided
    NoText,
    /// Too many attachments were provided with this message. A maximum of 100 attachments are allowed on a message.
    TooManyAttachments,
    /// Application has posted too many messages, read the Rate Limit documentation for more information
    RateLimited,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse,
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
            "request_timeout" => PostMessageError::RequestTimeout,
            _ => PostMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for PostMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for PostMessageError<E> {
    fn description(&self) -> &str {
        match self {
            &PostMessageError::ChannelNotFound => "channel_not_found",
            &PostMessageError::NotInChannel => "not_in_channel",
            &PostMessageError::IsArchived => "is_archived",
            &PostMessageError::MsgTooLong => "msg_too_long",
            &PostMessageError::NoText => "no_text",
            &PostMessageError::TooManyAttachments => "too_many_attachments",
            &PostMessageError::RateLimited => "rate_limited",
            &PostMessageError::NotAuthed => "not_authed",
            &PostMessageError::InvalidAuth => "invalid_auth",
            &PostMessageError::AccountInactive => "account_inactive",
            &PostMessageError::InvalidArgName => "invalid_arg_name",
            &PostMessageError::InvalidArrayArg => "invalid_array_arg",
            &PostMessageError::InvalidCharset => "invalid_charset",
            &PostMessageError::InvalidFormData => "invalid_form_data",
            &PostMessageError::InvalidPostType => "invalid_post_type",
            &PostMessageError::MissingPostType => "missing_post_type",
            &PostMessageError::RequestTimeout => "request_timeout",
            &PostMessageError::MalformedResponse => "Malformed response data from Slack.",
            &PostMessageError::Unknown(ref s) => s,
            &PostMessageError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &PostMessageError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Updates a message.
///
/// Wraps https://api.slack.com/methods/chat.update

pub fn update<R>(client: &R,
                 request: &UpdateRequest)
                 -> Result<UpdateResponse, UpdateError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      Some(("ts", request.ts)),
                      Some(("channel", request.channel)),
                      Some(("text", request.text)),
                      request.attachments.map(|attachments| ("attachments", attachments)),
                      request.parse.map(|parse| ("parse", parse)),
                      request.link_names
                          .map(|link_names| ("link_names", if link_names { "1" } else { "0" })),
                      request.as_user.map(|as_user| ("as_user", if as_user { "1" } else { "0" }))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("chat.update", &params[..])
        .map_err(|err| UpdateError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|_| UpdateError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct UpdateRequest<'a> {
    /// Authentication token.
    /// Requires scope: chat:write:bot or chat:write:user
    pub token: &'a str,
    /// Timestamp of the message to be updated.
    pub ts: &'a str,
    /// Channel containing the message to be updated.
    pub channel: &'a str,
    /// New text for the message, using the default formatting rules.
    pub text: &'a str,
    /// Structured message attachments.
    pub attachments: Option<&'a str>,
    /// Change how messages are treated. Defaults to client, unlike chat.postMessage. See below.
    pub parse: Option<&'a str>,
    /// Find and link channel names and usernames. Defaults to none. This parameter should be used in conjunction with parse. To set link_names to 1, specify a parse mode of full.
    pub link_names: Option<bool>,
    /// Pass true to update the message as the authed user. Bot users in this context are considered authed users.
    pub as_user: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateResponse {
    pub channel: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub text: Option<String>,
    pub ts: Option<String>,
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
#[derive(Clone, Debug)]
pub enum UpdateError<E: Error> {
    /// No message exists with the requested timestamp.
    MessageNotFound,
    /// Authenticated user does not have permission to update this message.
    CantUpdateMessage,
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// The message cannot be edited due to the team message edit settings
    EditWindowClosed,
    /// Message text is too long
    MsgTooLong,
    /// Too many attachments were provided with this message. A maximum of 100 attachments are allowed on a message.
    TooManyAttachments,
    /// No message text provided
    NoText,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse,
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
            "no_text" => UpdateError::NoText,
            "not_authed" => UpdateError::NotAuthed,
            "invalid_auth" => UpdateError::InvalidAuth,
            "account_inactive" => UpdateError::AccountInactive,
            "invalid_arg_name" => UpdateError::InvalidArgName,
            "invalid_array_arg" => UpdateError::InvalidArrayArg,
            "invalid_charset" => UpdateError::InvalidCharset,
            "invalid_form_data" => UpdateError::InvalidFormData,
            "invalid_post_type" => UpdateError::InvalidPostType,
            "missing_post_type" => UpdateError::MissingPostType,
            "request_timeout" => UpdateError::RequestTimeout,
            _ => UpdateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UpdateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for UpdateError<E> {
    fn description(&self) -> &str {
        match self {
            &UpdateError::MessageNotFound => "message_not_found",
            &UpdateError::CantUpdateMessage => "cant_update_message",
            &UpdateError::ChannelNotFound => "channel_not_found",
            &UpdateError::EditWindowClosed => "edit_window_closed",
            &UpdateError::MsgTooLong => "msg_too_long",
            &UpdateError::TooManyAttachments => "too_many_attachments",
            &UpdateError::NoText => "no_text",
            &UpdateError::NotAuthed => "not_authed",
            &UpdateError::InvalidAuth => "invalid_auth",
            &UpdateError::AccountInactive => "account_inactive",
            &UpdateError::InvalidArgName => "invalid_arg_name",
            &UpdateError::InvalidArrayArg => "invalid_array_arg",
            &UpdateError::InvalidCharset => "invalid_charset",
            &UpdateError::InvalidFormData => "invalid_form_data",
            &UpdateError::InvalidPostType => "invalid_post_type",
            &UpdateError::MissingPostType => "missing_post_type",
            &UpdateError::RequestTimeout => "request_timeout",
            &UpdateError::MalformedResponse => "Malformed response data from Slack.",
            &UpdateError::Unknown(ref s) => s,
            &UpdateError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &UpdateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
