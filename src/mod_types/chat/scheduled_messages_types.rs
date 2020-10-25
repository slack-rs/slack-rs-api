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
pub struct ListRequest {
    /// The channel of the scheduled messages
    pub channel: Option<String>,
    /// A UNIX timestamp of the latest value in the time range
    pub latest: Option<u64>,
    /// A UNIX timestamp of the oldest value in the time range
    pub oldest: Option<u64>,
    /// Maximum number of original entries to return.
    pub limit: Option<u64>,
    /// For pagination purposes, this is the `cursor` value returned from a previous call to `chat.scheduledmessages.list` indicating where you want to start this call from.
    pub cursor: Option<String>,
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
    InvalidChannel,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    EkmAccessDenied,
    MissingScope,
    InvalidArguments,
    InvalidArgName,
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

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "invalid_channel" => ListError::InvalidChannel,
            "not_authed" => ListError::NotAuthed,
            "invalid_auth" => ListError::InvalidAuth,
            "account_inactive" => ListError::AccountInactive,
            "token_revoked" => ListError::TokenRevoked,
            "no_permission" => ListError::NoPermission,
            "org_login_required" => ListError::OrgLoginRequired,
            "ekm_access_denied" => ListError::EkmAccessDenied,
            "missing_scope" => ListError::MissingScope,
            "invalid_arguments" => ListError::InvalidArguments,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_post_type" => ListError::InvalidPostType,
            "missing_post_type" => ListError::MissingPostType,
            "team_added_to_org" => ListError::TeamAddedToOrg,
            "invalid_json" => ListError::InvalidJson,
            "json_not_object" => ListError::JsonNotObject,
            "request_timeout" => ListError::RequestTimeout,
            "upgrade_required" => ListError::UpgradeRequired,
            "fatal_error" => ListError::FatalError,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListError::InvalidChannel => write!(f, "Server returned error invalid_channel"),
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
            ListError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ListError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ListError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ListError::NoPermission => write!(f, "Server returned error no_permission"),
            ListError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            ListError::EkmAccessDenied => write!(f, "Server returned error ekm_access_denied"),
            ListError::MissingScope => write!(f, "Server returned error missing_scope"),
            ListError::InvalidArguments => write!(f, "Server returned error invalid_arguments"),
            ListError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ListError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ListError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ListError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ListError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ListError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ListError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ListError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ListError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ListError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            ListError::FatalError => write!(f, "Server returned error fatal_error"),
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
