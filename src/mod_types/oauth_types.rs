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

#[derive(Debug)]
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
            "team_added_to_org" => AccessError::TeamAddedToOrg,
            "request_timeout" => AccessError::RequestTimeout,
            _ => AccessError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AccessError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        AccessError::InvalidClientId => "invalid_client_id: Value passed for client_id was invalid.",
AccessError::BadClientSecret => "bad_client_secret: Value passed for client_secret was invalid.",
AccessError::InvalidCode => "invalid_code: Value passed for code was invalid.",
AccessError::BadRedirectUri => "bad_redirect_uri: Value passed for redirect_uri did not match the redirect_uri in the original request.",
AccessError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
AccessError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
AccessError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
AccessError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
AccessError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
AccessError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
AccessError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
AccessError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        AccessError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        AccessError::Unknown(ref s) => return write!(f, "{}", s),
                        AccessError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for AccessError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AccessError::MalformedResponse(_, ref e) => Some(e),
            AccessError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
