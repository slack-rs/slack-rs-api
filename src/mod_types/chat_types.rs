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

//! Post chat messages to Slack.

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest<'a> {
    /// Timestamp of the message to be deleted.
    pub ts: crate::Timestamp,
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
    pub ts: Option<crate::Timestamp>,
}

impl<E: Error> From<DeleteResponse> for Result<DeleteResponse, DeleteError<E>> {
    fn from(val: DeleteResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
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
            "invalid_arg_name" => DeleteError::InvalidArgName,
            "invalid_array_arg" => DeleteError::InvalidArrayArg,
            "invalid_charset" => DeleteError::InvalidCharset,
            "invalid_form_data" => DeleteError::InvalidFormData,
            "invalid_post_type" => DeleteError::InvalidPostType,
            "missing_post_type" => DeleteError::MissingPostType,
            "team_added_to_org" => DeleteError::TeamAddedToOrg,
            "request_timeout" => DeleteError::RequestTimeout,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        DeleteError::MessageNotFound => "message_not_found: No message exists with the requested timestamp.",
DeleteError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
DeleteError::CantDeleteMessage => "cant_delete_message: Authenticated user does not have permission to delete this message.",
DeleteError::ComplianceExportsPreventDeletion => "compliance_exports_prevent_deletion: Compliance exports are on, messages can not be deleted",
DeleteError::NotAuthed => "not_authed: No authentication token provided.",
DeleteError::InvalidAuth => "invalid_auth: Invalid authentication token.",
DeleteError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
DeleteError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
DeleteError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
DeleteError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
DeleteError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
DeleteError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
DeleteError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
DeleteError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
DeleteError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        DeleteError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        DeleteError::Unknown(ref s) => return write!(f, "{}", s),
                        DeleteError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
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
pub struct MeMessageRequest<'a> {
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
    pub ts: Option<crate::Timestamp>,
}

impl<E: Error> From<MeMessageResponse> for Result<MeMessageResponse, MeMessageError<E>> {
    fn from(val: MeMessageResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
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
            "invalid_arg_name" => MeMessageError::InvalidArgName,
            "invalid_array_arg" => MeMessageError::InvalidArrayArg,
            "invalid_charset" => MeMessageError::InvalidCharset,
            "invalid_form_data" => MeMessageError::InvalidFormData,
            "invalid_post_type" => MeMessageError::InvalidPostType,
            "missing_post_type" => MeMessageError::MissingPostType,
            "team_added_to_org" => MeMessageError::TeamAddedToOrg,
            "request_timeout" => MeMessageError::RequestTimeout,
            _ => MeMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MeMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        MeMessageError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
MeMessageError::NotInChannel => "not_in_channel: Cannot post user messages to a channel they are not in.",
MeMessageError::IsArchived => "is_archived: Channel has been archived.",
MeMessageError::MsgTooLong => "msg_too_long: Message text is too long",
MeMessageError::NoText => "no_text: No message text provided",
MeMessageError::RateLimited => "rate_limited: Application has posted too many messages, read the Rate Limit documentation for more information",
MeMessageError::NotAuthed => "not_authed: No authentication token provided.",
MeMessageError::InvalidAuth => "invalid_auth: Invalid authentication token.",
MeMessageError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
MeMessageError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
MeMessageError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
MeMessageError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
MeMessageError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
MeMessageError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
MeMessageError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
MeMessageError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
MeMessageError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        MeMessageError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        MeMessageError::Unknown(ref s) => return write!(f, "{}", s),
                        MeMessageError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
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
pub struct PostMessageRequest<'a> {
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
    pub thread_ts: Option<crate::Timestamp>,
    /// Used in conjunction with thread_ts and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to false.
    pub reply_broadcast: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageResponse {
    pub channel: Option<String>,
    error: Option<String>,
    pub message: Option<crate::Message>,
    #[serde(default)]
    ok: bool,
    pub ts: Option<crate::Timestamp>,
}

impl<E: Error> From<PostMessageResponse> for Result<PostMessageResponse, PostMessageError<E>> {
    fn from(val: PostMessageResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
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
            "team_added_to_org" => PostMessageError::TeamAddedToOrg,
            "request_timeout" => PostMessageError::RequestTimeout,
            _ => PostMessageError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for PostMessageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        PostMessageError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
PostMessageError::NotInChannel => "not_in_channel: Cannot post user messages to a channel they are not in.",
PostMessageError::IsArchived => "is_archived: Channel has been archived.",
PostMessageError::MsgTooLong => "msg_too_long: Message text is too long",
PostMessageError::NoText => "no_text: No message text provided",
PostMessageError::TooManyAttachments => "too_many_attachments: Too many attachments were provided with this message. A maximum of 100 attachments are allowed on a message.",
PostMessageError::RateLimited => "rate_limited: Application has posted too many messages, read the Rate Limit documentation for more information",
PostMessageError::NotAuthed => "not_authed: No authentication token provided.",
PostMessageError::InvalidAuth => "invalid_auth: Invalid authentication token.",
PostMessageError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
PostMessageError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
PostMessageError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
PostMessageError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
PostMessageError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
PostMessageError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
PostMessageError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
PostMessageError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
PostMessageError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        PostMessageError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        PostMessageError::Unknown(ref s) => return write!(f, "{}", s),
                        PostMessageError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
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
pub struct UnfurlRequest<'a> {
    /// Channel ID of the message
    pub channel: &'a str,
    /// Timestamp of the message to add unfurl behavior to
    pub ts: &'a str,
    /// JSON mapping a set of URLs from the message to their unfurl attachments
    pub unfurls: &'a str,
    /// Set to true or 1 to indicate the user must install your Slack app to trigger unfurls for this domain
    pub user_auth_required: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UnfurlResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> From<UnfurlResponse> for Result<UnfurlResponse, UnfurlError<E>> {
    fn from(val: UnfurlResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum UnfurlError<E: Error> {
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
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
            "not_authed" => UnfurlError::NotAuthed,
            "invalid_auth" => UnfurlError::InvalidAuth,
            "account_inactive" => UnfurlError::AccountInactive,
            "user_is_bot" => UnfurlError::UserIsBot,
            "invalid_arg_name" => UnfurlError::InvalidArgName,
            "invalid_array_arg" => UnfurlError::InvalidArrayArg,
            "invalid_charset" => UnfurlError::InvalidCharset,
            "invalid_form_data" => UnfurlError::InvalidFormData,
            "invalid_post_type" => UnfurlError::InvalidPostType,
            "missing_post_type" => UnfurlError::MissingPostType,
            "team_added_to_org" => UnfurlError::TeamAddedToOrg,
            "request_timeout" => UnfurlError::RequestTimeout,
            _ => UnfurlError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UnfurlError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        UnfurlError::NotAuthed => "not_authed: No authentication token provided.",
UnfurlError::InvalidAuth => "invalid_auth: Invalid authentication token.",
UnfurlError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
UnfurlError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
UnfurlError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
UnfurlError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
UnfurlError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
UnfurlError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
UnfurlError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
UnfurlError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
UnfurlError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
UnfurlError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        UnfurlError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        UnfurlError::Unknown(ref s) => return write!(f, "{}", s),
                        UnfurlError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
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
    /// Timestamp of the message to be updated.
    pub ts: crate::Timestamp,
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
    pub ts: Option<crate::Timestamp>,
}

impl<E: Error> From<UpdateResponse> for Result<UpdateResponse, UpdateError<E>> {
    fn from(val: UpdateResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
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
            "team_added_to_org" => UpdateError::TeamAddedToOrg,
            "request_timeout" => UpdateError::RequestTimeout,
            _ => UpdateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UpdateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        UpdateError::MessageNotFound => "message_not_found: No message exists with the requested timestamp.",
UpdateError::CantUpdateMessage => "cant_update_message: Authenticated user does not have permission to update this message.",
UpdateError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
UpdateError::EditWindowClosed => "edit_window_closed: The message cannot be edited due to the team message edit settings",
UpdateError::MsgTooLong => "msg_too_long: Message text is too long",
UpdateError::TooManyAttachments => "too_many_attachments: Too many attachments were provided with this message. A maximum of 100 attachments are allowed on a message.",
UpdateError::NoText => "no_text: No message text provided",
UpdateError::NotAuthed => "not_authed: No authentication token provided.",
UpdateError::InvalidAuth => "invalid_auth: Invalid authentication token.",
UpdateError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
UpdateError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
UpdateError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
UpdateError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
UpdateError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
UpdateError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
UpdateError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
UpdateError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
UpdateError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        UpdateError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        UpdateError::Unknown(ref s) => return write!(f, "{}", s),
                        UpdateError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
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
