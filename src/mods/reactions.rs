
#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use crate::requests::SlackWebRequestSender;

/// Adds a reaction to an item.
///
/// Wraps https://api.slack.com/methods/reactions.add

pub async fn add<R>(
    client: &R,
    token: &str,
    request: &AddRequest<'_>,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("name", request.name)),
        request.file.map(|file| ("file", file)),
        request
            .file_comment
            .map(|file_comment| ("file_comment", file_comment)),
        request.channel.map(|channel| ("channel", channel)),
        request.timestamp.map(|timestamp| ("timestamp", timestamp)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("reactions.add");
    client
        .send(&url, &params[..])
        .await
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result).map_err(AddError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct AddRequest<'a> {
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

impl<E: Error> Into<Result<AddResponse, AddError<E>>> for AddResponse {
    fn into(self) -> Result<AddResponse, AddError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(serde_json::error::Error),
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
            "team_added_to_org" => AddError::TeamAddedToOrg,
            "request_timeout" => AddError::RequestTimeout,
            _ => AddError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for AddError<E> {
    fn description(&self) -> &str {
        match *self {
                        AddError::BadTimestamp => "bad_timestamp: Value passed for timestamp was invalid.",
AddError::FileNotFound => "file_not_found: File specified by file does not exist.",
AddError::FileCommentNotFound => "file_comment_not_found: File comment specified by file_comment does not exist.",
AddError::MessageNotFound => "message_not_found: Message specified by channel and timestamp does not exist.",
AddError::NoItemSpecified => "no_item_specified: file, file_comment, or combination of channel and timestamp was not specified.",
AddError::InvalidName => "invalid_name: Value passed for name was invalid.",
AddError::AlreadyReacted => "already_reacted: The specified item already has the user/reaction combination.",
AddError::TooManyEmoji => "too_many_emoji: The limit for distinct reactions (i.e emoji) on the item has been reached.",
AddError::TooManyReactions => "too_many_reactions: The limit for reactions a person may add to the item has been reached.",
AddError::NotAuthed => "not_authed: No authentication token provided.",
AddError::InvalidAuth => "invalid_auth: Invalid authentication token.",
AddError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
AddError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
AddError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
AddError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
AddError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
AddError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
AddError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
AddError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
AddError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        AddError::MalformedResponse(ref e) => e.description(),
                        AddError::Unknown(ref s) => s,
                        AddError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            AddError::MalformedResponse(ref e) => Some(e),
            AddError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets reactions for an item.
///
/// Wraps https://api.slack.com/methods/reactions.get

pub async fn get<R>(
    client: &R,
    token: &str,
    request: &GetRequest<'_>,
) -> Result<GetResponse, GetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request.file.map(|file| ("file", file)),
        request
            .file_comment
            .map(|file_comment| ("file_comment", file_comment)),
        request.channel.map(|channel| ("channel", channel)),
        request.timestamp.map(|timestamp| ("timestamp", timestamp)),
        request
            .full
            .map(|full| ("full", if full { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("reactions.get");
    client
        .send(&url, &params[..])
        .await
        .map_err(GetError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result).map_err(GetError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct GetRequest<'a> {
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

impl<'de> ::serde::Deserialize<'de> for GetResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        use ::serde::de::Error as SerdeError;

        const VARIANTS: &'static [&'static str] = &["message", "file", "file_comment"];

        let value = ::serde_json::Value::deserialize(deserializer)?;
        if let Some(ty_val) = value.get("type") {
            if let Some(ty) = ty_val.as_str() {
                match ty {
                    "message" => ::serde_json::from_value::<GetResponseMessage>(value.clone())
                        .map(GetResponse::Message)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "file" => ::serde_json::from_value::<GetResponseFile>(value.clone())
                        .map(GetResponse::File)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "file_comment" => {
                        ::serde_json::from_value::<GetResponseFileComment>(value.clone())
                            .map(GetResponse::FileComment)
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
            Err(D::Error::missing_field("type"))
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponseFile {
    error: Option<String>,
    pub file: crate::File,
    #[serde(default)]
    ok: bool,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponseFileComment {
    pub comment: crate::FileComment,
    error: Option<String>,
    pub file: crate::File,
    #[serde(default)]
    ok: bool,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponseMessage {
    pub channel: String,
    error: Option<String>,
    pub message: crate::Message,
    #[serde(default)]
    ok: bool,
    #[serde(rename = "type")]
    pub ty: String,
}

impl<E: Error> Into<Result<GetResponse, GetError<E>>> for GetResponse {
    fn into(self) -> Result<GetResponse, GetError<E>> {
        match self {
            GetResponse::Message(inner) => {
                let x: Result<GetResponseMessage, GetError<E>> = inner.into();
                x.map(GetResponse::Message)
            }
            GetResponse::File(inner) => {
                let x: Result<GetResponseFile, GetError<E>> = inner.into();
                x.map(GetResponse::File)
            }
            GetResponse::FileComment(inner) => {
                let x: Result<GetResponseFileComment, GetError<E>> = inner.into();
                x.map(GetResponse::FileComment)
            }
        }
    }
}

impl<E: Error> Into<Result<GetResponseMessage, GetError<E>>> for GetResponseMessage {
    fn into(self) -> Result<GetResponseMessage, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
impl<E: Error> Into<Result<GetResponseFile, GetError<E>>> for GetResponseFile {
    fn into(self) -> Result<GetResponseFile, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
impl<E: Error> Into<Result<GetResponseFileComment, GetError<E>>> for GetResponseFileComment {
    fn into(self) -> Result<GetResponseFileComment, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(serde_json::error::Error),
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
            "team_added_to_org" => GetError::TeamAddedToOrg,
            "request_timeout" => GetError::RequestTimeout,
            _ => GetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for GetError<E> {
    fn description(&self) -> &str {
        match *self {
                        GetError::BadTimestamp => "bad_timestamp: Value passed for timestamp was invalid.",
GetError::FileNotFound => "file_not_found: File specified by file does not exist.",
GetError::FileCommentNotFound => "file_comment_not_found: File comment specified by file_comment does not exist.",
GetError::MessageNotFound => "message_not_found: Message specified by channel and timestamp does not exist.",
GetError::NoItemSpecified => "no_item_specified: file, file_comment, or combination of channel and timestamp was not specified.",
GetError::NotAuthed => "not_authed: No authentication token provided.",
GetError::InvalidAuth => "invalid_auth: Invalid authentication token.",
GetError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
GetError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
GetError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
GetError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
GetError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
GetError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
GetError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
GetError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
GetError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        GetError::MalformedResponse(ref e) => e.description(),
                        GetError::Unknown(ref s) => s,
                        GetError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            GetError::MalformedResponse(ref e) => Some(e),
            GetError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Lists reactions made by a user.
///
/// Wraps https://api.slack.com/methods/reactions.list

pub async fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        request.user.map(|user| ("user", user)),
        request
            .full
            .map(|full| ("full", if full { "1" } else { "0" })),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("reactions.list");
    client
        .send(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(ListError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Show reactions made by this user. Defaults to the authed user.
    pub user: Option<&'a str>,
    /// If true always return the complete reaction list.
    pub full: Option<bool>,
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
    pub paging: Option<crate::Paging>,
}

#[derive(Clone, Debug)]
pub enum ListResponseItem {
    Message(ListResponseItemMessage),
    File(ListResponseItemFile),
    FileComment(ListResponseItemFileComment),
}

impl<'de> ::serde::Deserialize<'de> for ListResponseItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        use ::serde::de::Error as SerdeError;

        const VARIANTS: &'static [&'static str] = &["message", "file", "file_comment"];

        let value = ::serde_json::Value::deserialize(deserializer)?;
        if let Some(ty_val) = value.get("type") {
            if let Some(ty) = ty_val.as_str() {
                match ty {
                    "message" => ::serde_json::from_value::<ListResponseItemMessage>(value.clone())
                        .map(ListResponseItem::Message)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "file" => ::serde_json::from_value::<ListResponseItemFile>(value.clone())
                        .map(ListResponseItem::File)
                        .map_err(|e| D::Error::custom(&format!("{}", e))),
                    "file_comment" => {
                        ::serde_json::from_value::<ListResponseItemFileComment>(value.clone())
                            .map(ListResponseItem::FileComment)
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
            Err(D::Error::missing_field("type"))
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemFile {
    pub file: crate::File,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemFileComment {
    pub comment: crate::FileComment,
    pub file: crate::File,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseItemMessage {
    pub channel: String,
    pub message: crate::Message,
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
#[derive(Debug)]
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(serde_json::error::Error),
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
            "team_added_to_org" => ListError::TeamAddedToOrg,
            "request_timeout" => ListError::RequestTimeout,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for ListError<E> {
    fn description(&self) -> &str {
        match *self {
                        ListError::UserNotFound => "user_not_found: Value passed for user was invalid.",
ListError::NotAuthed => "not_authed: No authentication token provided.",
ListError::InvalidAuth => "invalid_auth: Invalid authentication token.",
ListError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
ListError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
ListError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
ListError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
ListError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
ListError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
ListError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
ListError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
ListError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        ListError::MalformedResponse(ref e) => e.description(),
                        ListError::Unknown(ref s) => s,
                        ListError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ListError::MalformedResponse(ref e) => Some(e),
            ListError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Removes a reaction from an item.
///
/// Wraps https://api.slack.com/methods/reactions.remove

pub async fn remove<R>(
    client: &R,
    token: &str,
    request: &RemoveRequest<'_>,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("name", request.name)),
        request.file.map(|file| ("file", file)),
        request
            .file_comment
            .map(|file_comment| ("file_comment", file_comment)),
        request.channel.map(|channel| ("channel", channel)),
        request.timestamp.map(|timestamp| ("timestamp", timestamp)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("reactions.remove");
    client
        .send(&url, &params[..])
        .await
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result).map_err(RemoveError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct RemoveRequest<'a> {
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

impl<E: Error> Into<Result<RemoveResponse, RemoveError<E>>> for RemoveResponse {
    fn into(self) -> Result<RemoveResponse, RemoveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(serde_json::error::Error),
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
            "team_added_to_org" => RemoveError::TeamAddedToOrg,
            "request_timeout" => RemoveError::RequestTimeout,
            _ => RemoveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RemoveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for RemoveError<E> {
    fn description(&self) -> &str {
        match *self {
                        RemoveError::BadTimestamp => "bad_timestamp: Value passed for timestamp was invalid.",
RemoveError::FileNotFound => "file_not_found: File specified by file does not exist.",
RemoveError::FileCommentNotFound => "file_comment_not_found: File comment specified by file_comment does not exist.",
RemoveError::MessageNotFound => "message_not_found: Message specified by channel and timestamp does not exist.",
RemoveError::NoItemSpecified => "no_item_specified: file, file_comment, or combination of channel and timestamp was not specified.",
RemoveError::InvalidName => "invalid_name: Value passed for name was invalid.",
RemoveError::NoReaction => "no_reaction: The specified item does not have the user/reaction combination.",
RemoveError::NotAuthed => "not_authed: No authentication token provided.",
RemoveError::InvalidAuth => "invalid_auth: Invalid authentication token.",
RemoveError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
RemoveError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
RemoveError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
RemoveError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
RemoveError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
RemoveError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
RemoveError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
RemoveError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
RemoveError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        RemoveError::MalformedResponse(ref e) => e.description(),
                        RemoveError::Unknown(ref s) => s,
                        RemoveError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            RemoveError::MalformedResponse(ref e) => Some(e),
            RemoveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
