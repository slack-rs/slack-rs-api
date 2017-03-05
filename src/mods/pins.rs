

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Pins an item to a channel.
///
/// Wraps https://api.slack.com/methods/pins.add

pub fn add<R>(client: &R, token: &str, request: &AddRequest) -> Result<AddResponse, AddError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      request.file.map(|file| ("file", file)),
                      request.file_comment.map(|file_comment| ("file_comment", file_comment)),
                      request.timestamp.map(|timestamp| ("timestamp", timestamp))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("pins.add", &params[..])
        .map_err(|err| AddError::Client(err))
        .and_then(|result| serde_json::from_str::<AddResponse>(&result).map_err(|_| AddError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct AddRequest<'a> {
    /// Channel to pin the item in.
    pub channel: &'a str,
    /// File to pin.
    pub file: Option<&'a str>,
    /// File comment to pin.
    pub file_comment: Option<&'a str>,
    /// Timestamp of the message to pin.
    pub timestamp: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<AddResponse, AddError<E>>> for AddResponse {
    fn into(self) -> Result<AddResponse, AddError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum AddError<E: Error> {
    /// Value passed for timestamp was invalid.
    BadTimestamp,
    /// File specified by file does not exist.
    FileNotFound,
    /// File comment specified by file_comment does not exist.
    FileCommentNotFound,
    /// Message specified by channel and timestamp does not exist.
    MessageNotFound,
    /// The channel argument was not specified or was invalid
    ChannelNotFound,
    /// One of file, file_comment, or timestamp was not specified.
    NoItemSpecified,
    /// The specified item is already pinned to the channel.
    AlreadyPinned,
    /// The user does not have permission to add pins to the channel.
    PermissionDenied,
    /// File specified by file is not public nor shared to the channel.
    FileNotShared,
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

impl<'a, E: Error> From<&'a str> for AddError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "bad_timestamp" => AddError::BadTimestamp,
            "file_not_found" => AddError::FileNotFound,
            "file_comment_not_found" => AddError::FileCommentNotFound,
            "message_not_found" => AddError::MessageNotFound,
            "channel_not_found" => AddError::ChannelNotFound,
            "no_item_specified" => AddError::NoItemSpecified,
            "already_pinned" => AddError::AlreadyPinned,
            "permission_denied" => AddError::PermissionDenied,
            "file_not_shared" => AddError::FileNotShared,
            "not_authed" => AddError::NotAuthed,
            "invalid_auth" => AddError::InvalidAuth,
            "account_inactive" => AddError::AccountInactive,
            "invalid_arg_name" => AddError::InvalidArgName,
            "invalid_array_arg" => AddError::InvalidArrayArg,
            "invalid_charset" => AddError::InvalidCharset,
            "invalid_form_data" => AddError::InvalidFormData,
            "invalid_post_type" => AddError::InvalidPostType,
            "missing_post_type" => AddError::MissingPostType,
            "request_timeout" => AddError::RequestTimeout,
            _ => AddError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for AddError<E> {
    fn description(&self) -> &str {
        match self {
            &AddError::BadTimestamp => "bad_timestamp: Value passed for timestamp was invalid.",
            &AddError::FileNotFound => "file_not_found: File specified by file does not exist.",
            &AddError::FileCommentNotFound => {
                "file_comment_not_found: File comment specified by file_comment does not exist."
            }
            &AddError::MessageNotFound => {
                "message_not_found: Message specified by channel and timestamp does not exist."
            }
            &AddError::ChannelNotFound => "channel_not_found: The channel argument was not specified or was invalid",
            &AddError::NoItemSpecified => {
                "no_item_specified: One of file, file_comment, or timestamp was not specified."
            }
            &AddError::AlreadyPinned => "already_pinned: The specified item is already pinned to the channel.",
            &AddError::PermissionDenied => {
                "permission_denied: The user does not have permission to add pins to the channel."
            }
            &AddError::FileNotShared => {
                "file_not_shared: File specified by file is not public nor shared to the channel."
            }
            &AddError::NotAuthed => "not_authed: No authentication token provided.",
            &AddError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &AddError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &AddError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common \
                 decency. This includes very long names and names with non-alphanumeric characters other than _. If \
                 you get this error, it is typically an indication that you have made a very malformed API call."
            }
            &AddError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). \
                 These are never valid with the Slack API."
            }
            &AddError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the \
                 Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            &AddError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type \
                 application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or \
                 syntactically invalid."
            }
            &AddError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was \
                 invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data \
                 text/plain."
            }
            &AddError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the \
                 request did not include a Content-Type header."
            }
            &AddError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or \
                 truncated."
            }
            &AddError::MalformedResponse => "Malformed response data from Slack.",
            &AddError::Unknown(ref s) => s,
            &AddError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &AddError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Lists items pinned to a channel.
