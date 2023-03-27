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
pub struct RevokeRequest {
    /// Setting this parameter to 1 triggers a testing mode where the specified token will not actually be revoked.
    pub test: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevokeResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub revoked: Option<bool>,
}

impl<E: Error> From<RevokeResponse> for Result<RevokeResponse, RevokeError<E>> {
    fn from(val: RevokeResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum RevokeError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for RevokeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => RevokeError::NotAuthed,
            "invalid_auth" => RevokeError::InvalidAuth,
            "account_inactive" => RevokeError::AccountInactive,
            "invalid_arg_name" => RevokeError::InvalidArgName,
            "invalid_array_arg" => RevokeError::InvalidArrayArg,
            "invalid_charset" => RevokeError::InvalidCharset,
            "invalid_form_data" => RevokeError::InvalidFormData,
            "invalid_post_type" => RevokeError::InvalidPostType,
            "missing_post_type" => RevokeError::MissingPostType,
            "team_added_to_org" => RevokeError::TeamAddedToOrg,
            "request_timeout" => RevokeError::RequestTimeout,
            _ => RevokeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RevokeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        RevokeError::NotAuthed => "not_authed: No authentication token provided.",
RevokeError::InvalidAuth => "invalid_auth: Invalid authentication token.",
RevokeError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
RevokeError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
RevokeError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
RevokeError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
RevokeError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
RevokeError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
RevokeError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
RevokeError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
RevokeError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        RevokeError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        RevokeError::Unknown(ref s) => return write!(f, "{}", s),
                        RevokeError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for RevokeError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RevokeError::MalformedResponse(_, ref e) => Some(e),
            RevokeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TestResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub team: Option<String>,
    pub team_id: Option<String>,
    pub url: Option<String>,
    pub user: Option<String>,
    pub user_id: Option<String>,
}

impl<E: Error> From<TestResponse> for Result<TestResponse, TestError<E>> {
    fn from(val: TestResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum TestError<E: Error> {
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

impl<'a, E: Error> From<&'a str> for TestError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => TestError::NotAuthed,
            "invalid_auth" => TestError::InvalidAuth,
            "account_inactive" => TestError::AccountInactive,
            "invalid_arg_name" => TestError::InvalidArgName,
            "invalid_array_arg" => TestError::InvalidArrayArg,
            "invalid_charset" => TestError::InvalidCharset,
            "invalid_form_data" => TestError::InvalidFormData,
            "invalid_post_type" => TestError::InvalidPostType,
            "missing_post_type" => TestError::MissingPostType,
            "team_added_to_org" => TestError::TeamAddedToOrg,
            "request_timeout" => TestError::RequestTimeout,
            _ => TestError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for TestError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        TestError::NotAuthed => "not_authed: No authentication token provided.",
TestError::InvalidAuth => "invalid_auth: Invalid authentication token.",
TestError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
TestError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
TestError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
TestError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
TestError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
TestError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
TestError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
TestError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
TestError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        TestError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        TestError::Unknown(ref s) => return write!(f, "{}", s),
                        TestError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for TestError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            TestError::MalformedResponse(_, ref e) => Some(e),
            TestError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
