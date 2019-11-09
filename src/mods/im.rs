//! Get info on your direct messages.

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Close a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.close

pub fn close<R>(
    client: &R,
    token: &str,
    request: &CloseRequest,
) -> Result<CloseResponse, CloseError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("im.close");
    client
        .send(&url, &params[..])
        .map_err(CloseError::Client)
        .and_then(|result| {
            serde_json::from_str::<CloseResponse>(&result).map_err(CloseError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct CloseRequest<'a> {
    /// Direct message channel to close.
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CloseResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<CloseResponse, CloseError<E>>> for CloseResponse {
    fn into(self) -> Result<CloseResponse, CloseError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum CloseError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Calling user does not own this DM channel.
    UserDoesNotOwnChannel,
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

impl<'a, E: Error> From<&'a str> for CloseError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => CloseError::ChannelNotFound,
            "user_does_not_own_channel" => CloseError::UserDoesNotOwnChannel,
            "not_authed" => CloseError::NotAuthed,
            "invalid_auth" => CloseError::InvalidAuth,
            "account_inactive" => CloseError::AccountInactive,
            "invalid_arg_name" => CloseError::InvalidArgName,
            "invalid_array_arg" => CloseError::InvalidArrayArg,
            "invalid_charset" => CloseError::InvalidCharset,
            "invalid_form_data" => CloseError::InvalidFormData,
            "invalid_post_type" => CloseError::InvalidPostType,
            "missing_post_type" => CloseError::MissingPostType,
            "team_added_to_org" => CloseError::TeamAddedToOrg,
            "request_timeout" => CloseError::RequestTimeout,
            _ => CloseError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CloseError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for CloseError<E> {
    fn description(&self) -> &str {
        match *self {
                        CloseError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
CloseError::UserDoesNotOwnChannel => "user_does_not_own_channel: Calling user does not own this DM channel.",
CloseError::NotAuthed => "not_authed: No authentication token provided.",
CloseError::InvalidAuth => "invalid_auth: Invalid authentication token.",
CloseError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
CloseError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
CloseError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
CloseError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
CloseError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
CloseError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
CloseError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
CloseError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
CloseError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        CloseError::MalformedResponse(ref e) => e.description(),
                        CloseError::Unknown(ref s) => s,
                        CloseError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CloseError::MalformedResponse(ref e) => Some(e),
            CloseError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Fetches history of messages and events from direct message channel.
///
/// Wraps https://api.slack.com/methods/im.history

pub fn history<R>(
    client: &R,
    token: &str,
    request: &HistoryRequest,
) -> Result<HistoryResponse, HistoryError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        request.latest.map(|latest| ("latest", latest)),
        request.oldest.map(|oldest| ("oldest", oldest)),
        request
            .inclusive
            .map(|inclusive| ("inclusive", if inclusive { "1" } else { "0" })),
        count.as_ref().map(|count| ("count", &count[..])),
        request
            .unreads
            .map(|unreads| ("unreads", if unreads { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("im.history");
    client
        .send(&url, &params[..])
        .map_err(HistoryError::Client)
        .and_then(|result| {
            serde_json::from_str::<HistoryResponse>(&result)
                .map_err(HistoryError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct HistoryRequest<'a> {
    /// Direct message channel to fetch history for.
    pub channel: &'a str,
    /// End of time range of messages to include in results.
    pub latest: Option<&'a str>,
    /// Start of time range of messages to include in results.
    pub oldest: Option<&'a str>,
    /// Include messages with latest or oldest timestamp in results.
    pub inclusive: Option<bool>,
    /// Number of messages to return, between 1 and 1000.
    pub count: Option<u32>,
    /// Include unread_count_display in the output?
    pub unreads: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryResponse {
    error: Option<String>,
    pub has_more: Option<bool>,
    pub latest: Option<String>,
    pub messages: Option<Vec<::Message>>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<HistoryResponse, HistoryError<E>>> for HistoryResponse {
    fn into(self) -> Result<HistoryResponse, HistoryError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum HistoryError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for latest was invalid
    InvalidTsLatest,
    /// Value passed for oldest was invalid
    InvalidTsOldest,
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

impl<'a, E: Error> From<&'a str> for HistoryError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => HistoryError::ChannelNotFound,
            "invalid_ts_latest" => HistoryError::InvalidTsLatest,
            "invalid_ts_oldest" => HistoryError::InvalidTsOldest,
            "not_authed" => HistoryError::NotAuthed,
            "invalid_auth" => HistoryError::InvalidAuth,
            "account_inactive" => HistoryError::AccountInactive,
            "invalid_arg_name" => HistoryError::InvalidArgName,
            "invalid_array_arg" => HistoryError::InvalidArrayArg,
            "invalid_charset" => HistoryError::InvalidCharset,
            "invalid_form_data" => HistoryError::InvalidFormData,
            "invalid_post_type" => HistoryError::InvalidPostType,
            "missing_post_type" => HistoryError::MissingPostType,
            "team_added_to_org" => HistoryError::TeamAddedToOrg,
            "request_timeout" => HistoryError::RequestTimeout,
            _ => HistoryError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for HistoryError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for HistoryError<E> {
    fn description(&self) -> &str {
        match *self {
                        HistoryError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
HistoryError::InvalidTsLatest => "invalid_ts_latest: Value passed for latest was invalid",
HistoryError::InvalidTsOldest => "invalid_ts_oldest: Value passed for oldest was invalid",
HistoryError::NotAuthed => "not_authed: No authentication token provided.",
HistoryError::InvalidAuth => "invalid_auth: Invalid authentication token.",
HistoryError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
HistoryError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
HistoryError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
HistoryError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
HistoryError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
HistoryError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
HistoryError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
HistoryError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
HistoryError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        HistoryError::MalformedResponse(ref e) => e.description(),
                        HistoryError::Unknown(ref s) => s,
                        HistoryError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            HistoryError::MalformedResponse(ref e) => Some(e),
            HistoryError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Lists direct message channels for the calling user.
///
/// Wraps https://api.slack.com/methods/im.list

pub fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let limit = request.limit.map(|limit| limit.to_string());
    let params = vec![
        Some(("token", token)),
        request.cursor.map(|cursor| ("cursor", cursor)),
        limit.as_ref().map(|limit| ("limit", &limit[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("im.list");
    client
        .send(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(ListError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See pagination for more detail.
    pub cursor: Option<&'a str>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    error: Option<String>,
    pub ims: Option<Vec<::Im>>,
    #[serde(default)]
    ok: bool,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for ListError<E> {
    fn description(&self) -> &str {
        match *self {
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

    fn cause(&self) -> Option<&Error> {
        match *self {
            ListError::MalformedResponse(ref e) => Some(e),
            ListError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Sets the read cursor in a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.mark

pub fn mark<R>(
    client: &R,
    token: &str,
    request: &MarkRequest,
) -> Result<MarkResponse, MarkError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("ts", request.ts)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("im.mark");
    client
        .send(&url, &params[..])
        .map_err(MarkError::Client)
        .and_then(|result| {
            serde_json::from_str::<MarkResponse>(&result).map_err(MarkError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct MarkRequest<'a> {
    /// Direct message channel to set reading cursor in.
    pub channel: &'a str,
    /// Timestamp of the most recently seen message.
    pub ts: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MarkResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<MarkResponse, MarkError<E>>> for MarkResponse {
    fn into(self) -> Result<MarkResponse, MarkError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum MarkError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for timestamp was invalid.
    InvalidTimestamp,
    /// Caller is not a member of the channel.
    NotInChannel,
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

impl<'a, E: Error> From<&'a str> for MarkError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => MarkError::ChannelNotFound,
            "invalid_timestamp" => MarkError::InvalidTimestamp,
            "not_in_channel" => MarkError::NotInChannel,
            "not_authed" => MarkError::NotAuthed,
            "invalid_auth" => MarkError::InvalidAuth,
            "account_inactive" => MarkError::AccountInactive,
            "invalid_arg_name" => MarkError::InvalidArgName,
            "invalid_array_arg" => MarkError::InvalidArrayArg,
            "invalid_charset" => MarkError::InvalidCharset,
            "invalid_form_data" => MarkError::InvalidFormData,
            "invalid_post_type" => MarkError::InvalidPostType,
            "missing_post_type" => MarkError::MissingPostType,
            "team_added_to_org" => MarkError::TeamAddedToOrg,
            "request_timeout" => MarkError::RequestTimeout,
            _ => MarkError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MarkError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for MarkError<E> {
    fn description(&self) -> &str {
        match *self {
                        MarkError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
MarkError::InvalidTimestamp => "invalid_timestamp: Value passed for timestamp was invalid.",
MarkError::NotInChannel => "not_in_channel: Caller is not a member of the channel.",
MarkError::NotAuthed => "not_authed: No authentication token provided.",
MarkError::InvalidAuth => "invalid_auth: Invalid authentication token.",
MarkError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
MarkError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
MarkError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
MarkError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
MarkError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
MarkError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
MarkError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
MarkError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
MarkError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        MarkError::MalformedResponse(ref e) => e.description(),
                        MarkError::Unknown(ref s) => s,
                        MarkError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            MarkError::MalformedResponse(ref e) => Some(e),
            MarkError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Opens a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.open

pub fn open<R>(
    client: &R,
    token: &str,
    request: &OpenRequest,
) -> Result<OpenResponse, OpenError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("user", request.user)),
        request
            .return_im
            .map(|return_im| ("return_im", if return_im { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("im.open");
    client
        .send(&url, &params[..])
        .map_err(OpenError::Client)
        .and_then(|result| {
            serde_json::from_str::<OpenResponse>(&result).map_err(OpenError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct OpenRequest<'a> {
    /// User to open a direct message channel with.
    pub user: &'a str,
    /// Boolean, indicates you want the full IM channel definition in the response.
    pub return_im: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenResponse {
    pub channel: Option<::Im>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<OpenResponse, OpenError<E>>> for OpenResponse {
    fn into(self) -> Result<OpenResponse, OpenError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum OpenError<E: Error> {
    /// Value passed for user was invalid.
    UserNotFound,
    /// The calling user is restricted from seeing the requested user.
    UserNotVisible,
    /// The user has been disabled.
    UserDisabled,
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

impl<'a, E: Error> From<&'a str> for OpenError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "user_not_found" => OpenError::UserNotFound,
            "user_not_visible" => OpenError::UserNotVisible,
            "user_disabled" => OpenError::UserDisabled,
            "not_authed" => OpenError::NotAuthed,
            "invalid_auth" => OpenError::InvalidAuth,
            "account_inactive" => OpenError::AccountInactive,
            "invalid_arg_name" => OpenError::InvalidArgName,
            "invalid_array_arg" => OpenError::InvalidArrayArg,
            "invalid_charset" => OpenError::InvalidCharset,
            "invalid_form_data" => OpenError::InvalidFormData,
            "invalid_post_type" => OpenError::InvalidPostType,
            "missing_post_type" => OpenError::MissingPostType,
            "team_added_to_org" => OpenError::TeamAddedToOrg,
            "request_timeout" => OpenError::RequestTimeout,
            _ => OpenError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for OpenError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for OpenError<E> {
    fn description(&self) -> &str {
        match *self {
                        OpenError::UserNotFound => "user_not_found: Value passed for user was invalid.",
OpenError::UserNotVisible => "user_not_visible: The calling user is restricted from seeing the requested user.",
OpenError::UserDisabled => "user_disabled: The user has been disabled.",
OpenError::NotAuthed => "not_authed: No authentication token provided.",
OpenError::InvalidAuth => "invalid_auth: Invalid authentication token.",
OpenError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
OpenError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
OpenError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
OpenError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
OpenError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
OpenError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
OpenError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
OpenError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
OpenError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        OpenError::MalformedResponse(ref e) => e.description(),
                        OpenError::Unknown(ref s) => s,
                        OpenError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            OpenError::MalformedResponse(ref e) => Some(e),
            OpenError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Retrieve a thread of messages posted to a direct message conversation
///
/// Wraps https://api.slack.com/methods/im.replies

pub fn replies<R>(
    client: &R,
    token: &str,
    request: &RepliesRequest,
) -> Result<RepliesResponse, RepliesError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("thread_ts", request.thread_ts)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("im.replies");
    client
        .send(&url, &params[..])
        .map_err(RepliesError::Client)
        .and_then(|result| {
            serde_json::from_str::<RepliesResponse>(&result)
                .map_err(RepliesError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct RepliesRequest<'a> {
    /// Direct message channel to fetch thread from
    pub channel: &'a str,
    /// Unique identifier of a thread's parent message
    pub thread_ts: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepliesResponse {
    error: Option<String>,
    pub messages: Option<Vec<::Message>>,
    #[serde(default)]
    ok: bool,
    pub thread_info: Option<::ThreadInfo>,
}

impl<E: Error> Into<Result<RepliesResponse, RepliesError<E>>> for RepliesResponse {
    fn into(self) -> Result<RepliesResponse, RepliesError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum RepliesError<E: Error> {
    /// Value for channel was missing or invalid.
    ChannelNotFound,
    /// Value for thread_ts was missing or invalid.
    ThreadNotFound,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RepliesError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => RepliesError::ChannelNotFound,
            "thread_not_found" => RepliesError::ThreadNotFound,
            "not_authed" => RepliesError::NotAuthed,
            "invalid_auth" => RepliesError::InvalidAuth,
            "account_inactive" => RepliesError::AccountInactive,
            "user_is_bot" => RepliesError::UserIsBot,
            "invalid_arg_name" => RepliesError::InvalidArgName,
            "invalid_array_arg" => RepliesError::InvalidArrayArg,
            "invalid_charset" => RepliesError::InvalidCharset,
            "invalid_form_data" => RepliesError::InvalidFormData,
            "invalid_post_type" => RepliesError::InvalidPostType,
            "missing_post_type" => RepliesError::MissingPostType,
            "team_added_to_org" => RepliesError::TeamAddedToOrg,
            "request_timeout" => RepliesError::RequestTimeout,
            _ => RepliesError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RepliesError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for RepliesError<E> {
    fn description(&self) -> &str {
        match *self {
                        RepliesError::ChannelNotFound => "channel_not_found: Value for channel was missing or invalid.",
RepliesError::ThreadNotFound => "thread_not_found: Value for thread_ts was missing or invalid.",
RepliesError::NotAuthed => "not_authed: No authentication token provided.",
RepliesError::InvalidAuth => "invalid_auth: Invalid authentication token.",
RepliesError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
RepliesError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
RepliesError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
RepliesError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
RepliesError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
RepliesError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
RepliesError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
RepliesError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
RepliesError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
RepliesError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        RepliesError::MalformedResponse(ref e) => e.description(),
                        RepliesError::Unknown(ref s) => s,
                        RepliesError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RepliesError::MalformedResponse(ref e) => Some(e),
            RepliesError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
