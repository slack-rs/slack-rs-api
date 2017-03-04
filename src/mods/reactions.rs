
#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

#[allow(unused_imports)]
use ToResult;
use requests::SlackWebRequestSender;

/// Adds a reaction to an item.
///
/// Wraps https://api.slack.com/methods/reactions.add

pub fn add<R>(client: &R, request: &AddRequest) -> Result<AddResponse, AddError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      Some(("name", request.name)),
                      request.file.map(|file| ("file", file)),
                      request.file_comment.map(|file_comment| ("file_comment", file_comment)),
                      request.channel.map(|channel| ("channel", channel)),
                      request.timestamp.map(|timestamp| ("timestamp", timestamp))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("reactions.add", &params[..])
        .map_err(|err| AddError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result).map_err(|_| AddError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct AddRequest<'a> {
    /// Authentication token.
    /// Requires scope: reactions:write
    pub token: &'a str,
    /// Reaction (emoji) name.
    pub name: &'a str,
    /// File to add reaction to.
    pub file: Option<&'a str>,
    /// File comment to add reaction to.
    pub file_comment: Option<&'a str>,
    /// Channel where the message to add reaction to was posted.
    pub channel: Option<&'a str>,
    /// Timestamp of the message to add reaction to.
    pub timestamp: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> ToResult<AddResponse, AddError<E>> for AddResponse {
    fn to_result(self) -> Result<AddResponse, AddError<E>> {
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
    /// file, file_comment, or combination of channel and timestamp was not specified.
    NoItemSpecified,
    /// Value passed for name was invalid.
    InvalidName,
    /// The specified item already has the user/reaction combination.
    AlreadyReacted,
    /// The limit for distinct reactions (i.e emoji) on the item has been reached.
    TooManyEmoji,
    /// The limit for reactions a person may add to the item has been reached.
    TooManyReactions,
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
            "no_item_specified" => AddError::NoItemSpecified,
            "invalid_name" => AddError::InvalidName,
            "already_reacted" => AddError::AlreadyReacted,
            "too_many_emoji" => AddError::TooManyEmoji,
            "too_many_reactions" => AddError::TooManyReactions,
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
            &AddError::FileNotFound => "file_not_found",
            &AddError::FileCommentNotFound => "file_comment_not_found",
            &AddError::MessageNotFound => "message_not_found",
            &AddError::NoItemSpecified => "no_item_specified",
            &AddError::InvalidName => "invalid_name",
            &AddError::AlreadyReacted => "already_reacted",
            &AddError::TooManyEmoji => "too_many_emoji",
            &AddError::TooManyReactions => "too_many_reactions",
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

/// Gets reactions for an item.
///
/// Wraps https://api.slack.com/methods/reactions.get

pub fn get<R>(client: &R, request: &GetRequest) -> Result<GetResponse, GetError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      request.file.map(|file| ("file", file)),
                      request.file_comment.map(|file_comment| ("file_comment", file_comment)),
                      request.channel.map(|channel| ("channel", channel)),
                      request.timestamp.map(|timestamp| ("timestamp", timestamp)),
                      request.full.map(|full| ("full", if full { "1" } else { "0" }))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("reactions.get", &params[..])
        .map_err(|err| GetError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result).map_err(|_| GetError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct GetRequest<'a> {
    /// Authentication token.
    /// Requires scope: reactions:read
    pub token: &'a str,
    /// File to get reactions for.
    pub file: Option<&'a str>,
    /// File comment to get reactions for.
    pub file_comment: Option<&'a str>,
    /// Channel where the message to get reactions for was posted.
    pub channel: Option<&'a str>,
    /// Timestamp of the message to get reactions for.
    pub timestamp: Option<&'a str>,
    /// If true always return the complete reaction list.
    pub full: Option<bool>,
}


#[derive(Clone, Debug)]
pub enum GetResponse {
    Message(GetResponseMessage),
    File(GetResponseFile),
    FileComment(GetResponseFileComment),
}

impl ::serde::Deserialize for GetResponse {
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
                        ::serde_json::from_value::<GetResponseMessage>(value.clone())
                            .map(|obj| GetResponse::Message(obj))
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "file" => {
                        ::serde_json::from_value::<GetResponseFile>(value.clone())
                            .map(|obj| GetResponse::File(obj))
                            .map_err(|e| D::Error::custom(&format!("{}", e)))
                    }
                    "file_comment" => {
                        ::serde_json::from_value::<GetResponseFileComment>(value.clone())
                            .map(|obj| GetResponse::FileComment(obj))
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
pub struct GetResponseMessage {
    pub channel: String,
    error: Option<String>,
    pub message: ::Message,
    #[serde(default)]
    ok: bool,
    #[serde(rename = "type")]
    pub ty: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct GetResponseFile {
    error: Option<String>,
    pub file: ::File,
    #[serde(default)]
    ok: bool,
    #[serde(rename = "type")]
    pub ty: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct GetResponseFileComment {
    pub comment: ::FileComment,
    error: Option<String>,
    pub file: ::File,
    #[serde(default)]
    ok: bool,
    #[serde(rename = "type")]
    pub ty: String,
}


impl<E: Error> ToResult<GetResponse, GetError<E>> for GetResponse {
    fn to_result(self) -> Result<GetResponse, GetError<E>> {
        match self {
            GetResponse::Message(inner) => inner.to_result().map(|r| GetResponse::Message(r)),
            GetResponse::File(inner) => inner.to_result().map(|r| GetResponse::File(r)),
            GetResponse::FileComment(inner) => {
                inner.to_result().map(|r| GetResponse::FileComment(r))
            }
        }
    }
}

impl<E: Error> ToResult<GetResponseMessage, GetError<E>> for GetResponseMessage {
    fn to_result(self) -> Result<GetResponseMessage, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
impl<E: Error> ToResult<GetResponseFile, GetError<E>> for GetResponseFile {
    fn to_result(self) -> Result<GetResponseFile, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
impl<E: Error> ToResult<GetResponseFileComment, GetError<E>> for GetResponseFileComment {
    fn to_result(self) -> Result<GetResponseFileComment, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum GetError<E: Error> {
    /// Value passed for timestamp was invalid.
    BadTimestamp,
    /// File specified by file does not exist.
    FileNotFound,
    /// File comment specified by file_comment does not exist.
    FileCommentNotFound,
    /// Message specified by channel and timestamp does not exist.
    MessageNotFound,
    /// file, file_comment, or combination of channel and timestamp was not specified.
    NoItemSpecified,
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

impl<'a, E: Error> From<&'a str> for GetError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "bad_timestamp" => GetError::BadTimestamp,
            "file_not_found" => GetError::FileNotFound,
            "file_comment_not_found" => GetError::FileCommentNotFound,
            "message_not_found" => GetError::MessageNotFound,
            "no_item_specified" => GetError::NoItemSpecified,
            "not_authed" => GetError::NotAuthed,
            "invalid_auth" => GetError::InvalidAuth,
            "account_inactive" => GetError::AccountInactive,
            "invalid_arg_name" => GetError::InvalidArgName,
            "invalid_array_arg" => GetError::InvalidArrayArg,
            "invalid_charset" => GetError::InvalidCharset,
            "invalid_form_data" => GetError::InvalidFormData,
            "invalid_post_type" => GetError::InvalidPostType,
            "missing_post_type" => GetError::MissingPostType,
            "request_timeout" => GetError::RequestTimeout,
            _ => GetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for GetError<E> {
    fn description(&self) -> &str {
        match self {
            &GetError::BadTimestamp => "bad_timestamp",
            &GetError::FileNotFound => "file_not_found",
            &GetError::FileCommentNotFound => "file_comment_not_found",
            &GetError::MessageNotFound => "message_not_found",
            &GetError::NoItemSpecified => "no_item_specified",
            &GetError::NotAuthed => "not_authed",
            &GetError::InvalidAuth => "invalid_auth",
            &GetError::AccountInactive => "account_inactive",
            &GetError::InvalidArgName => "invalid_arg_name",
            &GetError::InvalidArrayArg => "invalid_array_arg",
            &GetError::InvalidCharset => "invalid_charset",
            &GetError::InvalidFormData => "invalid_form_data",
            &GetError::InvalidPostType => "invalid_post_type",
            &GetError::MissingPostType => "missing_post_type",
            &GetError::RequestTimeout => "request_timeout",
            &GetError::MalformedResponse => "Malformed response data from Slack.",
            &GetError::Unknown(ref s) => s,
            &GetError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &GetError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Lists reactions made by a user.
///
/// Wraps https://api.slack.com/methods/reactions.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
    where R: SlackWebRequestSender
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![Some(("token", request.token)),
                      request.user.map(|user| ("user", user)),
                      request.full.map(|full| ("full", full)),
                      count.as_ref().map(|count| ("count", &count[..])),
                      page.as_ref().map(|page| ("page", &page[..]))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("reactions.list", &params[..])
        .map_err(|err| ListError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(|_| ListError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Authentication token.
    /// Requires scope: reactions:read
    pub token: &'a str,
    /// Show reactions made by this user. Defaults to the authed user.
    pub user: Option<&'a str>,
    /// If true always return the complete reaction list.
    pub full: Option<&'a str>,
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


impl<E: Error> ToResult<ListResponse, ListError<E>> for ListResponse {
    fn to_result(self) -> Result<ListResponse, ListError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum ListError<E: Error> {
    /// Value passed for user was invalid.
    UserNotFound,
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
            "user_not_found" => ListError::UserNotFound,
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
            &ListError::UserNotFound => "user_not_found",
            &ListError::NotAuthed => "not_authed",
            &ListError::InvalidAuth => "invalid_auth",
            &ListError::AccountInactive => "account_inactive",
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

/// Removes a reaction from an item.
///
/// Wraps https://api.slack.com/methods/reactions.remove

pub fn remove<R>(client: &R,
                 request: &RemoveRequest)
                 -> Result<RemoveResponse, RemoveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      Some(("name", request.name)),
                      request.file.map(|file| ("file", file)),
                      request.file_comment.map(|file_comment| ("file_comment", file_comment)),
                      request.channel.map(|channel| ("channel", channel)),
                      request.timestamp.map(|timestamp| ("timestamp", timestamp))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("reactions.remove", &params[..])
        .map_err(|err| RemoveError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|_| RemoveError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct RemoveRequest<'a> {
    /// Authentication token.
    /// Requires scope: reactions:write
    pub token: &'a str,
    /// Reaction (emoji) name.
    pub name: &'a str,
    /// File to remove reaction from.
    pub file: Option<&'a str>,
    /// File comment to remove reaction from.
    pub file_comment: Option<&'a str>,
    /// Channel where the message to remove reaction from was posted.
    pub channel: Option<&'a str>,
    /// Timestamp of the message to remove reaction from.
    pub timestamp: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> ToResult<RemoveResponse, RemoveError<E>> for RemoveResponse {
    fn to_result(self) -> Result<RemoveResponse, RemoveError<E>> {
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
    /// file, file_comment, or combination of channel and timestamp was not specified.
    NoItemSpecified,
    /// Value passed for name was invalid.
    InvalidName,
    /// The specified item does not have the user/reaction combination.
    NoReaction,
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
            "invalid_name" => RemoveError::InvalidName,
            "no_reaction" => RemoveError::NoReaction,
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
            &RemoveError::FileNotFound => "file_not_found",
            &RemoveError::FileCommentNotFound => "file_comment_not_found",
            &RemoveError::MessageNotFound => "message_not_found",
            &RemoveError::NoItemSpecified => "no_item_specified",
            &RemoveError::InvalidName => "invalid_name",
            &RemoveError::NoReaction => "no_reaction",
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
