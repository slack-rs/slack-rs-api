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

//! Adjust and view Do Not Disturb settings for team members.

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use crate::requests::SlackWebRequestSender;

/// Ends the current user's Do Not Disturb session immediately.
///
/// Wraps https://api.slack.com/methods/dnd.endDnd

pub async fn end_dnd<R>(client: &R, token: &str) -> Result<EndDndResponse, EndDndError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = crate::get_slack_url_for_method("dnd.endDnd");
    client
        .send(&url, &params[..])
        .await
        .map_err(EndDndError::Client)
        .and_then(|result| {
            serde_json::from_str::<EndDndResponse>(&result)
                .map_err(|e| EndDndError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Debug, Deserialize)]
pub struct EndDndResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<EndDndResponse, EndDndError<E>>> for EndDndResponse {
    fn into(self) -> Result<EndDndResponse, EndDndError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum EndDndError<E: Error> {
    /// There was a mysterious problem ending the user's Do Not Disturb session
    UnknownError,
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

impl<'a, E: Error> From<&'a str> for EndDndError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "unknown_error" => EndDndError::UnknownError,
            "not_authed" => EndDndError::NotAuthed,
            "invalid_auth" => EndDndError::InvalidAuth,
            "account_inactive" => EndDndError::AccountInactive,
            "user_is_bot" => EndDndError::UserIsBot,
            "invalid_arg_name" => EndDndError::InvalidArgName,
            "invalid_array_arg" => EndDndError::InvalidArrayArg,
            "invalid_charset" => EndDndError::InvalidCharset,
            "invalid_form_data" => EndDndError::InvalidFormData,
            "invalid_post_type" => EndDndError::InvalidPostType,
            "missing_post_type" => EndDndError::MissingPostType,
            "team_added_to_org" => EndDndError::TeamAddedToOrg,
            "request_timeout" => EndDndError::RequestTimeout,
            _ => EndDndError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EndDndError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for EndDndError<E> {
    fn description(&self) -> &str {
        match *self {
                        EndDndError::UnknownError => "unknown_error: There was a mysterious problem ending the user's Do Not Disturb session",
EndDndError::NotAuthed => "not_authed: No authentication token provided.",
EndDndError::InvalidAuth => "invalid_auth: Invalid authentication token.",
EndDndError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
EndDndError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
EndDndError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
EndDndError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
EndDndError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
EndDndError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
EndDndError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
EndDndError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
EndDndError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
EndDndError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        EndDndError::MalformedResponse(_, ref e) => e.description(),
                        EndDndError::Unknown(ref s) => s,
                        EndDndError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            EndDndError::MalformedResponse(_, ref e) => Some(e),
            EndDndError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Ends the current user's snooze mode immediately.
///
/// Wraps https://api.slack.com/methods/dnd.endSnooze

pub async fn end_snooze<R>(
    client: &R,
    token: &str,
) -> Result<EndSnoozeResponse, EndSnoozeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = crate::get_slack_url_for_method("dnd.endSnooze");
    client
        .send(&url, &params[..])
        .await
        .map_err(EndSnoozeError::Client)
        .and_then(|result| {
            serde_json::from_str::<EndSnoozeResponse>(&result)
                .map_err(|e| EndSnoozeError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Debug, Deserialize)]
pub struct EndSnoozeResponse {
    pub dnd_enabled: Option<bool>,
    error: Option<String>,
    pub next_dnd_end_ts: Option<crate::Timestamp>,
    pub next_dnd_start_ts: Option<crate::Timestamp>,
    #[serde(default)]
    ok: bool,
    pub snooze_enabled: Option<bool>,
}

impl<E: Error> Into<Result<EndSnoozeResponse, EndSnoozeError<E>>> for EndSnoozeResponse {
    fn into(self) -> Result<EndSnoozeResponse, EndSnoozeError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum EndSnoozeError<E: Error> {
    /// Snooze is not active for this user and cannot be ended
    SnoozeNotActive,
    /// There was a problem setting the user's Do Not Disturb status
    SnoozeEndFailed,
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

impl<'a, E: Error> From<&'a str> for EndSnoozeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "snooze_not_active" => EndSnoozeError::SnoozeNotActive,
            "snooze_end_failed" => EndSnoozeError::SnoozeEndFailed,
            "not_authed" => EndSnoozeError::NotAuthed,
            "invalid_auth" => EndSnoozeError::InvalidAuth,
            "account_inactive" => EndSnoozeError::AccountInactive,
            "user_is_bot" => EndSnoozeError::UserIsBot,
            "invalid_arg_name" => EndSnoozeError::InvalidArgName,
            "invalid_array_arg" => EndSnoozeError::InvalidArrayArg,
            "invalid_charset" => EndSnoozeError::InvalidCharset,
            "invalid_form_data" => EndSnoozeError::InvalidFormData,
            "invalid_post_type" => EndSnoozeError::InvalidPostType,
            "missing_post_type" => EndSnoozeError::MissingPostType,
            "team_added_to_org" => EndSnoozeError::TeamAddedToOrg,
            "request_timeout" => EndSnoozeError::RequestTimeout,
            _ => EndSnoozeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EndSnoozeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for EndSnoozeError<E> {
    fn description(&self) -> &str {
        match *self {
                        EndSnoozeError::SnoozeNotActive => "snooze_not_active: Snooze is not active for this user and cannot be ended",
EndSnoozeError::SnoozeEndFailed => "snooze_end_failed: There was a problem setting the user's Do Not Disturb status",
EndSnoozeError::NotAuthed => "not_authed: No authentication token provided.",
EndSnoozeError::InvalidAuth => "invalid_auth: Invalid authentication token.",
EndSnoozeError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
EndSnoozeError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
EndSnoozeError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
EndSnoozeError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
EndSnoozeError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
EndSnoozeError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
EndSnoozeError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
EndSnoozeError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
EndSnoozeError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
EndSnoozeError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        EndSnoozeError::MalformedResponse(_, ref e) => e.description(),
                        EndSnoozeError::Unknown(ref s) => s,
                        EndSnoozeError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            EndSnoozeError::MalformedResponse(_, ref e) => Some(e),
            EndSnoozeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Retrieves a user's current Do Not Disturb status.
///
/// Wraps https://api.slack.com/methods/dnd.info

pub async fn info<R>(
    client: &R,
    token: &str,
    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request.user.map(|user| ("user", user)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("dnd.info");
    client
        .send(&url, &params[..])
        .await
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// User to fetch status for (defaults to current user)
    pub user: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub dnd_enabled: Option<bool>,
    error: Option<String>,
    pub next_dnd_end_ts: Option<crate::Timestamp>,
    pub next_dnd_start_ts: Option<crate::Timestamp>,
    #[serde(default)]
    ok: bool,
    pub snooze_enabled: Option<bool>,
    pub snooze_endtime: Option<crate::Timestamp>,
    pub snooze_remaining: Option<f32>,
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
#[derive(Debug)]
pub enum InfoError<E: Error> {
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
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "user_not_found" => InfoError::UserNotFound,
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_post_type" => InfoError::InvalidPostType,
            "missing_post_type" => InfoError::MissingPostType,
            "team_added_to_org" => InfoError::TeamAddedToOrg,
            "request_timeout" => InfoError::RequestTimeout,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for InfoError<E> {
    fn description(&self) -> &str {
        match *self {
                        InfoError::UserNotFound => "user_not_found: Value passed for user was invalid.",
InfoError::NotAuthed => "not_authed: No authentication token provided.",
InfoError::InvalidAuth => "invalid_auth: Invalid authentication token.",
InfoError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
InfoError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
InfoError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
InfoError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
InfoError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
InfoError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
InfoError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
InfoError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
InfoError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        InfoError::MalformedResponse(_, ref e) => e.description(),
                        InfoError::Unknown(ref s) => s,
                        InfoError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            InfoError::MalformedResponse(_, ref e) => Some(e),
            InfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Turns on Do Not Disturb mode for the current user, or changes its duration.
///
/// Wraps https://api.slack.com/methods/dnd.setSnooze

pub async fn set_snooze<R>(
    client: &R,
    token: &str,
    request: &SetSnoozeRequest,
) -> Result<SetSnoozeResponse, SetSnoozeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let num_minutes = request.num_minutes.to_string();
    let params = vec![
        Some(("token", token)),
        Some(("num_minutes", &num_minutes[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("dnd.setSnooze");
    client
        .send(&url, &params[..])
        .await
        .map_err(SetSnoozeError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetSnoozeResponse>(&result)
                .map_err(|e| SetSnoozeError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SetSnoozeRequest {
    /// Number of minutes, from now, to snooze until.
    pub num_minutes: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetSnoozeResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub snooze_enabled: Option<bool>,
    pub snooze_endtime: Option<crate::Timestamp>,
    pub snooze_remaining: Option<f32>,
}

impl<E: Error> Into<Result<SetSnoozeResponse, SetSnoozeError<E>>> for SetSnoozeResponse {
    fn into(self) -> Result<SetSnoozeResponse, SetSnoozeError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum SetSnoozeError<E: Error> {
    /// No value provided for num_minutes
    MissingDuration,
    /// There was a problem setting the user's Do Not Disturb status
    SnoozeFailed,
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

impl<'a, E: Error> From<&'a str> for SetSnoozeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "missing_duration" => SetSnoozeError::MissingDuration,
            "snooze_failed" => SetSnoozeError::SnoozeFailed,
            "not_authed" => SetSnoozeError::NotAuthed,
            "invalid_auth" => SetSnoozeError::InvalidAuth,
            "account_inactive" => SetSnoozeError::AccountInactive,
            "user_is_bot" => SetSnoozeError::UserIsBot,
            "invalid_arg_name" => SetSnoozeError::InvalidArgName,
            "invalid_array_arg" => SetSnoozeError::InvalidArrayArg,
            "invalid_charset" => SetSnoozeError::InvalidCharset,
            "invalid_form_data" => SetSnoozeError::InvalidFormData,
            "invalid_post_type" => SetSnoozeError::InvalidPostType,
            "missing_post_type" => SetSnoozeError::MissingPostType,
            "team_added_to_org" => SetSnoozeError::TeamAddedToOrg,
            "request_timeout" => SetSnoozeError::RequestTimeout,
            _ => SetSnoozeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetSnoozeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for SetSnoozeError<E> {
    fn description(&self) -> &str {
        match *self {
                        SetSnoozeError::MissingDuration => "missing_duration: No value provided for num_minutes",
SetSnoozeError::SnoozeFailed => "snooze_failed: There was a problem setting the user's Do Not Disturb status",
SetSnoozeError::NotAuthed => "not_authed: No authentication token provided.",
SetSnoozeError::InvalidAuth => "invalid_auth: Invalid authentication token.",
SetSnoozeError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
SetSnoozeError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
SetSnoozeError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
SetSnoozeError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
SetSnoozeError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
SetSnoozeError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
SetSnoozeError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
SetSnoozeError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
SetSnoozeError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
SetSnoozeError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        SetSnoozeError::MalformedResponse(_, ref e) => e.description(),
                        SetSnoozeError::Unknown(ref s) => s,
                        SetSnoozeError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SetSnoozeError::MalformedResponse(_, ref e) => Some(e),
            SetSnoozeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Retrieves the Do Not Disturb status for users on a team.
///
/// Wraps https://api.slack.com/methods/dnd.teamInfo

pub async fn team_info<R>(
    client: &R,
    token: &str,
    request: &TeamInfoRequest<'_>,
) -> Result<TeamInfoResponse, TeamInfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request.users.map(|users| ("users", users)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("dnd.teamInfo");
    client
        .send(&url, &params[..])
        .await
        .map_err(TeamInfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<TeamInfoResponse>(&result)
                .map_err(|e| TeamInfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct TeamInfoRequest<'a> {
    /// Comma-separated list of users to fetch Do Not Disturb status for
    pub users: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeamInfoResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub users: Option<HashMap<String, bool>>,
}

impl<E: Error> Into<Result<TeamInfoResponse, TeamInfoError<E>>> for TeamInfoResponse {
    fn into(self) -> Result<TeamInfoResponse, TeamInfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum TeamInfoError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for TeamInfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => TeamInfoError::NotAuthed,
            "invalid_auth" => TeamInfoError::InvalidAuth,
            "account_inactive" => TeamInfoError::AccountInactive,
            "invalid_arg_name" => TeamInfoError::InvalidArgName,
            "invalid_array_arg" => TeamInfoError::InvalidArrayArg,
            "invalid_charset" => TeamInfoError::InvalidCharset,
            "invalid_form_data" => TeamInfoError::InvalidFormData,
            "invalid_post_type" => TeamInfoError::InvalidPostType,
            "missing_post_type" => TeamInfoError::MissingPostType,
            "team_added_to_org" => TeamInfoError::TeamAddedToOrg,
            "request_timeout" => TeamInfoError::RequestTimeout,
            _ => TeamInfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for TeamInfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for TeamInfoError<E> {
    fn description(&self) -> &str {
        match *self {
                        TeamInfoError::NotAuthed => "not_authed: No authentication token provided.",
TeamInfoError::InvalidAuth => "invalid_auth: Invalid authentication token.",
TeamInfoError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
TeamInfoError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
TeamInfoError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
TeamInfoError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
TeamInfoError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
TeamInfoError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
TeamInfoError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
TeamInfoError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
TeamInfoError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        TeamInfoError::MalformedResponse(_, ref e) => e.description(),
                        TeamInfoError::Unknown(ref s) => s,
                        TeamInfoError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            TeamInfoError::MalformedResponse(_, ref e) => Some(e),
            TeamInfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