///
/// Wraps https://api.slack.com/methods/pins.list

pub fn list<R>(client: &R, token: &str, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("pins.list", &params[..])
        .map_err(|err| ListError::Client(err))
        .and_then(|result| serde_json::from_str::<ListResponse>(&result).map_err(|_| ListError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Channel to get pinned items for.
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    error: Option<String>,
    pub items: Option<Vec<ListResponseItem>>,
    #[serde(default)]
    ok: bool,
}


#[derive(Clone, Debug)]
pub enum ListResponseItem {
    Message(ListResponseItemMessage),
    File(ListResponseItemFile),
    FileComment(ListResponseItemFileComment),
}

impl ::serde::Deserialize for ListResponseItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::Deserializer
    {
        use serde::de::Error as SerdeError;

        const VARIANTS: &'static [&'static str] = &["message", "file", "file_comment"];

        let value = ::serde_json::Value::deserialize(deserializer)?;
        if let Some(ty_val) = value.get("type") {
            if let Some(ty) = ty_val.as_str() {
                match ty {
                    "message" => {
                        ::serde_json::from_value::<ListResponseItemMessage>(value.clone())
                            .map(|obj| ListResponseItem::Message(obj))
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "file" => {
                        ::serde_json::from_value::<ListResponseItemFile>(value.clone())
                            .map(|obj| ListResponseItem::File(obj))
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "file_comment" => {
                        ::serde_json::from_value::<ListResponseItemFileComment>(value.clone())
                            .map(|obj| ListResponseItem::FileComment(obj))
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    _ => Err(D::Error::unknown_variant(ty, VARIANTS)),
                }
            } else {
                Err(D::Error::invalid_type(::serde::de::Unexpected::Unit, &"a string"))
            }
        } else {
            Err(D::Error::missing_field("type"))
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemMessage {
    pub channel: String,
    pub created: Option<f32>,
    pub created_by: Option<String>,
    pub message: ::Message,
    #[serde(rename = "type")]
    pub ty: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemFile {
    pub created: Option<f32>,
    pub created_by: Option<String>,
    pub file: ::File,
    #[serde(rename = "type")]
    pub ty: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemFileComment {
    pub comment: ::FileComment,
    pub created: Option<f32>,
    pub created_by: Option<String>,
    pub file: ::File,
    #[serde(rename = "type")]
    pub ty: String,
}


impl<E: Error> Into<Result<ListResponse, ListError<E>>> for ListResponse {
    fn into(self) -> Result<ListResponse, ListError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum ListError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
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

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => ListError::ChannelNotFound,
            "not_authed" => ListError::NotAuthed,
            "invalid_auth" => ListError::InvalidAuth,
            "account_inactive" => ListError::AccountInactive,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_array_arg" => ListError::InvalidArrayArg,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_post_type" => ListError::InvalidPostType,
            "missing_post_type" => ListError::MissingPostType,
            "request_timeout" => ListError::RequestTimeout,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for ListError<E> {
    fn description(&self) -> &str {
        match self {
            &ListError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &ListError::NotAuthed => "not_authed: No authentication token provided.",
            &ListError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &ListError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &ListError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common \
                 decency. This includes very long names and names with non-alphanumeric characters other than _. If \
                 you get this error, it is typically an indication that you have made a very malformed API call."
            }
            &ListError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). \
                 These are never valid with the Slack API."
            }
            &ListError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the \
                 Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            &ListError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type \
                 application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or \
                 syntactically invalid."
            }
            &ListError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was \
                 invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data \
                 text/plain."
            }
            &ListError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the \
                 request did not include a Content-Type header."
            }
            &ListError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or \
                 truncated."
            }
            &ListError::MalformedResponse => "Malformed response data from Slack.",
            &ListError::Unknown(ref s) => s,
            &ListError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ListError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Un-pins an item from a channel.
///
/// Wraps https://api.slack.com/methods/pins.remove

pub fn remove<R>(client: &R, token: &str, request: &RemoveRequest) -> Result<RemoveResponse, RemoveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      request.file.map(|file| ("file", file)),
                      request.file_comment.map(|file_comment| ("file_comment", file_comment)),
                      request.timestamp.map(|timestamp| ("timestamp", timestamp))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("pins.remove", &params[..])
        .map_err(|err| RemoveError::Client(err))
        .and_then(|result| serde_json::from_str::<RemoveResponse>(&result).map_err(|_| RemoveError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct RemoveRequest<'a> {
    /// Channel where the item is pinned to.
    pub channel: &'a str,
    /// File to un-pin.
    pub file: Option<&'a str>,
    /// File comment to un-pin.
    pub file_comment: Option<&'a str>,
    /// Timestamp of the message to un-pin.
    pub timestamp: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<RemoveResponse, RemoveError<E>>> for RemoveResponse {
    fn into(self) -> Result<RemoveResponse, RemoveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum RemoveError<E: Error> {
    /// Value passed for timestamp was invalid.
    BadTimestamp,
    /// File specified by file does not exist.
    FileNotFound,
    /// File comment specified by file_comment does not exist.
    FileCommentNotFound,
    /// Message specified by channel and timestamp does not exist.
    MessageNotFound,
    /// One of file, file_comment, or timestamp was not specified.
    NoItemSpecified,
    /// The specified item is not pinned to the channel.
    NotPinned,
    /// The user does not have permission to remove pins from the channel.
    PermissionDenied,
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

impl<'a, E: Error> From<&'a str> for RemoveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "bad_timestamp" => RemoveError::BadTimestamp,
            "file_not_found" => RemoveError::FileNotFound,
            "file_comment_not_found" => RemoveError::FileCommentNotFound,
            "message_not_found" => RemoveError::MessageNotFound,
            "no_item_specified" => RemoveError::NoItemSpecified,
            "not_pinned" => RemoveError::NotPinned,
            "permission_denied" => RemoveError::PermissionDenied,
            "not_authed" => RemoveError::NotAuthed,
            "invalid_auth" => RemoveError::InvalidAuth,
            "account_inactive" => RemoveError::AccountInactive,
            "invalid_arg_name" => RemoveError::InvalidArgName,
            "invalid_array_arg" => RemoveError::InvalidArrayArg,
            "invalid_charset" => RemoveError::InvalidCharset,
            "invalid_form_data" => RemoveError::InvalidFormData,
            "invalid_post_type" => RemoveError::InvalidPostType,
            "missing_post_type" => RemoveError::MissingPostType,
            "request_timeout" => RemoveError::RequestTimeout,
            _ => RemoveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RemoveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for RemoveError<E> {
    fn description(&self) -> &str {
        match self {
            &RemoveError::BadTimestamp => "bad_timestamp: Value passed for timestamp was invalid.",
            &RemoveError::FileNotFound => "file_not_found: File specified by file does not exist.",
            &RemoveError::FileCommentNotFound => {
                "file_comment_not_found: File comment specified by file_comment does not exist."
            }
            &RemoveError::MessageNotFound => {
                "message_not_found: Message specified by channel and timestamp does not exist."
            }
            &RemoveError::NoItemSpecified => {
                "no_item_specified: One of file, file_comment, or timestamp was not specified."
            }
            &RemoveError::NotPinned => "not_pinned: The specified item is not pinned to the channel.",
            &RemoveError::PermissionDenied => {
                "permission_denied: The user does not have permission to remove pins from the channel."
            }
            &RemoveError::NotAuthed => "not_authed: No authentication token provided.",
            &RemoveError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &RemoveError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &RemoveError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common \
                 decency. This includes very long names and names with non-alphanumeric characters other than _. If \
                 you get this error, it is typically an indication that you have made a very malformed API call."
            }
            &RemoveError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). \
                 These are never valid with the Slack API."
            }
            &RemoveError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the \
                 Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            &RemoveError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type \
                 application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or \
                 syntactically invalid."
            }
            &RemoveError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was \
                 invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data \
                 text/plain."
            }
            &RemoveError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the \
                 request did not include a Content-Type header."
            }
            &RemoveError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or \
                 truncated."
            }
            &RemoveError::MalformedResponse => "Malformed response data from Slack.",
            &RemoveError::Unknown(ref s) => s,
            &RemoveError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &RemoveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
