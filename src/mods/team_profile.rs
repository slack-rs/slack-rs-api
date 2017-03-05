

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Retrieve a team's profile.
///
/// Wraps https://api.slack.com/methods/team.profile.get

pub fn get<R>(client: &R, token: &str, request: &GetRequest) -> Result<GetResponse, GetError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)), request.visibility.map(|visibility| ("visibility", visibility))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("team.profile.get", &params[..])
        .map_err(|err| GetError::Client(err))
        .and_then(|result| serde_json::from_str::<GetResponse>(&result).map_err(|_| GetError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct GetRequest<'a> {
    /// Filter by visibility.
    pub visibility: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub profile: Option<GetResponseProfile>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponseProfile {
    pub fields: Option<Vec<GetResponseProfileField>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponseProfileField {
    pub hint: Option<String>,
    pub id: Option<String>,
    pub is_hidden: Option<bool>,
    pub label: Option<String>,
    pub options: Option<HashMap<String, String>>,
    pub ordering: Option<i32>,
    pub possible_values: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}


impl<E: Error> Into<Result<GetResponse, GetError<E>>> for GetResponse {
    fn into(self) -> Result<GetResponse, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum GetError<E: Error> {
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
            &GetError::NotAuthed => "not_authed: No authentication token provided.",
            &GetError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &GetError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &GetError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &GetError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common \
                 decency. This includes very long names and names with non-alphanumeric characters other than _. If \
                 you get this error, it is typically an indication that you have made a very malformed API call."
            }
            &GetError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). \
                 These are never valid with the Slack API."
            }
            &GetError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the \
                 Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            &GetError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type \
                 application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or \
                 syntactically invalid."
            }
            &GetError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was \
                 invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data \
                 text/plain."
            }
            &GetError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the \
                 request did not include a Content-Type header."
            }
            &GetError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or \
                 truncated."
            }
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
