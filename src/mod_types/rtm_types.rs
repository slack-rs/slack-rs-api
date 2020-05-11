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

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub struct ConnectResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    #[serde(rename = "self")]
    pub slf: Option<ConnectResponseSelf>,
    pub team: Option<ConnectResponseTeam>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConnectResponseSelf {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConnectResponseTeam {
    pub domain: Option<String>,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

impl<E: Error> Into<Result<ConnectResponse, ConnectError<E>>> for ConnectResponse {
    fn into(self) -> Result<ConnectResponse, ConnectError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum ConnectError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for ConnectError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => ConnectError::NotAuthed,
            "invalid_auth" => ConnectError::InvalidAuth,
            "account_inactive" => ConnectError::AccountInactive,
            "invalid_arg_name" => ConnectError::InvalidArgName,
            "invalid_array_arg" => ConnectError::InvalidArrayArg,
            "invalid_charset" => ConnectError::InvalidCharset,
            "invalid_form_data" => ConnectError::InvalidFormData,
            "invalid_post_type" => ConnectError::InvalidPostType,
            "missing_post_type" => ConnectError::MissingPostType,
            "team_added_to_org" => ConnectError::TeamAddedToOrg,
            "request_timeout" => ConnectError::RequestTimeout,
            _ => ConnectError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ConnectError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        ConnectError::NotAuthed => "not_authed: No authentication token provided.",
ConnectError::InvalidAuth => "invalid_auth: Invalid authentication token.",
ConnectError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
ConnectError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
ConnectError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
ConnectError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
ConnectError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
ConnectError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
ConnectError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
ConnectError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
ConnectError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        ConnectError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        ConnectError::Unknown(ref s) => return write!(f, "{}", s),
                        ConnectError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for ConnectError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ConnectError::MalformedResponse(_, ref e) => Some(e),
            ConnectError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct StartRequest {
    /// Skip unread counts for each channel (improves performance).
    pub no_unreads: Option<bool>,
    /// Returns MPIMs to the client in the API response.
    pub mpim_aware: Option<bool>,
    /// Exclude latest timestamps for channels, groups, mpims, and ims. Automatically sets no_unreads to 1
    pub no_latest: Option<bool>,
    /// Only deliver presence events when requested by subscription. See [presence subscriptions](/docs/presence-and-status#subscriptions).
    pub batch_presence_aware: Option<bool>,
    /// Set this to `true` to receive the locale for users and channels. Defaults to `false`
    pub include_locale: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StartResponse {
    pub bots: Option<Vec<crate::Bot>>,
    pub channels: Option<Vec<crate::Channel>>,
    error: Option<String>,
    pub groups: Option<Vec<crate::Group>>,
    pub ims: Option<Vec<crate::Im>>,
    pub mpims: Option<Vec<crate::Mpim>>,
    #[serde(default)]
    ok: bool,
    #[serde(rename = "self")]
    pub slf: Option<crate::User>,
    pub team: Option<crate::Team>,
    pub url: Option<String>,
    pub users: Option<Vec<crate::User>>,
}

impl<E: Error> Into<Result<StartResponse, StartError<E>>> for StartResponse {
    fn into(self) -> Result<StartResponse, StartError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum StartError<E: Error> {
    /// Team is being migrated between servers. See the team_migration_started event documentation for details.
    MigrationInProgress,
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

impl<'a, E: Error> From<&'a str> for StartError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "migration_in_progress" => StartError::MigrationInProgress,
            "not_authed" => StartError::NotAuthed,
            "invalid_auth" => StartError::InvalidAuth,
            "account_inactive" => StartError::AccountInactive,
            "invalid_arg_name" => StartError::InvalidArgName,
            "invalid_array_arg" => StartError::InvalidArrayArg,
            "invalid_charset" => StartError::InvalidCharset,
            "invalid_form_data" => StartError::InvalidFormData,
            "invalid_post_type" => StartError::InvalidPostType,
            "missing_post_type" => StartError::MissingPostType,
            "team_added_to_org" => StartError::TeamAddedToOrg,
            "request_timeout" => StartError::RequestTimeout,
            _ => StartError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for StartError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        StartError::MigrationInProgress => "migration_in_progress: Team is being migrated between servers. See the team_migration_started event documentation for details.",
StartError::NotAuthed => "not_authed: No authentication token provided.",
StartError::InvalidAuth => "invalid_auth: Invalid authentication token.",
StartError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
StartError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
StartError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
StartError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
StartError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
StartError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
StartError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
StartError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
StartError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        StartError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        StartError::Unknown(ref s) => return write!(f, "{}", s),
                        StartError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for StartError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            StartError::MalformedResponse(_, ref e) => Some(e),
            StartError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
