
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

pub fn delete_photo<R>(client: &R,
                       request: &DeletePhotoRequest)
                       -> Result<DeletePhotoResponse, DeletePhotoError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.deletePhoto", &params[..])
        .map_err(|err| DeletePhotoError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<DeletePhotoResponse>(&result).map_err(|_| DeletePhotoError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct DeletePhotoRequest<'a> {
    /// Authentication token.
    /// Requires scope: users.profile:write
    pub token: &'a str,
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
#[derive(Clone, Debug)]
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
        match self {
            &DeletePhotoError::NotAuthed => "not_authed",
            &DeletePhotoError::InvalidAuth => "invalid_auth",
            &DeletePhotoError::AccountInactive => "account_inactive",
            &DeletePhotoError::UserIsBot => "user_is_bot",
            &DeletePhotoError::InvalidArgName => "invalid_arg_name",
            &DeletePhotoError::InvalidArrayArg => "invalid_array_arg",
            &DeletePhotoError::InvalidCharset => "invalid_charset",
            &DeletePhotoError::InvalidFormData => "invalid_form_data",
            &DeletePhotoError::InvalidPostType => "invalid_post_type",
            &DeletePhotoError::MissingPostType => "missing_post_type",
            &DeletePhotoError::RequestTimeout => "request_timeout",
            &DeletePhotoError::MalformedResponse => "Malformed response data from Slack.",
            &DeletePhotoError::Unknown(ref s) => s,
            &DeletePhotoError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &DeletePhotoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets user presence information.
///
/// Wraps https://api.slack.com/methods/users.getPresence

pub fn get_presence<R>(client: &R,
                       request: &GetPresenceRequest)
                       -> Result<GetPresenceResponse, GetPresenceError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("user", request.user))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.getPresence", &params[..])
        .map_err(|err| GetPresenceError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<GetPresenceResponse>(&result).map_err(|_| GetPresenceError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct GetPresenceRequest<'a> {
    /// Authentication token.
    /// Requires scope: users:read
    pub token: &'a str,
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
#[derive(Clone, Debug)]
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
        match self {
            &GetPresenceError::NotAuthed => "not_authed",
            &GetPresenceError::InvalidAuth => "invalid_auth",
            &GetPresenceError::AccountInactive => "account_inactive",
            &GetPresenceError::InvalidArgName => "invalid_arg_name",
            &GetPresenceError::InvalidArrayArg => "invalid_array_arg",
            &GetPresenceError::InvalidCharset => "invalid_charset",
            &GetPresenceError::InvalidFormData => "invalid_form_data",
            &GetPresenceError::InvalidPostType => "invalid_post_type",
            &GetPresenceError::MissingPostType => "missing_post_type",
            &GetPresenceError::RequestTimeout => "request_timeout",
            &GetPresenceError::MalformedResponse => "Malformed response data from Slack.",
            &GetPresenceError::Unknown(ref s) => s,
            &GetPresenceError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &GetPresenceError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Get a user's identity.
///
/// Wraps https://api.slack.com/methods/users.identity

pub fn identity<R>(client: &R, request: &IdentityRequest) -> Result<IdentityResponse, IdentityError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.identity", &params[..])
        .map_err(|err| IdentityError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<IdentityResponse>(&result).map_err(|_| IdentityError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct IdentityRequest<'a> {
    /// Authentication token.
    /// Requires scope: identity.basic
    pub token: &'a str,
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
#[derive(Clone, Debug)]
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
        match self {
            &IdentityError::NotAuthed => "not_authed",
            &IdentityError::InvalidAuth => "invalid_auth",
            &IdentityError::AccountInactive => "account_inactive",
            &IdentityError::UserIsBot => "user_is_bot",
            &IdentityError::InvalidArgName => "invalid_arg_name",
            &IdentityError::InvalidArrayArg => "invalid_array_arg",
            &IdentityError::InvalidCharset => "invalid_charset",
            &IdentityError::InvalidFormData => "invalid_form_data",
            &IdentityError::InvalidPostType => "invalid_post_type",
            &IdentityError::MissingPostType => "missing_post_type",
            &IdentityError::RequestTimeout => "request_timeout",
            &IdentityError::MalformedResponse => "Malformed response data from Slack.",
            &IdentityError::Unknown(ref s) => s,
            &IdentityError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &IdentityError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets information about a user.
///
/// Wraps https://api.slack.com/methods/users.info

pub fn info<R>(client: &R, request: &InfoRequest) -> Result<InfoResponse, InfoError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("user", request.user))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.info", &params[..])
        .map_err(|err| InfoError::Client(err))
        .and_then(|result| serde_json::from_str::<InfoResponse>(&result).map_err(|_| InfoError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Authentication token.
    /// Requires scope: users:read
    pub token: &'a str,
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
#[derive(Clone, Debug)]
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
            &InfoError::UserNotFound => "user_not_found",
            &InfoError::UserNotVisible => "user_not_visible",
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

/// Lists all users in a Slack team.
///
/// Wraps https://api.slack.com/methods/users.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      request.presence
                          .map(|presence| ("presence", if presence { "1" } else { "0" }))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.list", &params[..])
        .map_err(|err| ListError::Client(err))
        .and_then(|result| serde_json::from_str::<ListResponse>(&result).map_err(|_| ListError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Authentication token.
    /// Requires scope: users:read
    pub token: &'a str,
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
#[derive(Clone, Debug)]
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

/// Marks a user as active.
///
/// Wraps https://api.slack.com/methods/users.setActive

pub fn set_active<R>(client: &R, request: &SetActiveRequest) -> Result<SetActiveResponse, SetActiveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.setActive", &params[..])
        .map_err(|err| SetActiveError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<SetActiveResponse>(&result).map_err(|_| SetActiveError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SetActiveRequest<'a> {
    /// Authentication token.
    /// Requires scope: users:write
    pub token: &'a str,
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
#[derive(Clone, Debug)]
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
        match self {
            &SetActiveError::NotAuthed => "not_authed",
            &SetActiveError::InvalidAuth => "invalid_auth",
            &SetActiveError::AccountInactive => "account_inactive",
            &SetActiveError::InvalidArgName => "invalid_arg_name",
            &SetActiveError::InvalidArrayArg => "invalid_array_arg",
            &SetActiveError::InvalidCharset => "invalid_charset",
            &SetActiveError::InvalidFormData => "invalid_form_data",
            &SetActiveError::InvalidPostType => "invalid_post_type",
            &SetActiveError::MissingPostType => "missing_post_type",
            &SetActiveError::RequestTimeout => "request_timeout",
            &SetActiveError::MalformedResponse => "Malformed response data from Slack.",
            &SetActiveError::Unknown(ref s) => s,
            &SetActiveError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SetActiveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Set the user profile photo
///
/// Wraps https://api.slack.com/methods/users.setPhoto

pub fn set_photo<R>(client: &R, request: &SetPhotoRequest) -> Result<SetPhotoResponse, SetPhotoError<R::Error>>
    where R: SlackWebRequestSender
{
    let crop_x = request.crop_x.map(|crop_x| crop_x.to_string());
    let crop_y = request.crop_y.map(|crop_y| crop_y.to_string());
    let crop_w = request.crop_w.map(|crop_w| crop_w.to_string());
    let params = vec![Some(("token", request.token)),
                      Some(("image", request.image)),
                      crop_x.as_ref().map(|crop_x| ("crop_x", &crop_x[..])),
                      crop_y.as_ref().map(|crop_y| ("crop_y", &crop_y[..])),
                      crop_w.as_ref().map(|crop_w| ("crop_w", &crop_w[..]))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.setPhoto", &params[..])
        .map_err(|err| SetPhotoError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<SetPhotoResponse>(&result).map_err(|_| SetPhotoError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SetPhotoRequest<'a> {
    /// Authentication token.
    /// Requires scope: users.profile:write
    pub token: &'a str,
    /// File contents via multipart/form-data.
    pub image: &'a str,
    /// X coordinate of top-left corner of crop box
    pub crop_x: Option<u32>,
    /// Y coordinate of top-left corner of crop box
    pub crop_y: Option<u32>,
    /// Width/height of crop box (always square)
    pub crop_w: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPhotoResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<SetPhotoResponse, SetPhotoError<E>>> for SetPhotoResponse {
    fn into(self) -> Result<SetPhotoResponse, SetPhotoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum SetPhotoError<E: Error> {
    /// The uploaded image could not be processed - try passing a JPEG, GIF or PNG
    BadImage,
    /// The uploaded image had excessive dimensions
    TooLarge,
    /// An animated GIF with too many frames was uploaded
    TooManyFrames,
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

impl<'a, E: Error> From<&'a str> for SetPhotoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "bad_image" => SetPhotoError::BadImage,
            "too_large" => SetPhotoError::TooLarge,
            "too_many_frames" => SetPhotoError::TooManyFrames,
            "not_authed" => SetPhotoError::NotAuthed,
            "invalid_auth" => SetPhotoError::InvalidAuth,
            "account_inactive" => SetPhotoError::AccountInactive,
            "user_is_bot" => SetPhotoError::UserIsBot,
            "invalid_arg_name" => SetPhotoError::InvalidArgName,
            "invalid_array_arg" => SetPhotoError::InvalidArrayArg,
            "invalid_charset" => SetPhotoError::InvalidCharset,
            "invalid_form_data" => SetPhotoError::InvalidFormData,
            "invalid_post_type" => SetPhotoError::InvalidPostType,
            "missing_post_type" => SetPhotoError::MissingPostType,
            "request_timeout" => SetPhotoError::RequestTimeout,
            _ => SetPhotoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetPhotoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for SetPhotoError<E> {
    fn description(&self) -> &str {
        match self {
            &SetPhotoError::BadImage => "bad_image",
            &SetPhotoError::TooLarge => "too_large",
            &SetPhotoError::TooManyFrames => "too_many_frames",
            &SetPhotoError::NotAuthed => "not_authed",
            &SetPhotoError::InvalidAuth => "invalid_auth",
            &SetPhotoError::AccountInactive => "account_inactive",
            &SetPhotoError::UserIsBot => "user_is_bot",
            &SetPhotoError::InvalidArgName => "invalid_arg_name",
            &SetPhotoError::InvalidArrayArg => "invalid_array_arg",
            &SetPhotoError::InvalidCharset => "invalid_charset",
            &SetPhotoError::InvalidFormData => "invalid_form_data",
            &SetPhotoError::InvalidPostType => "invalid_post_type",
            &SetPhotoError::MissingPostType => "missing_post_type",
            &SetPhotoError::RequestTimeout => "request_timeout",
            &SetPhotoError::MalformedResponse => "Malformed response data from Slack.",
            &SetPhotoError::Unknown(ref s) => s,
            &SetPhotoError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SetPhotoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Manually sets user presence.
///
/// Wraps https://api.slack.com/methods/users.setPresence

pub fn set_presence<R>(client: &R,
                       request: &SetPresenceRequest)
                       -> Result<SetPresenceResponse, SetPresenceError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("presence", request.presence))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.setPresence", &params[..])
        .map_err(|err| SetPresenceError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<SetPresenceResponse>(&result).map_err(|_| SetPresenceError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SetPresenceRequest<'a> {
    /// Authentication token.
    /// Requires scope: users:write
    pub token: &'a str,
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
#[derive(Clone, Debug)]
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
        match self {
            &SetPresenceError::InvalidPresence => "invalid_presence",
            &SetPresenceError::NotAuthed => "not_authed",
            &SetPresenceError::InvalidAuth => "invalid_auth",
            &SetPresenceError::AccountInactive => "account_inactive",
            &SetPresenceError::InvalidArgName => "invalid_arg_name",
            &SetPresenceError::InvalidArrayArg => "invalid_array_arg",
            &SetPresenceError::InvalidCharset => "invalid_charset",
            &SetPresenceError::InvalidFormData => "invalid_form_data",
            &SetPresenceError::InvalidPostType => "invalid_post_type",
            &SetPresenceError::MissingPostType => "missing_post_type",
            &SetPresenceError::RequestTimeout => "request_timeout",
            &SetPresenceError::MalformedResponse => "Malformed response data from Slack.",
            &SetPresenceError::Unknown(ref s) => s,
            &SetPresenceError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SetPresenceError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
