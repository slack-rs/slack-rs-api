
#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Add a comment to an existing file.
///
/// Wraps https://api.slack.com/methods/files.comments.add

pub fn add<R>(client: &R, request: &AddRequest) -> Result<AddResponse, AddError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      Some(("file", request.file)),
                      Some(("comment", request.comment)),
                      request.channel.map(|channel| ("channel", channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("files.comments.add", &params[..])
        .map_err(|err| AddError::Client(err))
        .and_then(|result| serde_json::from_str::<AddResponse>(&result).map_err(|_| AddError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct AddRequest<'a> {
    /// Authentication token.
    /// Requires scope: files:write:user
    pub token: &'a str,
    /// File to add a comment to.
    pub file: &'a str,
    /// Text of the comment to add.
    pub comment: &'a str,
    /// Channel id (encoded) of which location to associate with the new comment.
    pub channel: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddResponse {
    pub comment: Option<::FileComment>,
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
    /// The requested file could not be found.
    FileNotFound,
    /// The requested file was previously deleted.
    FileDeleted,
    /// The comment field was empty.
    NoComment,
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
            "file_not_found" => AddError::FileNotFound,
            "file_deleted" => AddError::FileDeleted,
            "no_comment" => AddError::NoComment,
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
            &AddError::FileNotFound => "file_not_found",
            &AddError::FileDeleted => "file_deleted",
            &AddError::NoComment => "no_comment",
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

/// Deletes an existing comment on a file.
///
/// Wraps https://api.slack.com/methods/files.comments.delete

pub fn delete<R>(client: &R, request: &DeleteRequest) -> Result<DeleteResponse, DeleteError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("file", request.file)), Some(("id", request.id))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("files.comments.delete", &params[..])
        .map_err(|err| DeleteError::Client(err))
        .and_then(|result| serde_json::from_str::<DeleteResponse>(&result).map_err(|_| DeleteError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest<'a> {
    /// Authentication token.
    /// Requires scope: files:write:user
    pub token: &'a str,
    /// File to delete a comment from.
    pub file: &'a str,
    /// The comment to delete.
    pub id: &'a str,
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
    /// The requested file could not be found.
    FileNotFound,
    /// The requested file was previously deleted.
    FileDeleted,
    /// The requested comment could not be deleted.
    CantDelete,
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
            "cant_delete" => DeleteError::CantDelete,
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
            &DeleteError::CantDelete => "cant_delete",
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

/// Edit an existing file comment.
///
/// Wraps https://api.slack.com/methods/files.comments.edit

pub fn edit<R>(client: &R, request: &EditRequest) -> Result<EditResponse, EditError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      Some(("file", request.file)),
                      Some(("id", request.id)),
                      Some(("comment", request.comment))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("files.comments.edit", &params[..])
        .map_err(|err| EditError::Client(err))
        .and_then(|result| serde_json::from_str::<EditResponse>(&result).map_err(|_| EditError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct EditRequest<'a> {
    /// Authentication token.
    /// Requires scope: files:write:user
    pub token: &'a str,
    /// File containing the comment to edit.
    pub file: &'a str,
    /// The comment to edit.
    pub id: &'a str,
    /// Text of the comment to edit.
    pub comment: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EditResponse {
    pub comment: Option<::FileComment>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<EditResponse, EditError<E>>> for EditResponse {
    fn into(self) -> Result<EditResponse, EditError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum EditError<E: Error> {
    /// The requested file could not be found.
    FileNotFound,
    /// The requested file was previously deleted.
    FileDeleted,
    /// The comment field was empty.
    NoComment,
    /// The timeframe for editing the comment has expired.
    EditWindowClosed,
    /// The requested file could not be found.
    CantEdit,
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

impl<'a, E: Error> From<&'a str> for EditError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "file_not_found" => EditError::FileNotFound,
            "file_deleted" => EditError::FileDeleted,
            "no_comment" => EditError::NoComment,
            "edit_window_closed" => EditError::EditWindowClosed,
            "cant_edit" => EditError::CantEdit,
            "not_authed" => EditError::NotAuthed,
            "invalid_auth" => EditError::InvalidAuth,
            "account_inactive" => EditError::AccountInactive,
            "invalid_arg_name" => EditError::InvalidArgName,
            "invalid_array_arg" => EditError::InvalidArrayArg,
            "invalid_charset" => EditError::InvalidCharset,
            "invalid_form_data" => EditError::InvalidFormData,
            "invalid_post_type" => EditError::InvalidPostType,
            "missing_post_type" => EditError::MissingPostType,
            "request_timeout" => EditError::RequestTimeout,
            _ => EditError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EditError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for EditError<E> {
    fn description(&self) -> &str {
        match self {
            &EditError::FileNotFound => "file_not_found",
            &EditError::FileDeleted => "file_deleted",
            &EditError::NoComment => "no_comment",
            &EditError::EditWindowClosed => "edit_window_closed",
            &EditError::CantEdit => "cant_edit",
            &EditError::NotAuthed => "not_authed",
            &EditError::InvalidAuth => "invalid_auth",
            &EditError::AccountInactive => "account_inactive",
            &EditError::InvalidArgName => "invalid_arg_name",
            &EditError::InvalidArrayArg => "invalid_array_arg",
            &EditError::InvalidCharset => "invalid_charset",
            &EditError::InvalidFormData => "invalid_form_data",
            &EditError::InvalidPostType => "invalid_post_type",
            &EditError::MissingPostType => "missing_post_type",
            &EditError::RequestTimeout => "request_timeout",
            &EditError::MalformedResponse => "Malformed response data from Slack.",
            &EditError::Unknown(ref s) => s,
            &EditError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &EditError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
