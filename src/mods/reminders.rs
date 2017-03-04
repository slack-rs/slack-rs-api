
#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

#[allow(unused_imports)]
use ToResult;
use requests::SlackWebRequestSender;

/// Creates a reminder.
///
/// Wraps https://api.slack.com/methods/reminders.add

pub fn add<R>(client: &R, request: &AddRequest) -> Result<AddResponse, AddError<R::Error>>
    where R: SlackWebRequestSender
{
    let time = request.time.to_string();
    let params = vec![Some(("token", request.token)),
                      Some(("text", request.text)),
                      Some(("time", &time[..])),
                      request.user.map(|user| ("user", user))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("reminders.add", &params[..])
        .map_err(|err| AddError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result).map_err(|_| AddError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct AddRequest<'a> {
    /// Authentication token.
    /// Requires scope: reminders:write
    pub token: &'a str,
    /// The content of the reminder
    pub text: &'a str,
    /// When this reminder should happen: the Unix timestamp (up to five years from now), the number of seconds until the reminder (if within 24 hours), or a natural language description (Ex. "in 15 minutes," or "every Thursday")
    pub time: u32,
    /// The user who will receive the reminder. If no user is specified, the reminder will go to user who created it.
    pub user: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub reminder: Option<::Reminder>,
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
    /// The phrasing of the timing for this reminder is unclear. You must include a complete time description. Some examples that work: 1458678068, 20, in 5 minutes, tomorrow, at 3:30pm, on Tuesday, or next week.
    CannotParse,
    /// That user can't be found.
    UserNotFound,
    /// Reminders can't be sent to bots.
    CannotAddBot,
    /// Reminders can't be sent to Slackbot.
    CannotAddSlackbot,
    /// Guests can't set reminders for other team members.
    CannotAddOthers,
    /// Recurring reminders can't be set for other team members.
    CannotAddOthersRecurring,
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

impl<'a, E: Error> From<&'a str> for AddError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "cannot_parse" => AddError::CannotParse,
            "user_not_found" => AddError::UserNotFound,
            "cannot_add_bot" => AddError::CannotAddBot,
            "cannot_add_slackbot" => AddError::CannotAddSlackbot,
            "cannot_add_others" => AddError::CannotAddOthers,
            "cannot_add_others_recurring" => AddError::CannotAddOthersRecurring,
            "not_authed" => AddError::NotAuthed,
            "invalid_auth" => AddError::InvalidAuth,
            "account_inactive" => AddError::AccountInactive,
            "user_is_bot" => AddError::UserIsBot,
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
            &AddError::CannotParse => "cannot_parse",
            &AddError::UserNotFound => "user_not_found",
            &AddError::CannotAddBot => "cannot_add_bot",
            &AddError::CannotAddSlackbot => "cannot_add_slackbot",
            &AddError::CannotAddOthers => "cannot_add_others",
            &AddError::CannotAddOthersRecurring => "cannot_add_others_recurring",
            &AddError::NotAuthed => "not_authed",
            &AddError::InvalidAuth => "invalid_auth",
            &AddError::AccountInactive => "account_inactive",
            &AddError::UserIsBot => "user_is_bot",
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

/// Marks a reminder as complete.
///
/// Wraps https://api.slack.com/methods/reminders.complete

pub fn complete<R>(client: &R,
                   request: &CompleteRequest)
                   -> Result<CompleteResponse, CompleteError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("reminder", request.reminder))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("reminders.complete", &params[..])
        .map_err(|err| CompleteError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<CompleteResponse>(&result)
                .map_err(|_| CompleteError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct CompleteRequest<'a> {
    /// Authentication token.
    /// Requires scope: reminders:write
    pub token: &'a str,
    /// The ID of the reminder to be marked as complete
    pub reminder: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CompleteResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> ToResult<CompleteResponse, CompleteError<E>> for CompleteResponse {
    fn to_result(self) -> Result<CompleteResponse, CompleteError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum CompleteError<E: Error> {
    /// That reminder can't be found.
    NotFound,
    /// Recurring reminders can't be marked complete.
    CannotCompleteRecurring,
    /// Reminders for other team members can't be marked complete.
    CannotCompleteOthers,
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

impl<'a, E: Error> From<&'a str> for CompleteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_found" => CompleteError::NotFound,
            "cannot_complete_recurring" => CompleteError::CannotCompleteRecurring,
            "cannot_complete_others" => CompleteError::CannotCompleteOthers,
            "not_authed" => CompleteError::NotAuthed,
            "invalid_auth" => CompleteError::InvalidAuth,
            "account_inactive" => CompleteError::AccountInactive,
            "user_is_bot" => CompleteError::UserIsBot,
            "invalid_arg_name" => CompleteError::InvalidArgName,
            "invalid_array_arg" => CompleteError::InvalidArrayArg,
            "invalid_charset" => CompleteError::InvalidCharset,
            "invalid_form_data" => CompleteError::InvalidFormData,
            "invalid_post_type" => CompleteError::InvalidPostType,
            "missing_post_type" => CompleteError::MissingPostType,
            "request_timeout" => CompleteError::RequestTimeout,
            _ => CompleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CompleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for CompleteError<E> {
    fn description(&self) -> &str {
        match self {
            &CompleteError::NotFound => "not_found",
            &CompleteError::CannotCompleteRecurring => "cannot_complete_recurring",
            &CompleteError::CannotCompleteOthers => "cannot_complete_others",
            &CompleteError::NotAuthed => "not_authed",
            &CompleteError::InvalidAuth => "invalid_auth",
            &CompleteError::AccountInactive => "account_inactive",
            &CompleteError::UserIsBot => "user_is_bot",
            &CompleteError::InvalidArgName => "invalid_arg_name",
            &CompleteError::InvalidArrayArg => "invalid_array_arg",
            &CompleteError::InvalidCharset => "invalid_charset",
            &CompleteError::InvalidFormData => "invalid_form_data",
            &CompleteError::InvalidPostType => "invalid_post_type",
            &CompleteError::MissingPostType => "missing_post_type",
            &CompleteError::RequestTimeout => "request_timeout",
            &CompleteError::MalformedResponse => "Malformed response data from Slack.",
            &CompleteError::Unknown(ref s) => s,
            &CompleteError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &CompleteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Deletes a reminder.
///
/// Wraps https://api.slack.com/methods/reminders.delete

pub fn delete<R>(client: &R,
                 request: &DeleteRequest)
                 -> Result<DeleteResponse, DeleteError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("reminder", request.reminder))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("reminders.delete", &params[..])
        .map_err(|err| DeleteError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|_| DeleteError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest<'a> {
    /// Authentication token.
    /// Requires scope: reminders:write
    pub token: &'a str,
    /// The ID of the reminder
    pub reminder: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> ToResult<DeleteResponse, DeleteError<E>> for DeleteResponse {
    fn to_result(self) -> Result<DeleteResponse, DeleteError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum DeleteError<E: Error> {
    /// That reminder can't be found.
    NotFound,
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

impl<'a, E: Error> From<&'a str> for DeleteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_found" => DeleteError::NotFound,
            "not_authed" => DeleteError::NotAuthed,
            "invalid_auth" => DeleteError::InvalidAuth,
            "account_inactive" => DeleteError::AccountInactive,
            "user_is_bot" => DeleteError::UserIsBot,
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
            &DeleteError::NotFound => "not_found",
            &DeleteError::NotAuthed => "not_authed",
            &DeleteError::InvalidAuth => "invalid_auth",
            &DeleteError::AccountInactive => "account_inactive",
            &DeleteError::UserIsBot => "user_is_bot",
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

/// Gets information about a reminder.
///
/// Wraps https://api.slack.com/methods/reminders.info

pub fn info<R>(client: &R, request: &InfoRequest) -> Result<InfoResponse, InfoError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("reminder", request.reminder))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("reminders.info", &params[..])
        .map_err(|err| InfoError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result).map_err(|_| InfoError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Authentication token.
    /// Requires scope: reminders:read
    pub token: &'a str,
    /// The ID of the reminder
    pub reminder: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub reminder: Option<::Reminder>,
}


impl<E: Error> ToResult<InfoResponse, InfoError<E>> for InfoResponse {
    fn to_result(self) -> Result<InfoResponse, InfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum InfoError<E: Error> {
    /// That reminder can't be found.
    NotFound,
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

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_found" => InfoError::NotFound,
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
            "user_is_bot" => InfoError::UserIsBot,
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
            &InfoError::NotFound => "not_found",
            &InfoError::NotAuthed => "not_authed",
            &InfoError::InvalidAuth => "invalid_auth",
            &InfoError::AccountInactive => "account_inactive",
            &InfoError::UserIsBot => "user_is_bot",
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

/// Lists all reminders created by or for a given user.
///
/// Wraps https://api.slack.com/methods/reminders.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("reminders.list", &params[..])
        .map_err(|err| ListError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(|_| ListError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Authentication token.
    /// Requires scope: reminders:read
    pub token: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub reminders: Option<Vec<::Reminder>>,
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
