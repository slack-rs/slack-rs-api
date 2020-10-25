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
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    FatalError,
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
            "token_revoked" => ConnectError::TokenRevoked,
            "no_permission" => ConnectError::NoPermission,
            "invalid_arg_name" => ConnectError::InvalidArgName,
            "invalid_array_arg" => ConnectError::InvalidArrayArg,
            "invalid_charset" => ConnectError::InvalidCharset,
            "invalid_form_data" => ConnectError::InvalidFormData,
            "invalid_post_type" => ConnectError::InvalidPostType,
            "missing_post_type" => ConnectError::MissingPostType,
            "team_added_to_org" => ConnectError::TeamAddedToOrg,
            "invalid_json" => ConnectError::InvalidJson,
            "json_not_object" => ConnectError::JsonNotObject,
            "request_timeout" => ConnectError::RequestTimeout,
            "upgrade_required" => ConnectError::UpgradeRequired,
            "fatal_error" => ConnectError::FatalError,
            _ => ConnectError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ConnectError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConnectError::NotAuthed => write!(f, "Server returned error not_authed"),
            ConnectError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ConnectError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ConnectError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ConnectError::NoPermission => write!(f, "Server returned error no_permission"),
            ConnectError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ConnectError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ConnectError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ConnectError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ConnectError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ConnectError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ConnectError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ConnectError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ConnectError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ConnectError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ConnectError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            ConnectError::FatalError => write!(f, "Server returned error fatal_error"),
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
