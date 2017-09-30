//! Search your team's files and messages.


#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Searches for messages and files matching a query.
///
/// Wraps https://api.slack.com/methods/search.all

pub fn all<R>(
    client: &R,
    token: &str,
    request: &AllRequest,
) -> Result<AllResponse, AllError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        Some(("query", request.query)),
        request.sort.map(|sort| ("sort", sort)),
        request.sort_dir.map(|sort_dir| ("sort_dir", sort_dir)),
        request.highlight.map(|highlight| {
            ("highlight", if highlight { "1" } else { "0" })
        }),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("search.all");
    client
        .send(&url, &params[..])
        .map_err(AllError::Client)
        .and_then(|result| {
            serde_json::from_str::<AllResponse>(&result).map_err(AllError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct AllRequest<'a> {
    /// Search query. May contains booleans, etc.
    pub query: &'a str,
    /// Return matches sorted by either score or timestamp.
    pub sort: Option<&'a str>,
    /// Change sort direction to ascending (asc) or descending (desc).
    pub sort_dir: Option<&'a str>,
    /// Pass a value of true to enable query highlight markers (see below).
    pub highlight: Option<bool>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AllResponse {
    error: Option<String>,
    pub files: Option<AllResponseFiles>,
    pub messages: Option<AllResponseMessages>,
    #[serde(default)]
    ok: bool,
    pub query: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AllResponseFiles {
    pub matches: Vec<::File>,
    pub paging: ::Paging,
}


#[derive(Clone, Debug, Deserialize)]
pub struct AllResponseMessages {
    pub matches: Vec<::Message>,
    pub paging: ::Paging,
}


impl<E: Error> Into<Result<AllResponse, AllError<E>>> for AllResponse {
    fn into(self) -> Result<AllResponse, AllError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum AllError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for AllError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => AllError::NotAuthed,
            "invalid_auth" => AllError::InvalidAuth,
            "account_inactive" => AllError::AccountInactive,
            "user_is_bot" => AllError::UserIsBot,
            "invalid_arg_name" => AllError::InvalidArgName,
            "invalid_array_arg" => AllError::InvalidArrayArg,
            "invalid_charset" => AllError::InvalidCharset,
            "invalid_form_data" => AllError::InvalidFormData,
            "invalid_post_type" => AllError::InvalidPostType,
            "missing_post_type" => AllError::MissingPostType,
            "team_added_to_org" => AllError::TeamAddedToOrg,
            "request_timeout" => AllError::RequestTimeout,
            _ => AllError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AllError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for AllError<E> {
    fn description(&self) -> &str {
        match *self {
            AllError::NotAuthed => "not_authed: No authentication token provided.",
            AllError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            AllError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            AllError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            AllError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            AllError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            AllError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            AllError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            AllError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            AllError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            AllError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            AllError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            AllError::MalformedResponse(ref e) => e.description(),
            AllError::Unknown(ref s) => s,
            AllError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AllError::MalformedResponse(ref e) => Some(e),
            AllError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Searches for files matching a query.
///
/// Wraps https://api.slack.com/methods/search.files

pub fn files<R>(
    client: &R,
    token: &str,
    request: &FilesRequest,
) -> Result<FilesResponse, FilesError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        Some(("query", request.query)),
        request.sort.map(|sort| ("sort", sort)),
        request.sort_dir.map(|sort_dir| ("sort_dir", sort_dir)),
        request.highlight.map(|highlight| {
            ("highlight", if highlight { "1" } else { "0" })
        }),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("search.files");
    client
        .send(&url, &params[..])
        .map_err(FilesError::Client)
        .and_then(|result| {
            serde_json::from_str::<FilesResponse>(&result).map_err(FilesError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct FilesRequest<'a> {
    /// Search query. May contain booleans, etc.
    pub query: &'a str,
    /// Return matches sorted by either score or timestamp.
    pub sort: Option<&'a str>,
    /// Change sort direction to ascending (asc) or descending (desc).
    pub sort_dir: Option<&'a str>,
    /// Pass a value of true to enable query highlight markers (see below).
    pub highlight: Option<bool>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FilesResponse {
    error: Option<String>,
    pub files: Option<FilesResponseFiles>,
    #[serde(default)]
    ok: bool,
    pub query: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FilesResponseFiles {
    pub matches: Option<Vec<::File>>,
    pub paging: Option<::Paging>,
    pub total: Option<i32>,
}


impl<E: Error> Into<Result<FilesResponse, FilesError<E>>> for FilesResponse {
    fn into(self) -> Result<FilesResponse, FilesError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum FilesError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for FilesError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => FilesError::NotAuthed,
            "invalid_auth" => FilesError::InvalidAuth,
            "account_inactive" => FilesError::AccountInactive,
            "user_is_bot" => FilesError::UserIsBot,
            "invalid_arg_name" => FilesError::InvalidArgName,
            "invalid_array_arg" => FilesError::InvalidArrayArg,
            "invalid_charset" => FilesError::InvalidCharset,
            "invalid_form_data" => FilesError::InvalidFormData,
            "invalid_post_type" => FilesError::InvalidPostType,
            "missing_post_type" => FilesError::MissingPostType,
            "team_added_to_org" => FilesError::TeamAddedToOrg,
            "request_timeout" => FilesError::RequestTimeout,
            _ => FilesError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for FilesError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for FilesError<E> {
    fn description(&self) -> &str {
        match *self {
            FilesError::NotAuthed => "not_authed: No authentication token provided.",
            FilesError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            FilesError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            FilesError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            FilesError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            FilesError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            FilesError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            FilesError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            FilesError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            FilesError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            FilesError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            FilesError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            FilesError::MalformedResponse(ref e) => e.description(),
            FilesError::Unknown(ref s) => s,
            FilesError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            FilesError::MalformedResponse(ref e) => Some(e),
            FilesError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Searches for messages matching a query.
///
/// Wraps https://api.slack.com/methods/search.messages

pub fn messages<R>(
    client: &R,
    token: &str,
    request: &MessagesRequest,
) -> Result<MessagesResponse, MessagesError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        Some(("query", request.query)),
        request.sort.map(|sort| ("sort", sort)),
        request.sort_dir.map(|sort_dir| ("sort_dir", sort_dir)),
        request.highlight.map(|highlight| {
            ("highlight", if highlight { "1" } else { "0" })
        }),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("search.messages");
    client
        .send(&url, &params[..])
        .map_err(MessagesError::Client)
        .and_then(|result| {
            serde_json::from_str::<MessagesResponse>(&result).map_err(
                MessagesError::MalformedResponse,
            )
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct MessagesRequest<'a> {
    /// Search query. May contains booleans, etc.
    pub query: &'a str,
    /// Return matches sorted by either score or timestamp.
    pub sort: Option<&'a str>,
    /// Change sort direction to ascending (asc) or descending (desc).
    pub sort_dir: Option<&'a str>,
    /// Pass a value of true to enable query highlight markers (see below).
    pub highlight: Option<bool>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessagesResponse {
    error: Option<String>,
    pub messages: Option<MessagesResponseMessages>,
    #[serde(default)]
    ok: bool,
    pub query: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessagesResponseMessages {
    pub matches: Option<Vec<::Message>>,
    pub paging: Option<::Paging>,
    pub total: Option<i32>,
}


impl<E: Error> Into<Result<MessagesResponse, MessagesError<E>>> for MessagesResponse {
    fn into(self) -> Result<MessagesResponse, MessagesError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum MessagesError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for MessagesError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => MessagesError::NotAuthed,
            "invalid_auth" => MessagesError::InvalidAuth,
            "account_inactive" => MessagesError::AccountInactive,
            "user_is_bot" => MessagesError::UserIsBot,
            "invalid_arg_name" => MessagesError::InvalidArgName,
            "invalid_array_arg" => MessagesError::InvalidArrayArg,
            "invalid_charset" => MessagesError::InvalidCharset,
            "invalid_form_data" => MessagesError::InvalidFormData,
            "invalid_post_type" => MessagesError::InvalidPostType,
            "missing_post_type" => MessagesError::MissingPostType,
            "team_added_to_org" => MessagesError::TeamAddedToOrg,
            "request_timeout" => MessagesError::RequestTimeout,
            _ => MessagesError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MessagesError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for MessagesError<E> {
    fn description(&self) -> &str {
        match *self {
            MessagesError::NotAuthed => "not_authed: No authentication token provided.",
            MessagesError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            MessagesError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            MessagesError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            MessagesError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            MessagesError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            MessagesError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            MessagesError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            MessagesError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            MessagesError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            MessagesError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            MessagesError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            MessagesError::MalformedResponse(ref e) => e.description(),
            MessagesError::Unknown(ref s) => s,
            MessagesError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            MessagesError::MalformedResponse(ref e) => Some(e),
            MessagesError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
