

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Exchanges a temporary OAuth code for an API token.
///
/// Wraps https://api.slack.com/methods/oauth.access

pub fn access<R>(client: &R, request: &AccessRequest) -> Result<AccessResponse, AccessError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("client_id", request.client_id)),
                      Some(("client_secret", request.client_secret)),
                      Some(("code", request.code)),
                      request.redirect_uri.map(|redirect_uri| ("redirect_uri", redirect_uri))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("oauth.access", &params[..])
        .map_err(|err| AccessError::Client(err))
        .and_then(|result| serde_json::from_str::<AccessResponse>(&result).map_err(|_| AccessError::MalformedResponse))
}

#[derive(Clone, Default, Debug)]
pub struct AccessRequest<'a> {
    /// Issued when you created your application.
    pub client_id: &'a str,
    /// Issued when you created your application.
    pub client_secret: &'a str,
    /// The code param returned via the OAuth callback.
    pub code: &'a str,
    /// This must match the originally submitted URI (if one was sent).
    pub redirect_uri: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccessResponse {
    pub access_token: Option<String>,
    pub scope: Option<String>,
}



#[derive(Clone, Debug)]
pub enum AccessError<E: Error> {
    /// Value passed for client_id was invalid.
    InvalidClientId,
    /// Value passed for client_secret was invalid.
    BadClientSecret,
    /// Value passed for code was invalid.
    InvalidCode,
    /// Value passed for redirect_uri did not match the redirect_uri in the original request.
    BadRedirectUri,
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

impl<'a, E: Error> From<&'a str> for AccessError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "invalid_client_id" => AccessError::InvalidClientId,
            "bad_client_secret" => AccessError::BadClientSecret,
            "invalid_code" => AccessError::InvalidCode,
            "bad_redirect_uri" => AccessError::BadRedirectUri,
            "invalid_arg_name" => AccessError::InvalidArgName,
            "invalid_array_arg" => AccessError::InvalidArrayArg,
            "invalid_charset" => AccessError::InvalidCharset,
            "invalid_form_data" => AccessError::InvalidFormData,
            "invalid_post_type" => AccessError::InvalidPostType,
            "missing_post_type" => AccessError::MissingPostType,
            "request_timeout" => AccessError::RequestTimeout,
            _ => AccessError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AccessError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for AccessError<E> {
    fn description(&self) -> &str {
        match self {
            &AccessError::InvalidClientId => "invalid_client_id: Value passed for client_id was invalid.",
            &AccessError::BadClientSecret => "bad_client_secret: Value passed for client_secret was invalid.",
            &AccessError::InvalidCode => "invalid_code: Value passed for code was invalid.",
            &AccessError::BadRedirectUri => {
                "bad_redirect_uri: Value passed for redirect_uri did not match the redirect_uri in the original \
                 request."
            }
            &AccessError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common \
                 decency. This includes very long names and names with non-alphanumeric characters other than _. If \
                 you get this error, it is typically an indication that you have made a very malformed API call."
            }
            &AccessError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). \
                 These are never valid with the Slack API."
            }
            &AccessError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the \
                 Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            &AccessError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type \
                 application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or \
                 syntactically invalid."
            }
            &AccessError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was \
                 invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data \
                 text/plain."
            }
            &AccessError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the \
                 request did not include a Content-Type header."
            }
            &AccessError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or \
                 truncated."
            }
            &AccessError::MalformedResponse => "Malformed response data from Slack.",
            &AccessError::Unknown(ref s) => s,
            &AccessError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &AccessError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
