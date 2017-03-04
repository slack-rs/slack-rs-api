
#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

#[allow(unused_imports)]
use ToResult;
use requests::SlackWebRequestSender;

/// Retrieves a user's profile information.
///
/// Wraps https://api.slack.com/methods/users.profile.get

pub fn get<R>(client: &R, request: &GetRequest) -> Result<GetResponse, GetError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      request.user.map(|user| ("user", user)),
                      request.include_labels.map(|include_labels| {
                          ("include_labels", if include_labels { "1" } else { "0" })
                      })];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.profile.get", &params[..])
        .map_err(|err| GetError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result).map_err(|_| GetError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct GetRequest<'a> {
    /// Authentication token.
    /// Requires scope: users.profile:read
    pub token: &'a str,
    /// User to retrieve profile info for
    pub user: Option<&'a str>,
    /// Include labels for each ID in custom profile fields
    pub include_labels: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub profile: Option<::UserProfile>,
}


impl<E: Error> ToResult<GetResponse, GetError<E>> for GetResponse {
    fn to_result(self) -> Result<GetResponse, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum GetError<E: Error> {
    /// Value passed for user was invalid.
    UserNotFound,
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

impl<'a, E: Error> From<&'a str> for GetError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "user_not_found" => GetError::UserNotFound,
            "not_authed" => GetError::NotAuthed,
            "invalid_auth" => GetError::InvalidAuth,
            "account_inactive" => GetError::AccountInactive,
            "user_is_bot" => GetError::UserIsBot,
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
            &GetError::UserNotFound => "user_not_found",
            &GetError::NotAuthed => "not_authed",
            &GetError::InvalidAuth => "invalid_auth",
            &GetError::AccountInactive => "account_inactive",
            &GetError::UserIsBot => "user_is_bot",
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

/// Set the profile information for a user.
///
/// Wraps https://api.slack.com/methods/users.profile.set

pub fn set<R>(client: &R, request: &SetRequest) -> Result<SetResponse, SetError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      request.user.map(|user| ("user", user)),
                      request.profile.map(|profile| ("profile", profile)),
                      request.name.map(|name| ("name", name)),
                      request.value.map(|value| ("value", value))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("users.profile.set", &params[..])
        .map_err(|err| SetError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<SetResponse>(&result).map_err(|_| SetError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct SetRequest<'a> {
    /// Authentication token.
    /// Requires scope: users.profile:write
    pub token: &'a str,
    /// ID of user to change. This argument may only be specified by team admins on paid teams.
    pub user: Option<&'a str>,
    /// Collection of key:value pairs presented as a URL-encoded JSON hash.
    pub profile: Option<&'a str>,
    /// Name of a single key to set. Usable only if profile is not passed.
    pub name: Option<&'a str>,
    /// Value to set a single key to. Usable only if profile is not passed.
    pub value: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub profile: Option<::UserProfile>,
}


impl<E: Error> ToResult<SetResponse, SetError<E>> for SetResponse {
    fn to_result(self) -> Result<SetResponse, SetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum SetError<E: Error> {
    /// First or last name are reserved.
    ReservedName,
    /// Profile object passed in is not valid JSON (make sure it is URL encoded!).
    InvalidProfile,
    /// Failed to set user profile.
    ProfileSetFailed,
    /// Only admins can update the profile of another user.
    NotAdmin,
    /// Only team owners and selected members can update the profile of a bot user.
    NotAppAdmin,
    /// Only a primary owner can update the profile of an admin.
    CannotUpdateAdminUser,
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

impl<'a, E: Error> From<&'a str> for SetError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "reserved_name" => SetError::ReservedName,
            "invalid_profile" => SetError::InvalidProfile,
            "profile_set_failed" => SetError::ProfileSetFailed,
            "not_admin" => SetError::NotAdmin,
            "not_app_admin" => SetError::NotAppAdmin,
            "cannot_update_admin_user" => SetError::CannotUpdateAdminUser,
            "not_authed" => SetError::NotAuthed,
            "invalid_auth" => SetError::InvalidAuth,
            "account_inactive" => SetError::AccountInactive,
            "user_is_bot" => SetError::UserIsBot,
            "invalid_arg_name" => SetError::InvalidArgName,
            "invalid_array_arg" => SetError::InvalidArrayArg,
            "invalid_charset" => SetError::InvalidCharset,
            "invalid_form_data" => SetError::InvalidFormData,
            "invalid_post_type" => SetError::InvalidPostType,
            "missing_post_type" => SetError::MissingPostType,
            "request_timeout" => SetError::RequestTimeout,
            _ => SetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for SetError<E> {
    fn description(&self) -> &str {
        match self {
            &SetError::ReservedName => "reserved_name",
            &SetError::InvalidProfile => "invalid_profile",
            &SetError::ProfileSetFailed => "profile_set_failed",
            &SetError::NotAdmin => "not_admin",
            &SetError::NotAppAdmin => "not_app_admin",
            &SetError::CannotUpdateAdminUser => "cannot_update_admin_user",
            &SetError::NotAuthed => "not_authed",
            &SetError::InvalidAuth => "invalid_auth",
            &SetError::AccountInactive => "account_inactive",
            &SetError::UserIsBot => "user_is_bot",
            &SetError::InvalidArgName => "invalid_arg_name",
            &SetError::InvalidArrayArg => "invalid_array_arg",
            &SetError::InvalidCharset => "invalid_charset",
            &SetError::InvalidFormData => "invalid_form_data",
            &SetError::InvalidPostType => "invalid_post_type",
            &SetError::MissingPostType => "missing_post_type",
            &SetError::RequestTimeout => "request_timeout",
            &SetError::MalformedResponse => "Malformed response data from Slack.",
            &SetError::Unknown(ref s) => s,
            &SetError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SetError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
