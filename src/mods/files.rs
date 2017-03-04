
#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Deletes a file.
///
/// Wraps https://api.slack.com/methods/files.delete

pub fn delete<R>(client: &R, request: &DeleteRequest) -> Result<DeleteResponse, DeleteError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("file", request.file))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("files.delete", &params[..])
        .map_err(|err| DeleteError::Client(err))
        .and_then(|result| serde_json::from_str::<DeleteResponse>(&result).map_err(|_| DeleteError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest<'a> {
    /// Authentication token.
    /// Requires scope: files:write:user
    pub token: &'a str,
    /// ID of file to delete.
    pub file: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
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
    /// The file does not exist, or is not visible to the calling user.
    FileNotFound,
    /// The file has already been deleted.
    FileDeleted,
    /// Authenticated user does not have permission to delete this file.
    CantDeleteFile,
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
            "file_not_found" => DeleteError::FileNotFound,
            "file_deleted" => DeleteError::FileDeleted,
            "cant_delete_file" => DeleteError::CantDeleteFile,
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
            &DeleteError::FileNotFound => "file_not_found",
            &DeleteError::FileDeleted => "file_deleted",
            &DeleteError::CantDeleteFile => "cant_delete_file",
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

/// Gets information about a team file.
///
/// Wraps https://api.slack.com/methods/files.info

pub fn info<R>(client: &R, request: &InfoRequest) -> Result<InfoResponse, InfoError<R::Error>>
    where R: SlackWebRequestSender
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![Some(("token", request.token)),
                      Some(("file", request.file)),
                      count.as_ref().map(|count| ("count", &count[..])),
                      page.as_ref().map(|page| ("page", &page[..]))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("files.info", &params[..])
        .map_err(|err| InfoError::Client(err))
        .and_then(|result| serde_json::from_str::<InfoResponse>(&result).map_err(|_| InfoError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Authentication token.
    /// Requires scope: files:read
    pub token: &'a str,
    /// Specify a file by providing its ID.
    pub file: &'a str,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub comments: Option<Vec<::FileComment>>,
    error: Option<String>,
    pub file: Option<::File>,
    #[serde(default)]
    ok: bool,
    pub paging: Option<::Paging>,
}


impl<E: Error> Into<Result<InfoResponse, InfoError<E>>> for InfoResponse {
    fn into(self) -> Result<InfoResponse, InfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum InfoError<E: Error> {
    /// Value passed for file was invalid
    FileNotFound,
    /// The requested file has been deleted
    FileDeleted,
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

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "file_not_found" => InfoError::FileNotFound,
            "file_deleted" => InfoError::FileDeleted,
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_post_type" => InfoError::InvalidPostType,
            "missing_post_type" => InfoError::MissingPostType,
            "request_timeout" => InfoError::RequestTimeout,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for InfoError<E> {
    fn description(&self) -> &str {
        match self {
            &InfoError::FileNotFound => "file_not_found",
            &InfoError::FileDeleted => "file_deleted",
            &InfoError::NotAuthed => "not_authed",
            &InfoError::InvalidAuth => "invalid_auth",
            &InfoError::AccountInactive => "account_inactive",
            &InfoError::InvalidArgName => "invalid_arg_name",
            &InfoError::InvalidArrayArg => "invalid_array_arg",
            &InfoError::InvalidCharset => "invalid_charset",
            &InfoError::InvalidFormData => "invalid_form_data",
            &InfoError::InvalidPostType => "invalid_post_type",
            &InfoError::MissingPostType => "missing_post_type",
            &InfoError::RequestTimeout => "request_timeout",
            &InfoError::MalformedResponse => "Malformed response data from Slack.",
            &InfoError::Unknown(ref s) => s,
            &InfoError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &InfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Lists & filters team files.
///
/// Wraps https://api.slack.com/methods/files.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
    where R: SlackWebRequestSender
{
    let ts_from = request.ts_from.map(|ts_from| ts_from.to_string());
    let ts_to = request.ts_to.map(|ts_to| ts_to.to_string());
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![Some(("token", request.token)),
                      request.user.map(|user| ("user", user)),
                      request.channel.map(|channel| ("channel", channel)),
                      ts_from.as_ref().map(|ts_from| ("ts_from", &ts_from[..])),
                      ts_to.as_ref().map(|ts_to| ("ts_to", &ts_to[..])),
                      request.types.map(|types| ("types", types)),
                      count.as_ref().map(|count| ("count", &count[..])),
                      page.as_ref().map(|page| ("page", &page[..]))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("files.list", &params[..])
        .map_err(|err| ListError::Client(err))
        .and_then(|result| serde_json::from_str::<ListResponse>(&result).map_err(|_| ListError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Authentication token.
    /// Requires scope: files:read
    pub token: &'a str,
    /// Filter files created by a single user.
    pub user: Option<&'a str>,
    /// Filter files appearing in a specific channel, indicated by its ID.
    pub channel: Option<&'a str>,
    /// Filter files created after this timestamp (inclusive).
    pub ts_from: Option<u32>,
    /// Filter files created before this timestamp (inclusive).
    pub ts_to: Option<u32>,
    /// Filter files by type:
    ///
    ///
    /// all - All files
    /// spaces - Posts
    /// snippets - Snippets
    /// images - Image files
    /// gdocs - Google docs
    /// zips - Zip files
    /// pdfs - PDF files
    ///
    ///
    /// You can pass multiple values in the types argument, like types=spaces,snippets.The default value is all, which does not filter the list.
    pub types: Option<&'a str>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    error: Option<String>,
    pub files: Option<Vec<::File>>,
    #[serde(default)]
    ok: bool,
    pub paging: Option<::Paging>,
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
    /// Value passed for types was invalid
    UnknownType,
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
            "unknown_type" => ListError::UnknownType,
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
            &ListError::UnknownType => "unknown_type",
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

/// Revokes public/external sharing access for a file
///
/// Wraps https://api.slack.com/methods/files.revokePublicURL

pub fn revoke_public_url<R>(client: &R,
                            request: &RevokePublicURLRequest)
                            -> Result<RevokePublicURLResponse, RevokePublicURLError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("file", request.file))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("files.revokePublicURL", &params[..])
        .map_err(|err| RevokePublicURLError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<RevokePublicURLResponse>(&result)
                .map_err(|_| RevokePublicURLError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct RevokePublicURLRequest<'a> {
    /// Authentication token.
    /// Requires scope: files:write:user
    pub token: &'a str,
    /// File to revoke
    pub file: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevokePublicURLResponse {
    error: Option<String>,
    pub file: Option<::File>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<RevokePublicURLResponse, RevokePublicURLError<E>>> for RevokePublicURLResponse {
    fn into(self) -> Result<RevokePublicURLResponse, RevokePublicURLError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum RevokePublicURLError<E: Error> {
    /// Value passed for file was invalid
    FileNotFound,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
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

impl<'a, E: Error> From<&'a str> for RevokePublicURLError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "file_not_found" => RevokePublicURLError::FileNotFound,
            "not_authed" => RevokePublicURLError::NotAuthed,
            "invalid_auth" => RevokePublicURLError::InvalidAuth,
            "account_inactive" => RevokePublicURLError::AccountInactive,
            "user_is_bot" => RevokePublicURLError::UserIsBot,
            "user_is_restricted" => RevokePublicURLError::UserIsRestricted,
            "invalid_arg_name" => RevokePublicURLError::InvalidArgName,
            "invalid_array_arg" => RevokePublicURLError::InvalidArrayArg,
            "invalid_charset" => RevokePublicURLError::InvalidCharset,
            "invalid_form_data" => RevokePublicURLError::InvalidFormData,
            "invalid_post_type" => RevokePublicURLError::InvalidPostType,
            "missing_post_type" => RevokePublicURLError::MissingPostType,
            "request_timeout" => RevokePublicURLError::RequestTimeout,
            _ => RevokePublicURLError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RevokePublicURLError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for RevokePublicURLError<E> {
    fn description(&self) -> &str {
        match self {
            &RevokePublicURLError::FileNotFound => "file_not_found",
            &RevokePublicURLError::NotAuthed => "not_authed",
            &RevokePublicURLError::InvalidAuth => "invalid_auth",
            &RevokePublicURLError::AccountInactive => "account_inactive",
            &RevokePublicURLError::UserIsBot => "user_is_bot",
            &RevokePublicURLError::UserIsRestricted => "user_is_restricted",
            &RevokePublicURLError::InvalidArgName => "invalid_arg_name",
            &RevokePublicURLError::InvalidArrayArg => "invalid_array_arg",
            &RevokePublicURLError::InvalidCharset => "invalid_charset",
            &RevokePublicURLError::InvalidFormData => "invalid_form_data",
            &RevokePublicURLError::InvalidPostType => "invalid_post_type",
            &RevokePublicURLError::MissingPostType => "missing_post_type",
            &RevokePublicURLError::RequestTimeout => "request_timeout",
            &RevokePublicURLError::MalformedResponse => "Malformed response data from Slack.",
            &RevokePublicURLError::Unknown(ref s) => s,
            &RevokePublicURLError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &RevokePublicURLError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Enables a file for public/external sharing.
///
/// Wraps https://api.slack.com/methods/files.sharedPublicURL

pub fn shared_public_url<R>(client: &R,
                            request: &SharedPublicURLRequest)
                            -> Result<SharedPublicURLResponse, SharedPublicURLError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("file", request.file))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("files.sharedPublicURL", &params[..])
        .map_err(|err| SharedPublicURLError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<SharedPublicURLResponse>(&result)
                .map_err(|_| SharedPublicURLError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SharedPublicURLRequest<'a> {
    /// Authentication token.
    /// Requires scope: files:write:user
    pub token: &'a str,
    /// File to share
    pub file: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SharedPublicURLResponse {
    error: Option<String>,
    pub file: Option<::File>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<SharedPublicURLResponse, SharedPublicURLError<E>>> for SharedPublicURLResponse {
    fn into(self) -> Result<SharedPublicURLResponse, SharedPublicURLError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum SharedPublicURLError<E: Error> {
    /// Value passed for file was invalid
    FileNotFound,
    /// Public sharing has been disabled for this team
    NotAllowed,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
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

impl<'a, E: Error> From<&'a str> for SharedPublicURLError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "file_not_found" => SharedPublicURLError::FileNotFound,
            "not_allowed" => SharedPublicURLError::NotAllowed,
            "not_authed" => SharedPublicURLError::NotAuthed,
            "invalid_auth" => SharedPublicURLError::InvalidAuth,
            "account_inactive" => SharedPublicURLError::AccountInactive,
            "user_is_bot" => SharedPublicURLError::UserIsBot,
            "user_is_restricted" => SharedPublicURLError::UserIsRestricted,
            "invalid_arg_name" => SharedPublicURLError::InvalidArgName,
            "invalid_array_arg" => SharedPublicURLError::InvalidArrayArg,
            "invalid_charset" => SharedPublicURLError::InvalidCharset,
            "invalid_form_data" => SharedPublicURLError::InvalidFormData,
            "invalid_post_type" => SharedPublicURLError::InvalidPostType,
            "missing_post_type" => SharedPublicURLError::MissingPostType,
            "request_timeout" => SharedPublicURLError::RequestTimeout,
            _ => SharedPublicURLError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SharedPublicURLError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for SharedPublicURLError<E> {
    fn description(&self) -> &str {
        match self {
            &SharedPublicURLError::FileNotFound => "file_not_found",
            &SharedPublicURLError::NotAllowed => "not_allowed",
            &SharedPublicURLError::NotAuthed => "not_authed",
            &SharedPublicURLError::InvalidAuth => "invalid_auth",
            &SharedPublicURLError::AccountInactive => "account_inactive",
            &SharedPublicURLError::UserIsBot => "user_is_bot",
            &SharedPublicURLError::UserIsRestricted => "user_is_restricted",
            &SharedPublicURLError::InvalidArgName => "invalid_arg_name",
            &SharedPublicURLError::InvalidArrayArg => "invalid_array_arg",
            &SharedPublicURLError::InvalidCharset => "invalid_charset",
            &SharedPublicURLError::InvalidFormData => "invalid_form_data",
            &SharedPublicURLError::InvalidPostType => "invalid_post_type",
            &SharedPublicURLError::MissingPostType => "missing_post_type",
            &SharedPublicURLError::RequestTimeout => "request_timeout",
            &SharedPublicURLError::MalformedResponse => "Malformed response data from Slack.",
            &SharedPublicURLError::Unknown(ref s) => s,
            &SharedPublicURLError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SharedPublicURLError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Uploads or creates a file.
///
/// Wraps https://api.slack.com/methods/files.upload

pub fn upload<R>(client: &R, request: &UploadRequest) -> Result<UploadResponse, UploadError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      request.file.map(|file| ("file", file)),
                      request.content.map(|content| ("content", content)),
                      request.filetype.map(|filetype| ("filetype", filetype)),
                      Some(("filename", request.filename)),
                      request.title.map(|title| ("title", title)),
                      request.initial_comment
                          .map(|initial_comment| ("initial_comment", initial_comment)),
                      request.channels.map(|channels| ("channels", channels))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("files.upload", &params[..])
        .map_err(|err| UploadError::Client(err))
        .and_then(|result| serde_json::from_str::<UploadResponse>(&result).map_err(|_| UploadError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct UploadRequest<'a> {
    /// Authentication token.
    /// Requires scope: files:write:user
    pub token: &'a str,
    /// File contents via multipart/form-data. If omitting this parameter, you must submit content.
    pub file: Option<&'a str>,
    /// File contents via a POST variable. If omitting this parameter, you must provide a file.
    pub content: Option<&'a str>,
    /// A file type identifier.
    pub filetype: Option<&'a str>,
    /// Filename of file.
    pub filename: &'a str,
    /// Title of file.
    pub title: Option<&'a str>,
    /// Initial comment to add to file.
    pub initial_comment: Option<&'a str>,
    /// Comma-separated list of channel names or IDs where the file will be shared.
    pub channels: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UploadResponse {
    error: Option<String>,
    pub file: Option<::File>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<UploadResponse, UploadError<E>>> for UploadResponse {
    fn into(self) -> Result<UploadResponse, UploadError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum UploadError<E: Error> {
    /// An admin has restricted posting to the #general channel.
    PostingToGeneralChannelDenied,
    /// One or more channels supplied are invalid
    InvalidChannel,
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

impl<'a, E: Error> From<&'a str> for UploadError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "posting_to_general_channel_denied" => UploadError::PostingToGeneralChannelDenied,
            "invalid_channel" => UploadError::InvalidChannel,
            "not_authed" => UploadError::NotAuthed,
            "invalid_auth" => UploadError::InvalidAuth,
            "account_inactive" => UploadError::AccountInactive,
            "invalid_arg_name" => UploadError::InvalidArgName,
            "invalid_array_arg" => UploadError::InvalidArrayArg,
            "invalid_charset" => UploadError::InvalidCharset,
            "invalid_form_data" => UploadError::InvalidFormData,
            "invalid_post_type" => UploadError::InvalidPostType,
            "missing_post_type" => UploadError::MissingPostType,
            "request_timeout" => UploadError::RequestTimeout,
            _ => UploadError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UploadError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for UploadError<E> {
    fn description(&self) -> &str {
        match self {
            &UploadError::PostingToGeneralChannelDenied => "posting_to_general_channel_denied",
            &UploadError::InvalidChannel => "invalid_channel",
            &UploadError::NotAuthed => "not_authed",
            &UploadError::InvalidAuth => "invalid_auth",
            &UploadError::AccountInactive => "account_inactive",
            &UploadError::InvalidArgName => "invalid_arg_name",
            &UploadError::InvalidArrayArg => "invalid_array_arg",
            &UploadError::InvalidCharset => "invalid_charset",
            &UploadError::InvalidFormData => "invalid_form_data",
            &UploadError::InvalidPostType => "invalid_post_type",
            &UploadError::MissingPostType => "missing_post_type",
            &UploadError::RequestTimeout => "request_timeout",
            &UploadError::MalformedResponse => "Malformed response data from Slack.",
            &UploadError::Unknown(ref s) => s,
            &UploadError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &UploadError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
