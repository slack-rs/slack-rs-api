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

pub mod resources_types;
pub mod scopes_types;
pub mod users_types;

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
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    UserIsBot,
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

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
            "token_revoked" => InfoError::TokenRevoked,
            "no_permission" => InfoError::NoPermission,
            "org_login_required" => InfoError::OrgLoginRequired,
            "user_is_bot" => InfoError::UserIsBot,
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_post_type" => InfoError::InvalidPostType,
            "missing_post_type" => InfoError::MissingPostType,
            "team_added_to_org" => InfoError::TeamAddedToOrg,
            "invalid_json" => InfoError::InvalidJson,
            "json_not_object" => InfoError::JsonNotObject,
            "request_timeout" => InfoError::RequestTimeout,
            "upgrade_required" => InfoError::UpgradeRequired,
            "fatal_error" => InfoError::FatalError,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InfoError::NotAuthed => write!(f, "Server returned error not_authed"),
            InfoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InfoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InfoError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            InfoError::NoPermission => write!(f, "Server returned error no_permission"),
            InfoError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            InfoError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            InfoError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            InfoError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            InfoError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            InfoError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            InfoError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            InfoError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            InfoError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            InfoError::InvalidJson => write!(f, "Server returned error invalid_json"),
            InfoError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            InfoError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            InfoError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            InfoError::FatalError => write!(f, "Server returned error fatal_error"),
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
pub struct RequestRequest {
    /// A comma separated list of scopes to request for
    pub scopes: String,
    /// Token used to trigger the permissions API
    pub trigger_id: String,
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
    InvalidTrigger,
    TriggerExchanged,
    InvalidScope,
    InvalidUser,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    UserIsBot,
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

impl<'a, E: Error> From<&'a str> for RequestError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "invalid_trigger" => RequestError::InvalidTrigger,
            "trigger_exchanged" => RequestError::TriggerExchanged,
            "invalid_scope" => RequestError::InvalidScope,
            "invalid_user" => RequestError::InvalidUser,
            "not_authed" => RequestError::NotAuthed,
            "invalid_auth" => RequestError::InvalidAuth,
            "account_inactive" => RequestError::AccountInactive,
            "token_revoked" => RequestError::TokenRevoked,
            "no_permission" => RequestError::NoPermission,
            "org_login_required" => RequestError::OrgLoginRequired,
            "user_is_bot" => RequestError::UserIsBot,
            "invalid_arg_name" => RequestError::InvalidArgName,
            "invalid_array_arg" => RequestError::InvalidArrayArg,
            "invalid_charset" => RequestError::InvalidCharset,
            "invalid_form_data" => RequestError::InvalidFormData,
            "invalid_post_type" => RequestError::InvalidPostType,
            "missing_post_type" => RequestError::MissingPostType,
            "team_added_to_org" => RequestError::TeamAddedToOrg,
            "invalid_json" => RequestError::InvalidJson,
            "json_not_object" => RequestError::JsonNotObject,
            "request_timeout" => RequestError::RequestTimeout,
            "upgrade_required" => RequestError::UpgradeRequired,
            "fatal_error" => RequestError::FatalError,
            _ => RequestError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RequestError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RequestError::InvalidTrigger => write!(f, "Server returned error invalid_trigger"),
            RequestError::TriggerExchanged => write!(f, "Server returned error trigger_exchanged"),
            RequestError::InvalidScope => write!(f, "Server returned error invalid_scope"),
            RequestError::InvalidUser => write!(f, "Server returned error invalid_user"),
            RequestError::NotAuthed => write!(f, "Server returned error not_authed"),
            RequestError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RequestError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RequestError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            RequestError::NoPermission => write!(f, "Server returned error no_permission"),
            RequestError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            RequestError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            RequestError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RequestError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RequestError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RequestError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RequestError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            RequestError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            RequestError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            RequestError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RequestError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RequestError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            RequestError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            RequestError::FatalError => write!(f, "Server returned error fatal_error"),
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
