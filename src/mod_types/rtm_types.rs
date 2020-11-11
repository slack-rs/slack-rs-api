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
pub struct ConnectRequest {
    /// Batch presence deliveries via subscription. Enabling changes the shape of `presence_change` events. See [batch presence](/docs/presence-and-status#batching).
    pub batch_presence_aware: Option<bool>,
    /// Only deliver presence events when requested by subscription. See [presence subscriptions](/docs/presence-and-status#subscriptions).
    pub presence_sub: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConnectSelfInner {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConnectTeamInner {
    pub domain: String,
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConnectResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub r#_self: ConnectSelfInner,
    pub team: ConnectTeamInner,
    pub url: String,
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

impl<'a, E: Error> From<&'a str> for ConnectError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => ConnectError::AccountInactive,
            "fatal_error" => ConnectError::FatalError,
            "invalid_arg_name" => ConnectError::InvalidArgName,
            "invalid_array_arg" => ConnectError::InvalidArrayArg,
            "invalid_auth" => ConnectError::InvalidAuth,
            "invalid_charset" => ConnectError::InvalidCharset,
            "invalid_form_data" => ConnectError::InvalidFormData,
            "invalid_json" => ConnectError::InvalidJson,
            "invalid_post_type" => ConnectError::InvalidPostType,
            "json_not_object" => ConnectError::JsonNotObject,
            "missing_post_type" => ConnectError::MissingPostType,
            "no_permission" => ConnectError::NoPermission,
            "not_authed" => ConnectError::NotAuthed,
            "request_timeout" => ConnectError::RequestTimeout,
            "team_added_to_org" => ConnectError::TeamAddedToOrg,
            "token_revoked" => ConnectError::TokenRevoked,
            "upgrade_required" => ConnectError::UpgradeRequired,
            _ => ConnectError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ConnectError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConnectError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ConnectError::FatalError => write!(f, "Server returned error fatal_error"),
            ConnectError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ConnectError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ConnectError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ConnectError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ConnectError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ConnectError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ConnectError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ConnectError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ConnectError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ConnectError::NoPermission => write!(f, "Server returned error no_permission"),
            ConnectError::NotAuthed => write!(f, "Server returned error not_authed"),
            ConnectError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ConnectError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ConnectError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ConnectError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            ConnectError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ConnectError::Unknown(ref s) => write!(f, "{}", s),
            ConnectError::Client(ref inner) => write!(f, "{}", inner),
        }
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
