
#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Adds a star to an item.
///
/// Wraps https://api.slack.com/methods/stars.add

pub fn add<R>(client: &R, request: &AddRequest) -> Result<AddResponse, AddError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      request.file.map(|file| ("file", file)),
                      request.file_comment.map(|file_comment| ("file_comment", file_comment)),
                      request.channel.map(|channel| ("channel", channel)),
                      request.timestamp.map(|timestamp| ("timestamp", timestamp))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("stars.add", &params[..])
        .map_err(|err| AddError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result).map_err(|_| AddError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct AddRequest<'a> {
    /// Authentication token.
    /// Requires scope: stars:write
    pub token: &'a str,
    /// File to add star to.
    pub file: Option<&'a str>,
    /// File comment to add star to.
    pub file_comment: Option<&'a str>,
    /// Channel to add star to, or channel where the message to add star to was posted (used with timestamp).
    pub channel: Option<&'a str>,
    /// Timestamp of the message to add star to.
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
    /// Message specified by channel and timestamp does not exist.
    MessageNotFound,
    /// File specified by file does not exist.
    FileNotFound,
    /// File comment specified by file_comment does not exist.
    FileCommentNotFound,
    /// Channel, private group, or DM specified by channel does not exist
    ChannelNotFound,
    /// file, file_comment, channel and timestamp was not specified.
    NoItemSpecified,
    /// The specified item has already been starred by the authenticated user.
    AlreadyStarred,
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
            "message_not_found" => AddError::MessageNotFound,
            "file_not_found" => AddError::FileNotFound,
            "file_comment_not_found" => AddError::FileCommentNotFound,
            "channel_not_found" => AddError::ChannelNotFound,
            "no_item_specified" => AddError::NoItemSpecified,
            "already_starred" => AddError::AlreadyStarred,
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
            &AddError::BadTimestamp => "bad_timestamp",
            &AddError::MessageNotFound => "message_not_found",
            &AddError::FileNotFound => "file_not_found",
            &AddError::FileCommentNotFound => "file_comment_not_found",
            &AddError::ChannelNotFound => "channel_not_found",
            &AddError::NoItemSpecified => "no_item_specified",
            &AddError::AlreadyStarred => "already_starred",
            &AddError::NotAuthed => "not_authed",
            &AddError::InvalidAuth => "invalid_auth",
            &AddError::AccountInactive => "account_inactive",
            &AddError::InvalidArgName => "invalid_arg_name",
            &AddError::InvalidArrayArg => "invalid_array_arg",
            &AddError::InvalidCharset => "invalid_charset",
            &AddError::InvalidFormData => "invalid_form_data",
            &AddError::InvalidPostType => "invalid_post_type",
            &AddError::MissingPostType => "missing_post_type",
            &AddError::RequestTimeout => "request_timeout",
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

/// Lists stars for a user.
///
/// Wraps https://api.slack.com/methods/stars.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
    where R: SlackWebRequestSender
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![Some(("token", request.token)),
                      count.as_ref().map(|count| ("count", &count[..])),
                      page.as_ref().map(|page| ("page", &page[..]))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("stars.list", &params[..])
        .map_err(|err| ListError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(|_| ListError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Authentication token.
    /// Requires scope: stars:read
    pub token: &'a str,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    error: Option<String>,
    pub items: Option<Vec<ListResponseItem>>,
    #[serde(default)]
    ok: bool,
    pub paging: Option<::Paging>,
}


#[derive(Clone, Debug)]
pub enum ListResponseItem {
    Message(ListResponseItemMessage),
    File(ListResponseItemFile),
    FileComment(ListResponseItemFileComment),
    Channel(ListResponseItemChannel),
    Im(ListResponseItemIm),
    Group(ListResponseItemGroup),
}

impl ::serde::Deserialize for ListResponseItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::Deserializer
    {
        use serde::de::Error as SerdeError;

        const VARIANTS: &'static [&'static str] =
            &["message", "file", "file_comment", "channel", "im", "group"];

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
                    "channel" => {
                        ::serde_json::from_value::<ListResponseItemChannel>(value.clone())
                            .map(|obj| ListResponseItem::Channel(obj))
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "im" => {
                        ::serde_json::from_value::<ListResponseItemIm>(value.clone())
                            .map(|obj| ListResponseItem::Im(obj))
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "group" => {
                        ::serde_json::from_value::<ListResponseItemGroup>(value.clone())
                            .map(|obj| ListResponseItem::Group(obj))
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
    pub message: ::Message,
    #[serde(rename = "type")]
    pub ty: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemFile {
    pub file: ::File,
    #[serde(rename = "type")]
    pub ty: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemFileComment {
    pub comment: ::FileComment,
    pub file: ::File,
    #[serde(rename = "type")]
    pub ty: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemChannel {
    pub channel: String,
    #[serde(rename = "type")]
    pub ty: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemIm {
    pub channel: String,
    #[serde(rename = "type")]
    pub ty: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemGroup {
    pub group: String,
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
    /// Value passed for user was invalid
    UserNotFound,
    /// The requested user is not visible to the calling user
    UserNotVisible,
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
            "user_not_found" => ListError::UserNotFound,
            "user_not_visible" => ListError::UserNotVisible,
            "not_authed" => ListError::NotAuthed,
            "invalid_auth" => ListError::InvalidAuth,
            "account_inactive" => ListError::AccountInactive,
            "user_is_bot" => ListError::UserIsBot,
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
            &ListError::UserNotFound => "user_not_found",
            &ListError::UserNotVisible => "user_not_visible",
            &ListError::NotAuthed => "not_authed",
            &ListError::InvalidAuth => "invalid_auth",
            &ListError::AccountInactive => "account_inactive",
            &ListError::UserIsBot => "user_is_bot",
            &ListError::InvalidArgName => "invalid_arg_name",
            &ListError::InvalidArrayArg => "invalid_array_arg",
            &ListError::InvalidCharset => "invalid_charset",
            &ListError::InvalidFormData => "invalid_form_data",
            &ListError::InvalidPostType => "invalid_post_type",
            &ListError::MissingPostType => "missing_post_type",
            &ListError::RequestTimeout => "request_timeout",
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

/// Removes a star from an item.
///
/// Wraps https://api.slack.com/methods/stars.remove

pub fn remove<R>(client: &R,
                 request: &RemoveRequest)
                 -> Result<RemoveResponse, RemoveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      request.file.map(|file| ("file", file)),
                      request.file_comment.map(|file_comment| ("file_comment", file_comment)),
                      request.channel.map(|channel| ("channel", channel)),
                      request.timestamp.map(|timestamp| ("timestamp", timestamp))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("stars.remove", &params[..])
        .map_err(|err| RemoveError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|_| RemoveError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct RemoveRequest<'a> {
    /// Authentication token.
    /// Requires scope: stars:write
    pub token: &'a str,
    /// File to remove star from.
    pub file: Option<&'a str>,
    /// File comment to remove star from.
    pub file_comment: Option<&'a str>,
    /// Channel to remove star from, or channel where the message to remove star from was posted (used with timestamp).
    pub channel: Option<&'a str>,
    /// Timestamp of the message to remove star from.
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
    /// Message specified by channel and timestamp does not exist.
    MessageNotFound,
    /// File specified by file does not exist.
    FileNotFound,
    /// File comment specified by file_comment does not exist.
    FileCommentNotFound,
    /// Channel, private group, or DM specified by channel does not exist
    ChannelNotFound,
    /// file, file_comment, channel and timestamp was not specified.
    NoItemSpecified,
    /// The specified item is not currently starred by the authenticated user.
    NotStarred,
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
            "message_not_found" => RemoveError::MessageNotFound,
            "file_not_found" => RemoveError::FileNotFound,
            "file_comment_not_found" => RemoveError::FileCommentNotFound,
            "channel_not_found" => RemoveError::ChannelNotFound,
            "no_item_specified" => RemoveError::NoItemSpecified,
            "not_starred" => RemoveError::NotStarred,
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
            &RemoveError::BadTimestamp => "bad_timestamp",
            &RemoveError::MessageNotFound => "message_not_found",
            &RemoveError::FileNotFound => "file_not_found",
            &RemoveError::FileCommentNotFound => "file_comment_not_found",
            &RemoveError::ChannelNotFound => "channel_not_found",
            &RemoveError::NoItemSpecified => "no_item_specified",
            &RemoveError::NotStarred => "not_starred",
            &RemoveError::NotAuthed => "not_authed",
            &RemoveError::InvalidAuth => "invalid_auth",
            &RemoveError::AccountInactive => "account_inactive",
            &RemoveError::InvalidArgName => "invalid_arg_name",
            &RemoveError::InvalidArrayArg => "invalid_array_arg",
            &RemoveError::InvalidCharset => "invalid_charset",
            &RemoveError::InvalidFormData => "invalid_form_data",
            &RemoveError::InvalidPostType => "invalid_post_type",
            &RemoveError::MissingPostType => "missing_post_type",
            &RemoveError::RequestTimeout => "request_timeout",
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
