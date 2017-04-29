

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Starts a Real Time Messaging session.
///
/// Wraps https://api.slack.com/methods/rtm.start

pub fn start<R>(client: &R,
                token: &str,
                request: &StartRequest)
                -> Result<StartResponse, StartError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      request
                          .no_unreads
                          .map(|no_unreads| ("no_unreads", no_unreads)),
                      request
                          .mpim_aware
                          .map(|mpim_aware| ("mpim_aware", mpim_aware))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("rtm.start");
    client
        .send(&url, &params[..])
        .map_err(|err| StartError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<StartResponse>(&result)
                            .map_err(|e| StartError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct StartRequest<'a> {
    /// Skip unread counts for each channel (improves performance).
    pub no_unreads: Option<&'a str>,
    /// Returns MPIMs to the client in the API response.
    pub mpim_aware: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StartResponse {
    pub bots: Option<Vec<::Bot>>,
    pub channels: Option<Vec<::Channel>>,
    error: Option<String>,
    pub groups: Option<Vec<::Group>>,
    pub ims: Option<Vec<::Im>>,
    pub mpims: Option<Vec<::Mpim>>,
    #[serde(default)]
    ok: bool,
    #[serde(rename = "self")]
    pub slf: Option<::User>,
    pub team: Option<::Team>,
    pub url: Option<String>,
    pub users: Option<Vec<::User>>,
}


impl<E: Error> Into<Result<StartResponse, StartError<E>>> for StartResponse {
    fn into(self) -> Result<StartResponse, StartError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(serde_json::error::Error),
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
            "request_timeout" => StartError::RequestTimeout,
            _ => StartError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for StartError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for StartError<E> {
    fn description(&self) -> &str {
        match self {
            &StartError::MigrationInProgress => "migration_in_progress: Team is being migrated between servers. See the team_migration_started event documentation for details.",
            &StartError::NotAuthed => "not_authed: No authentication token provided.",
            &StartError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &StartError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &StartError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &StartError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &StartError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &StartError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &StartError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &StartError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &StartError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &StartError::MalformedResponse(ref e) => e.description(),
            &StartError::Unknown(ref s) => s,
            &StartError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &StartError::MalformedResponse(ref e) => Some(e),
            &StartError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
