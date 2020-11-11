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

#![allow(unused_imports)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::blacklisted_name)]

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct RevokeRequest {
    /// Setting this parameter to `1` triggers a _testing mode_ where the specified token will not actually be revoked.
    pub test: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevokeResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub revoked: bool,
}

impl<E: Error> Into<Result<RevokeResponse, RevokeError<E>>> for RevokeResponse {
    fn into(self) -> Result<RevokeResponse, RevokeError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum RevokeError<E: Error> {
    AccountInactive,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
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
            "account_inactive" => RevokeError::AccountInactive,
            "fatal_error" => RevokeError::FatalError,
            "invalid_arg_name" => RevokeError::InvalidArgName,
            "invalid_array_arg" => RevokeError::InvalidArrayArg,
            "invalid_auth" => RevokeError::InvalidAuth,
            "invalid_charset" => RevokeError::InvalidCharset,
            "invalid_form_data" => RevokeError::InvalidFormData,
            "invalid_json" => RevokeError::InvalidJson,
            "invalid_post_type" => RevokeError::InvalidPostType,
            "json_not_object" => RevokeError::JsonNotObject,
            "missing_post_type" => RevokeError::MissingPostType,
            "no_permission" => RevokeError::NoPermission,
            "not_authed" => RevokeError::NotAuthed,
            "org_login_required" => RevokeError::OrgLoginRequired,
            "request_timeout" => RevokeError::RequestTimeout,
            "team_added_to_org" => RevokeError::TeamAddedToOrg,
            "token_revoked" => RevokeError::TokenRevoked,
            "upgrade_required" => RevokeError::UpgradeRequired,
            _ => RevokeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RevokeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RevokeError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RevokeError::FatalError => write!(f, "Server returned error fatal_error"),
            RevokeError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RevokeError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RevokeError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RevokeError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RevokeError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RevokeError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RevokeError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            RevokeError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RevokeError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            RevokeError::NoPermission => write!(f, "Server returned error no_permission"),
            RevokeError::NotAuthed => write!(f, "Server returned error not_authed"),
            RevokeError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            RevokeError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            RevokeError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            RevokeError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            RevokeError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            RevokeError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RevokeError::Unknown(ref s) => write!(f, "{}", s),
            RevokeError::Client(ref inner) => write!(f, "{}", inner),
        }
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

#[derive(Clone, Default, Debug)]
pub struct TestRequest {}

#[derive(Clone, Debug, Deserialize)]
pub struct TestResponse {
    pub bot_id: Option<String>,
    pub callstack: Option<String>,
    error: Option<String>,
    pub is_enterprise_install: Option<bool>,
    #[serde(default)]
    ok: bool,
    pub team: String,
    pub team_id: String,
    pub url: String,
    pub user: String,
    pub user_id: String,
}

impl<E: Error> Into<Result<TestResponse, TestError<E>>> for TestResponse {
    fn into(self) -> Result<TestResponse, TestError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum TestError<E: Error> {
    AccountInactive,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    NotAuthed,
    RequestTimeout,
    TokenRevoked,
    UpgradeRequired,
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
            "account_inactive" => TestError::AccountInactive,
            "invalid_arg_name" => TestError::InvalidArgName,
            "invalid_array_arg" => TestError::InvalidArrayArg,
            "invalid_auth" => TestError::InvalidAuth,
            "invalid_charset" => TestError::InvalidCharset,
            "invalid_form_data" => TestError::InvalidFormData,
            "invalid_json" => TestError::InvalidJson,
            "invalid_post_type" => TestError::InvalidPostType,
            "json_not_object" => TestError::JsonNotObject,
            "missing_post_type" => TestError::MissingPostType,
            "not_authed" => TestError::NotAuthed,
            "request_timeout" => TestError::RequestTimeout,
            "token_revoked" => TestError::TokenRevoked,
            "upgrade_required" => TestError::UpgradeRequired,
            _ => TestError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for TestError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TestError::AccountInactive => write!(f, "Server returned error account_inactive"),
            TestError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            TestError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            TestError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            TestError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            TestError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            TestError::InvalidJson => write!(f, "Server returned error invalid_json"),
            TestError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            TestError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            TestError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            TestError::NotAuthed => write!(f, "Server returned error not_authed"),
            TestError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            TestError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            TestError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            TestError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            TestError::Unknown(ref s) => write!(f, "{}", s),
            TestError::Client(ref inner) => write!(f, "{}", inner),
        }
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
