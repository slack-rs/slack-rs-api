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

pub mod resources_types;
pub mod scopes_types;
pub mod users_types;

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct InfoRequest {}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResourcesInner {
    pub excluded_ids: Option<Vec<Vec<String>>>,
    pub ids: Vec<Vec<String>>,
    pub wildcard: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoAppHomeInner {
    pub resources: Option<InfoResourcesInner>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResources1Inner {
    pub excluded_ids: Option<Vec<Vec<String>>>,
    pub ids: Vec<Vec<String>>,
    pub wildcard: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoChannelInner {
    pub resources: Option<InfoResources1Inner>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResources2Inner {
    pub excluded_ids: Option<Vec<Vec<String>>>,
    pub ids: Vec<Vec<String>>,
    pub wildcard: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoGroupInner {
    pub resources: Option<InfoResources2Inner>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResources3Inner {
    pub excluded_ids: Option<Vec<Vec<String>>>,
    pub ids: Vec<Vec<String>>,
    pub wildcard: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoImInner {
    pub resources: Option<InfoResources3Inner>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResources4Inner {
    pub excluded_ids: Option<Vec<Vec<String>>>,
    pub ids: Vec<Vec<String>>,
    pub wildcard: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoMpimInner {
    pub resources: Option<InfoResources4Inner>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResources5Inner {
    pub excluded_ids: Option<Vec<Vec<String>>>,
    pub ids: Vec<Vec<String>>,
    pub wildcard: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoTeamInner {
    pub resources: InfoResources5Inner,
    pub scopes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoInfoInner {
    pub app_home: InfoAppHomeInner,
    pub channel: InfoChannelInner,
    pub group: InfoGroupInner,
    pub im: InfoImInner,
    pub mpim: InfoMpimInner,
    pub team: InfoTeamInner,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub info: InfoInfoInner,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<InfoResponse, InfoError<E>>> for InfoResponse {
    fn into(self) -> Result<InfoResponse, InfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum InfoError<E: Error> {
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
    UserIsBot,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => InfoError::AccountInactive,
            "fatal_error" => InfoError::FatalError,
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_auth" => InfoError::InvalidAuth,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_json" => InfoError::InvalidJson,
            "invalid_post_type" => InfoError::InvalidPostType,
            "json_not_object" => InfoError::JsonNotObject,
            "missing_post_type" => InfoError::MissingPostType,
            "no_permission" => InfoError::NoPermission,
            "not_authed" => InfoError::NotAuthed,
            "org_login_required" => InfoError::OrgLoginRequired,
            "request_timeout" => InfoError::RequestTimeout,
            "team_added_to_org" => InfoError::TeamAddedToOrg,
            "token_revoked" => InfoError::TokenRevoked,
            "upgrade_required" => InfoError::UpgradeRequired,
            "user_is_bot" => InfoError::UserIsBot,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InfoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InfoError::FatalError => write!(f, "Server returned error fatal_error"),
            InfoError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            InfoError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            InfoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InfoError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            InfoError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            InfoError::InvalidJson => write!(f, "Server returned error invalid_json"),
            InfoError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            InfoError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            InfoError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            InfoError::NoPermission => write!(f, "Server returned error no_permission"),
            InfoError::NotAuthed => write!(f, "Server returned error not_authed"),
            InfoError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            InfoError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            InfoError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            InfoError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            InfoError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            InfoError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            InfoError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            InfoError::Unknown(ref s) => write!(f, "{}", s),
            InfoError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for InfoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            InfoError::MalformedResponse(_, ref e) => Some(e),
            InfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RequestRequest<'a> {
    /// A comma separated list of scopes to request for
    pub scopes: Cow<'a, str>,
    /// Token used to trigger the permissions API
    pub trigger_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RequestResponse, RequestError<E>>> for RequestResponse {
    fn into(self) -> Result<RequestResponse, RequestError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum RequestError<E: Error> {
    AccountInactive,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    InvalidScope,
    InvalidTrigger,
    InvalidUser,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    TriggerExchanged,
    UpgradeRequired,
    UserIsBot,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RequestError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => RequestError::AccountInactive,
            "fatal_error" => RequestError::FatalError,
            "invalid_arg_name" => RequestError::InvalidArgName,
            "invalid_array_arg" => RequestError::InvalidArrayArg,
            "invalid_auth" => RequestError::InvalidAuth,
            "invalid_charset" => RequestError::InvalidCharset,
            "invalid_form_data" => RequestError::InvalidFormData,
            "invalid_json" => RequestError::InvalidJson,
            "invalid_post_type" => RequestError::InvalidPostType,
            "invalid_scope" => RequestError::InvalidScope,
            "invalid_trigger" => RequestError::InvalidTrigger,
            "invalid_user" => RequestError::InvalidUser,
            "json_not_object" => RequestError::JsonNotObject,
            "missing_post_type" => RequestError::MissingPostType,
            "no_permission" => RequestError::NoPermission,
            "not_authed" => RequestError::NotAuthed,
            "org_login_required" => RequestError::OrgLoginRequired,
            "request_timeout" => RequestError::RequestTimeout,
            "team_added_to_org" => RequestError::TeamAddedToOrg,
            "token_revoked" => RequestError::TokenRevoked,
            "trigger_exchanged" => RequestError::TriggerExchanged,
            "upgrade_required" => RequestError::UpgradeRequired,
            "user_is_bot" => RequestError::UserIsBot,
            _ => RequestError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RequestError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RequestError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RequestError::FatalError => write!(f, "Server returned error fatal_error"),
            RequestError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RequestError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RequestError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RequestError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RequestError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RequestError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RequestError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            RequestError::InvalidScope => write!(f, "Server returned error invalid_scope"),
            RequestError::InvalidTrigger => write!(f, "Server returned error invalid_trigger"),
            RequestError::InvalidUser => write!(f, "Server returned error invalid_user"),
            RequestError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RequestError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            RequestError::NoPermission => write!(f, "Server returned error no_permission"),
            RequestError::NotAuthed => write!(f, "Server returned error not_authed"),
            RequestError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            RequestError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            RequestError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            RequestError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            RequestError::TriggerExchanged => write!(f, "Server returned error trigger_exchanged"),
            RequestError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            RequestError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            RequestError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RequestError::Unknown(ref s) => write!(f, "{}", s),
            RequestError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RequestError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RequestError::MalformedResponse(_, ref e) => Some(e),
            RequestError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
