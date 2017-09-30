//! Get info on your team's User Groups.


#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Create a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.create

pub fn create<R>(
    client: &R,
    token: &str,
    request: &CreateRequest,
) -> Result<CreateResponse, CreateError<R::Error>>
where
    R: SlackWebRequestSender,
{

    let params = vec![
        Some(("token", token)),
        Some(("name", request.name)),
        request.handle.map(|handle| ("handle", handle)),
        request.description.map(|description| {
            ("description", description)
        }),
        request.channels.map(|channels| ("channels", channels)),
        request.include_count.map(|include_count| {
            ("include_count", if include_count { "1" } else { "0" })
        }),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("usergroups.create");
    client
        .send(&url, &params[..])
        .map_err(CreateError::Client)
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result).map_err(CreateError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct CreateRequest<'a> {
    /// A name for the User Group. Must be unique among User Groups.
    pub name: &'a str,
    /// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<&'a str>,
    /// A short description of the User Group.
    pub description: Option<&'a str>,
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<&'a str>,
    /// Include the number of users in each User Group.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroup: Option<::Usergroup>,
}


impl<E: Error> Into<Result<CreateResponse, CreateError<E>>> for CreateResponse {
    fn into(self) -> Result<CreateResponse, CreateError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum CreateError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for CreateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => CreateError::NotAuthed,
            "invalid_auth" => CreateError::InvalidAuth,
            "account_inactive" => CreateError::AccountInactive,
            "user_is_bot" => CreateError::UserIsBot,
            "user_is_restricted" => CreateError::UserIsRestricted,
            "invalid_arg_name" => CreateError::InvalidArgName,
            "invalid_array_arg" => CreateError::InvalidArrayArg,
            "invalid_charset" => CreateError::InvalidCharset,
            "invalid_form_data" => CreateError::InvalidFormData,
            "invalid_post_type" => CreateError::InvalidPostType,
            "missing_post_type" => CreateError::MissingPostType,
            "team_added_to_org" => CreateError::TeamAddedToOrg,
            "request_timeout" => CreateError::RequestTimeout,
            _ => CreateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CreateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for CreateError<E> {
    fn description(&self) -> &str {
        match *self {
            CreateError::NotAuthed => "not_authed: No authentication token provided.",
            CreateError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            CreateError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            CreateError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            CreateError::UserIsRestricted => {
                "user_is_restricted: This method cannot be called by a restricted user or single channel guest."
            }
            CreateError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            CreateError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            CreateError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            CreateError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            CreateError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            CreateError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            CreateError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            CreateError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            CreateError::MalformedResponse(ref e) => e.description(),
            CreateError::Unknown(ref s) => s,
            CreateError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CreateError::MalformedResponse(ref e) => Some(e),
            CreateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Disable an existing User Group
///
/// Wraps https://api.slack.com/methods/usergroups.disable

pub fn disable<R>(
    client: &R,
    token: &str,
    request: &DisableRequest,
) -> Result<DisableResponse, DisableError<R::Error>>
where
    R: SlackWebRequestSender,
{

    let params = vec![
        Some(("token", token)),
        Some(("usergroup", request.usergroup)),
        request.include_count.map(|include_count| {
            ("include_count", if include_count { "1" } else { "0" })
        }),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("usergroups.disable");
    client
        .send(&url, &params[..])
        .map_err(DisableError::Client)
        .and_then(|result| {
            serde_json::from_str::<DisableResponse>(&result).map_err(
                DisableError::MalformedResponse,
            )
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct DisableRequest<'a> {
    /// The encoded ID of the User Group to disable.
    pub usergroup: &'a str,
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisableResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroup: Option<::Usergroup>,
}


impl<E: Error> Into<Result<DisableResponse, DisableError<E>>> for DisableResponse {
    fn into(self) -> Result<DisableResponse, DisableError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum DisableError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for DisableError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => DisableError::NotAuthed,
            "invalid_auth" => DisableError::InvalidAuth,
            "account_inactive" => DisableError::AccountInactive,
            "user_is_bot" => DisableError::UserIsBot,
            "user_is_restricted" => DisableError::UserIsRestricted,
            "invalid_arg_name" => DisableError::InvalidArgName,
            "invalid_array_arg" => DisableError::InvalidArrayArg,
            "invalid_charset" => DisableError::InvalidCharset,
            "invalid_form_data" => DisableError::InvalidFormData,
            "invalid_post_type" => DisableError::InvalidPostType,
            "missing_post_type" => DisableError::MissingPostType,
            "team_added_to_org" => DisableError::TeamAddedToOrg,
            "request_timeout" => DisableError::RequestTimeout,
            _ => DisableError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DisableError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for DisableError<E> {
    fn description(&self) -> &str {
        match *self {
            DisableError::NotAuthed => "not_authed: No authentication token provided.",
            DisableError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            DisableError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            DisableError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            DisableError::UserIsRestricted => {
                "user_is_restricted: This method cannot be called by a restricted user or single channel guest."
            }
            DisableError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            DisableError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            DisableError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            DisableError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            DisableError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            DisableError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            DisableError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            DisableError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            DisableError::MalformedResponse(ref e) => e.description(),
            DisableError::Unknown(ref s) => s,
            DisableError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DisableError::MalformedResponse(ref e) => Some(e),
            DisableError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Enable a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.enable

pub fn enable<R>(
    client: &R,
    token: &str,
    request: &EnableRequest,
) -> Result<EnableResponse, EnableError<R::Error>>
where
    R: SlackWebRequestSender,
{

    let params = vec![
        Some(("token", token)),
        Some(("usergroup", request.usergroup)),
        request.include_count.map(|include_count| {
            ("include_count", if include_count { "1" } else { "0" })
        }),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("usergroups.enable");
    client
        .send(&url, &params[..])
        .map_err(EnableError::Client)
        .and_then(|result| {
            serde_json::from_str::<EnableResponse>(&result).map_err(EnableError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct EnableRequest<'a> {
    /// The encoded ID of the User Group to enable.
    pub usergroup: &'a str,
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EnableResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroup: Option<::Usergroup>,
}


impl<E: Error> Into<Result<EnableResponse, EnableError<E>>> for EnableResponse {
    fn into(self) -> Result<EnableResponse, EnableError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum EnableError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for EnableError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => EnableError::NotAuthed,
            "invalid_auth" => EnableError::InvalidAuth,
            "account_inactive" => EnableError::AccountInactive,
            "user_is_bot" => EnableError::UserIsBot,
            "user_is_restricted" => EnableError::UserIsRestricted,
            "invalid_arg_name" => EnableError::InvalidArgName,
            "invalid_array_arg" => EnableError::InvalidArrayArg,
            "invalid_charset" => EnableError::InvalidCharset,
            "invalid_form_data" => EnableError::InvalidFormData,
            "invalid_post_type" => EnableError::InvalidPostType,
            "missing_post_type" => EnableError::MissingPostType,
            "team_added_to_org" => EnableError::TeamAddedToOrg,
            "request_timeout" => EnableError::RequestTimeout,
            _ => EnableError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EnableError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for EnableError<E> {
    fn description(&self) -> &str {
        match *self {
            EnableError::NotAuthed => "not_authed: No authentication token provided.",
            EnableError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            EnableError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            EnableError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            EnableError::UserIsRestricted => {
                "user_is_restricted: This method cannot be called by a restricted user or single channel guest."
            }
            EnableError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            EnableError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            EnableError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            EnableError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            EnableError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            EnableError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            EnableError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            EnableError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            EnableError::MalformedResponse(ref e) => e.description(),
            EnableError::Unknown(ref s) => s,
            EnableError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            EnableError::MalformedResponse(ref e) => Some(e),
            EnableError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// List all User Groups for a team
///
/// Wraps https://api.slack.com/methods/usergroups.list

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
        request.include_disabled.map(|include_disabled| {
            ("include_disabled", if include_disabled { "1" } else { "0" })
        }),
        request.include_count.map(|include_count| {
            ("include_count", if include_count { "1" } else { "0" })
        }),
        request.include_users.map(|include_users| {
            ("include_users", if include_users { "1" } else { "0" })
        }),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("usergroups.list");
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
    /// Include disabled User Groups.
    pub include_disabled: Option<bool>,
    /// Include the number of users in each User Group.
    pub include_count: Option<bool>,
    /// Include the list of users for each User Group.
    pub include_users: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroups: Option<Vec<::Usergroup>>,
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
            "user_is_bot" => ListError::UserIsBot,
            "user_is_restricted" => ListError::UserIsRestricted,
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
            ListError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            ListError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            ListError::UserIsRestricted => {
                "user_is_restricted: This method cannot be called by a restricted user or single channel guest."
            }
            ListError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            ListError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            ListError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            ListError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            ListError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            ListError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            ListError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            ListError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            ListError::MalformedResponse(ref e) => e.description(),
            ListError::Unknown(ref s) => s,
            ListError::Client(ref inner) => inner.description(),
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

/// Update an existing User Group
///
/// Wraps https://api.slack.com/methods/usergroups.update

pub fn update<R>(
    client: &R,
    token: &str,
    request: &UpdateRequest,
) -> Result<UpdateResponse, UpdateError<R::Error>>
where
    R: SlackWebRequestSender,
{

    let params = vec![
        Some(("token", token)),
        Some(("usergroup", request.usergroup)),
        request.name.map(|name| ("name", name)),
        request.handle.map(|handle| ("handle", handle)),
        request.description.map(|description| {
            ("description", description)
        }),
        request.channels.map(|channels| ("channels", channels)),
        request.include_count.map(|include_count| {
            ("include_count", if include_count { "1" } else { "0" })
        }),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("usergroups.update");
    client
        .send(&url, &params[..])
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result).map_err(UpdateError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct UpdateRequest<'a> {
    /// The encoded ID of the User Group to update.
    pub usergroup: &'a str,
    /// A name for the User Group. Must be unique among User Groups.
    pub name: Option<&'a str>,
    /// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<&'a str>,
    /// A short description of the User Group.
    pub description: Option<&'a str>,
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<&'a str>,
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroup: Option<::Usergroup>,
}


impl<E: Error> Into<Result<UpdateResponse, UpdateError<E>>> for UpdateResponse {
    fn into(self) -> Result<UpdateResponse, UpdateError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum UpdateError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for UpdateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => UpdateError::NotAuthed,
            "invalid_auth" => UpdateError::InvalidAuth,
            "account_inactive" => UpdateError::AccountInactive,
            "user_is_bot" => UpdateError::UserIsBot,
            "user_is_restricted" => UpdateError::UserIsRestricted,
            "invalid_arg_name" => UpdateError::InvalidArgName,
            "invalid_array_arg" => UpdateError::InvalidArrayArg,
            "invalid_charset" => UpdateError::InvalidCharset,
            "invalid_form_data" => UpdateError::InvalidFormData,
            "invalid_post_type" => UpdateError::InvalidPostType,
            "missing_post_type" => UpdateError::MissingPostType,
            "team_added_to_org" => UpdateError::TeamAddedToOrg,
            "request_timeout" => UpdateError::RequestTimeout,
            _ => UpdateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UpdateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for UpdateError<E> {
    fn description(&self) -> &str {
        match *self {
            UpdateError::NotAuthed => "not_authed: No authentication token provided.",
            UpdateError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            UpdateError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            UpdateError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            UpdateError::UserIsRestricted => {
                "user_is_restricted: This method cannot be called by a restricted user or single channel guest."
            }
            UpdateError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            UpdateError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            UpdateError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            UpdateError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            UpdateError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            UpdateError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            UpdateError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            UpdateError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            UpdateError::MalformedResponse(ref e) => e.description(),
            UpdateError::Unknown(ref s) => s,
            UpdateError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            UpdateError::MalformedResponse(ref e) => Some(e),
            UpdateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
