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
pub struct ListRequest<'a> {
    /// The channel of the scheduled messages
    pub channel: Option<Cow<'a, str>>,
    /// For pagination purposes, this is the `cursor` value returned from a previous call to `chat.scheduledmessages.list` indicating where you want to start this call from.
    pub cursor: Option<Cow<'a, str>>,
    /// A UNIX timestamp of the latest value in the time range
    pub latest: Option<f64>,
    /// Maximum number of original entries to return.
    pub limit: Option<u64>,
    /// A UNIX timestamp of the oldest value in the time range
    pub oldest: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseMetadataInner {
    pub next_cursor: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListScheduledMessagesInner {
    pub channel_id: String,
    pub date_created: u64,
    pub id: String,
    pub post_at: u64,
    pub text: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub response_metadata: ListResponseMetadataInner,
    pub scheduled_messages: Vec<ListScheduledMessagesInner>,
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
    AccountInactive,
    EkmAccessDenied,
    FatalError,
    InvalidArgName,
    InvalidArguments,
    InvalidAuth,
    InvalidChannel,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    MissingScope,
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

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => ListError::AccountInactive,
            "ekm_access_denied" => ListError::EkmAccessDenied,
            "fatal_error" => ListError::FatalError,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_arguments" => ListError::InvalidArguments,
            "invalid_auth" => ListError::InvalidAuth,
            "invalid_channel" => ListError::InvalidChannel,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_json" => ListError::InvalidJson,
            "invalid_post_type" => ListError::InvalidPostType,
            "json_not_object" => ListError::JsonNotObject,
            "missing_post_type" => ListError::MissingPostType,
            "missing_scope" => ListError::MissingScope,
            "no_permission" => ListError::NoPermission,
            "not_authed" => ListError::NotAuthed,
            "org_login_required" => ListError::OrgLoginRequired,
            "request_timeout" => ListError::RequestTimeout,
            "team_added_to_org" => ListError::TeamAddedToOrg,
            "token_revoked" => ListError::TokenRevoked,
            "upgrade_required" => ListError::UpgradeRequired,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ListError::EkmAccessDenied => write!(f, "Server returned error ekm_access_denied"),
            ListError::FatalError => write!(f, "Server returned error fatal_error"),
            ListError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ListError::InvalidArguments => write!(f, "Server returned error invalid_arguments"),
            ListError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ListError::InvalidChannel => write!(f, "Server returned error invalid_channel"),
            ListError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ListError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ListError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ListError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ListError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ListError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ListError::MissingScope => write!(f, "Server returned error missing_scope"),
            ListError::NoPermission => write!(f, "Server returned error no_permission"),
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
            ListError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            ListError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ListError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ListError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ListError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            ListError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ListError::Unknown(ref s) => write!(f, "{}", s),
            ListError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ListError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ListError::MalformedResponse(_, ref e) => Some(e),
            ListError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
