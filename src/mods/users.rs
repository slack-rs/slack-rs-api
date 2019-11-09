//! Get info on members of your Slack team.

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Delete the user profile photo
///
/// Wraps https://api.slack.com/methods/users.deletePhoto

pub fn delete_photo<R>(
    client: &R,
    token: &str,
) -> Result<DeletePhotoResponse, DeletePhotoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = ::get_slack_url_for_method("users.deletePhoto");
    client
        .send(&url, &params[..])
        .map_err(DeletePhotoError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeletePhotoResponse>(&result)
                .map_err(DeletePhotoError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeletePhotoResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<DeletePhotoResponse, DeletePhotoError<E>>> for DeletePhotoResponse {
    fn into(self) -> Result<DeletePhotoResponse, DeletePhotoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum DeletePhotoError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for DeletePhotoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => DeletePhotoError::NotAuthed,
            "invalid_auth" => DeletePhotoError::InvalidAuth,
            "account_inactive" => DeletePhotoError::AccountInactive,
            "user_is_bot" => DeletePhotoError::UserIsBot,
            "invalid_arg_name" => DeletePhotoError::InvalidArgName,
            "invalid_array_arg" => DeletePhotoError::InvalidArrayArg,
            "invalid_charset" => DeletePhotoError::InvalidCharset,
            "invalid_form_data" => DeletePhotoError::InvalidFormData,
            "invalid_post_type" => DeletePhotoError::InvalidPostType,
            "missing_post_type" => DeletePhotoError::MissingPostType,
            "team_added_to_org" => DeletePhotoError::TeamAddedToOrg,
            "request_timeout" => DeletePhotoError::RequestTimeout,
            _ => DeletePhotoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeletePhotoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for DeletePhotoError<E> {
    fn description(&self) -> &str {
        match *self {
                        DeletePhotoError::NotAuthed => "not_authed: No authentication token provided.",
DeletePhotoError::InvalidAuth => "invalid_auth: Invalid authentication token.",
DeletePhotoError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
DeletePhotoError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
DeletePhotoError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
DeletePhotoError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
DeletePhotoError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
DeletePhotoError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
DeletePhotoError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
DeletePhotoError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
DeletePhotoError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
DeletePhotoError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        DeletePhotoError::MalformedResponse(ref e) => e.description(),
                        DeletePhotoError::Unknown(ref s) => s,
                        DeletePhotoError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DeletePhotoError::MalformedResponse(ref e) => Some(e),
            DeletePhotoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets user presence information.
///
/// Wraps https://api.slack.com/methods/users.getPresence

pub fn get_presence<R>(
    client: &R,
    token: &str,
    request: &GetPresenceRequest,
) -> Result<GetPresenceResponse, GetPresenceError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("user", request.user))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("users.getPresence");
    client
        .send(&url, &params[..])
        .map_err(GetPresenceError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetPresenceResponse>(&result)
                .map_err(GetPresenceError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct GetPresenceRequest<'a> {
    /// User to get presence info on. Defaults to the authed user.
    pub user: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetPresenceResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub presence: Option<String>,
}

impl<E: Error> Into<Result<GetPresenceResponse, GetPresenceError<E>>> for GetPresenceResponse {
    fn into(self) -> Result<GetPresenceResponse, GetPresenceError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum GetPresenceError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for GetPresenceError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => GetPresenceError::NotAuthed,
            "invalid_auth" => GetPresenceError::InvalidAuth,
            "account_inactive" => GetPresenceError::AccountInactive,
            "invalid_arg_name" => GetPresenceError::InvalidArgName,
            "invalid_array_arg" => GetPresenceError::InvalidArrayArg,
            "invalid_charset" => GetPresenceError::InvalidCharset,
            "invalid_form_data" => GetPresenceError::InvalidFormData,
            "invalid_post_type" => GetPresenceError::InvalidPostType,
            "missing_post_type" => GetPresenceError::MissingPostType,
            "team_added_to_org" => GetPresenceError::TeamAddedToOrg,
            "request_timeout" => GetPresenceError::RequestTimeout,
            _ => GetPresenceError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetPresenceError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for GetPresenceError<E> {
    fn description(&self) -> &str {
        match *self {
                        GetPresenceError::NotAuthed => "not_authed: No authentication token provided.",
GetPresenceError::InvalidAuth => "invalid_auth: Invalid authentication token.",
GetPresenceError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
GetPresenceError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
GetPresenceError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
GetPresenceError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
GetPresenceError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
GetPresenceError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
GetPresenceError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
GetPresenceError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
GetPresenceError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        GetPresenceError::MalformedResponse(ref e) => e.description(),
                        GetPresenceError::Unknown(ref s) => s,
                        GetPresenceError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GetPresenceError::MalformedResponse(ref e) => Some(e),
            GetPresenceError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Get a user's identity.
///
/// Wraps https://api.slack.com/methods/users.identity

pub fn identity<R>(client: &R, token: &str) -> Result<IdentityResponse, IdentityError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = ::get_slack_url_for_method("users.identity");
    client
        .send(&url, &params[..])
        .map_err(IdentityError::Client)
        .and_then(|result| {
            serde_json::from_str::<IdentityResponse>(&result)
                .map_err(IdentityError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Debug, Deserialize)]
pub struct IdentityResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub team: Option<::Team>,
    pub user: Option<::User>,
}

impl<E: Error> Into<Result<IdentityResponse, IdentityError<E>>> for IdentityResponse {
    fn into(self) -> Result<IdentityResponse, IdentityError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum IdentityError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for IdentityError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => IdentityError::NotAuthed,
            "invalid_auth" => IdentityError::InvalidAuth,
            "account_inactive" => IdentityError::AccountInactive,
            "user_is_bot" => IdentityError::UserIsBot,
            "invalid_arg_name" => IdentityError::InvalidArgName,
            "invalid_array_arg" => IdentityError::InvalidArrayArg,
            "invalid_charset" => IdentityError::InvalidCharset,
            "invalid_form_data" => IdentityError::InvalidFormData,
            "invalid_post_type" => IdentityError::InvalidPostType,
            "missing_post_type" => IdentityError::MissingPostType,
            "team_added_to_org" => IdentityError::TeamAddedToOrg,
            "request_timeout" => IdentityError::RequestTimeout,
            _ => IdentityError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for IdentityError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for IdentityError<E> {
    fn description(&self) -> &str {
        match *self {
                        IdentityError::NotAuthed => "not_authed: No authentication token provided.",
IdentityError::InvalidAuth => "invalid_auth: Invalid authentication token.",
IdentityError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
IdentityError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
IdentityError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
IdentityError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
IdentityError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
IdentityError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
IdentityError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
IdentityError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
IdentityError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
IdentityError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        IdentityError::MalformedResponse(ref e) => e.description(),
                        IdentityError::Unknown(ref s) => s,
                        IdentityError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            IdentityError::MalformedResponse(ref e) => Some(e),
            IdentityError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets information about a user.
///
/// Wraps https://api.slack.com/methods/users.info

pub fn info<R>(
    client: &R,
    token: &str,
    request: &InfoRequest,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("user", request.user))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("users.info");
    client
        .send(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result).map_err(InfoError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// User to get info on
    pub user: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub user: Option<::User>,
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
    /// The requested user is not visible to the calling user
    UserNotVisible,
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

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "user_not_found" => InfoError::UserNotFound,
            "user_not_visible" => InfoError::UserNotVisible,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for InfoError<E> {
    fn description(&self) -> &str {
        match *self {
                        InfoError::UserNotFound => "user_not_found: Value passed for user was invalid.",
InfoError::UserNotVisible => "user_not_visible: The requested user is not visible to the calling user",
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
                        InfoError::MalformedResponse(ref e) => e.description(),
                        InfoError::Unknown(ref s) => s,
                        InfoError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            InfoError::MalformedResponse(ref e) => Some(e),
            InfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Lists all users in a Slack team.
///
/// Wraps https://api.slack.com/methods/users.list

pub fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request
            .presence
            .map(|presence| ("presence", if presence { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("users.list");
    client
        .send(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(ListError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest {
    /// Whether to include presence data in the output
    pub presence: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    error: Option<String>,
    pub members: Option<Vec<::User>>,
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

/// Marks a user as active.
///
/// Wraps https://api.slack.com/methods/users.setActive

pub fn set_active<R>(client: &R, token: &str) -> Result<SetActiveResponse, SetActiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = ::get_slack_url_for_method("users.setActive");
    client
        .send(&url, &params[..])
        .map_err(SetActiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetActiveResponse>(&result)
                .map_err(SetActiveError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetActiveResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetActiveResponse, SetActiveError<E>>> for SetActiveResponse {
    fn into(self) -> Result<SetActiveResponse, SetActiveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum SetActiveError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for SetActiveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => SetActiveError::NotAuthed,
            "invalid_auth" => SetActiveError::InvalidAuth,
            "account_inactive" => SetActiveError::AccountInactive,
            "invalid_arg_name" => SetActiveError::InvalidArgName,
            "invalid_array_arg" => SetActiveError::InvalidArrayArg,
            "invalid_charset" => SetActiveError::InvalidCharset,
            "invalid_form_data" => SetActiveError::InvalidFormData,
            "invalid_post_type" => SetActiveError::InvalidPostType,
            "missing_post_type" => SetActiveError::MissingPostType,
            "team_added_to_org" => SetActiveError::TeamAddedToOrg,
            "request_timeout" => SetActiveError::RequestTimeout,
            _ => SetActiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetActiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for SetActiveError<E> {
    fn description(&self) -> &str {
        match *self {
                        SetActiveError::NotAuthed => "not_authed: No authentication token provided.",
SetActiveError::InvalidAuth => "invalid_auth: Invalid authentication token.",
SetActiveError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
SetActiveError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
SetActiveError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
SetActiveError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
SetActiveError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
SetActiveError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
SetActiveError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
SetActiveError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
SetActiveError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        SetActiveError::MalformedResponse(ref e) => e.description(),
                        SetActiveError::Unknown(ref s) => s,
                        SetActiveError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SetActiveError::MalformedResponse(ref e) => Some(e),
            SetActiveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Manually sets user presence.
///
/// Wraps https://api.slack.com/methods/users.setPresence

pub fn set_presence<R>(
    client: &R,
    token: &str,
    request: &SetPresenceRequest,
) -> Result<SetPresenceResponse, SetPresenceError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("presence", request.presence))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("users.setPresence");
    client
        .send(&url, &params[..])
        .map_err(SetPresenceError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetPresenceResponse>(&result)
                .map_err(SetPresenceError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SetPresenceRequest<'a> {
    /// Either auto or away
    pub presence: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPresenceResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetPresenceResponse, SetPresenceError<E>>> for SetPresenceResponse {
    fn into(self) -> Result<SetPresenceResponse, SetPresenceError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum SetPresenceError<E: Error> {
    /// Value passed for presence was invalid.
    InvalidPresence,
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

impl<'a, E: Error> From<&'a str> for SetPresenceError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "invalid_presence" => SetPresenceError::InvalidPresence,
            "not_authed" => SetPresenceError::NotAuthed,
            "invalid_auth" => SetPresenceError::InvalidAuth,
            "account_inactive" => SetPresenceError::AccountInactive,
            "invalid_arg_name" => SetPresenceError::InvalidArgName,
            "invalid_array_arg" => SetPresenceError::InvalidArrayArg,
            "invalid_charset" => SetPresenceError::InvalidCharset,
            "invalid_form_data" => SetPresenceError::InvalidFormData,
            "invalid_post_type" => SetPresenceError::InvalidPostType,
            "missing_post_type" => SetPresenceError::MissingPostType,
            "team_added_to_org" => SetPresenceError::TeamAddedToOrg,
            "request_timeout" => SetPresenceError::RequestTimeout,
            _ => SetPresenceError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetPresenceError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for SetPresenceError<E> {
    fn description(&self) -> &str {
        match *self {
                        SetPresenceError::InvalidPresence => "invalid_presence: Value passed for presence was invalid.",
SetPresenceError::NotAuthed => "not_authed: No authentication token provided.",
SetPresenceError::InvalidAuth => "invalid_auth: Invalid authentication token.",
SetPresenceError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
SetPresenceError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
SetPresenceError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
SetPresenceError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
SetPresenceError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
SetPresenceError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
SetPresenceError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
SetPresenceError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
SetPresenceError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        SetPresenceError::MalformedResponse(ref e) => e.description(),
                        SetPresenceError::Unknown(ref s) => s,
                        SetPresenceError::Client(ref inner) => inner.description()
                    }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SetPresenceError::MalformedResponse(ref e) => Some(e),
            SetPresenceError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
