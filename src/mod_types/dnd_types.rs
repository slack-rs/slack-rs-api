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
pub struct TeamInfoRequest {
    /// Comma-separated list of users to fetch Do Not Disturb status for
    pub users: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeamInfoResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<TeamInfoResponse, TeamInfoError<E>>> for TeamInfoResponse {
    fn into(self) -> Result<TeamInfoResponse, TeamInfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(TeamInfoError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum TeamInfoError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for TeamInfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => TeamInfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for TeamInfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TeamInfoError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            TeamInfoError::Unknown(ref s) => write!(f, "{}", s),
            TeamInfoError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for TeamInfoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            TeamInfoError::MalformedResponse(_, ref e) => Some(e),
            TeamInfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct EndSnoozeRequest {}

#[derive(Clone, Debug, Deserialize)]
pub struct EndSnoozeResponse {
    pub callstack: Option<String>,
    pub dnd_enabled: bool,
    error: Option<String>,
    pub next_dnd_end_ts: u64,
    pub next_dnd_start_ts: u64,
    #[serde(default)]
    ok: bool,
    pub snooze_enabled: bool,
}

impl<E: Error> Into<Result<EndSnoozeResponse, EndSnoozeError<E>>> for EndSnoozeResponse {
    fn into(self) -> Result<EndSnoozeResponse, EndSnoozeError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum EndSnoozeError<E: Error> {
    SnoozeNotActive,
    SnoozeEndFailed,
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

impl<'a, E: Error> From<&'a str> for EndSnoozeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "snooze_not_active" => EndSnoozeError::SnoozeNotActive,
            "snooze_end_failed" => EndSnoozeError::SnoozeEndFailed,
            "not_authed" => EndSnoozeError::NotAuthed,
            "invalid_auth" => EndSnoozeError::InvalidAuth,
            "account_inactive" => EndSnoozeError::AccountInactive,
            "token_revoked" => EndSnoozeError::TokenRevoked,
            "no_permission" => EndSnoozeError::NoPermission,
            "org_login_required" => EndSnoozeError::OrgLoginRequired,
            "user_is_bot" => EndSnoozeError::UserIsBot,
            "invalid_arg_name" => EndSnoozeError::InvalidArgName,
            "invalid_array_arg" => EndSnoozeError::InvalidArrayArg,
            "invalid_charset" => EndSnoozeError::InvalidCharset,
            "invalid_form_data" => EndSnoozeError::InvalidFormData,
            "invalid_post_type" => EndSnoozeError::InvalidPostType,
            "missing_post_type" => EndSnoozeError::MissingPostType,
            "team_added_to_org" => EndSnoozeError::TeamAddedToOrg,
            "invalid_json" => EndSnoozeError::InvalidJson,
            "json_not_object" => EndSnoozeError::JsonNotObject,
            "request_timeout" => EndSnoozeError::RequestTimeout,
            "upgrade_required" => EndSnoozeError::UpgradeRequired,
            "fatal_error" => EndSnoozeError::FatalError,
            _ => EndSnoozeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EndSnoozeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EndSnoozeError::SnoozeNotActive => write!(f, "Server returned error snooze_not_active"),
            EndSnoozeError::SnoozeEndFailed => write!(f, "Server returned error snooze_end_failed"),
            EndSnoozeError::NotAuthed => write!(f, "Server returned error not_authed"),
            EndSnoozeError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            EndSnoozeError::AccountInactive => write!(f, "Server returned error account_inactive"),
            EndSnoozeError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            EndSnoozeError::NoPermission => write!(f, "Server returned error no_permission"),
            EndSnoozeError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            EndSnoozeError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            EndSnoozeError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            EndSnoozeError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            EndSnoozeError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            EndSnoozeError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            EndSnoozeError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            EndSnoozeError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            EndSnoozeError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            EndSnoozeError::InvalidJson => write!(f, "Server returned error invalid_json"),
            EndSnoozeError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            EndSnoozeError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            EndSnoozeError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            EndSnoozeError::FatalError => write!(f, "Server returned error fatal_error"),
            EndSnoozeError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            EndSnoozeError::Unknown(ref s) => write!(f, "{}", s),
            EndSnoozeError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for EndSnoozeError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            EndSnoozeError::MalformedResponse(_, ref e) => Some(e),
            EndSnoozeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest {
    /// User to fetch status for (defaults to current user)
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub callstack: Option<String>,
    pub dnd_enabled: bool,
    error: Option<String>,
    pub next_dnd_end_ts: u64,
    pub next_dnd_start_ts: u64,
    #[serde(default)]
    ok: bool,
    pub snooze_enabled: Option<bool>,
    pub snooze_endtime: Option<u64>,
    pub snooze_remaining: Option<u64>,
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
    UserNotFound,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
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
            "user_not_found" => InfoError::UserNotFound,
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
            "token_revoked" => InfoError::TokenRevoked,
            "no_permission" => InfoError::NoPermission,
            "org_login_required" => InfoError::OrgLoginRequired,
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
            InfoError::UserNotFound => write!(f, "Server returned error user_not_found"),
            InfoError::NotAuthed => write!(f, "Server returned error not_authed"),
            InfoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InfoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InfoError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            InfoError::NoPermission => write!(f, "Server returned error no_permission"),
            InfoError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
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
pub struct SetSnoozeRequest {
    /// Authentication token. Requires scope: `dnd:write`
    pub token: String,
    /// Number of minutes, from now, to snooze until.
    pub num_minutes: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetSnoozeResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub snooze_enabled: bool,
    pub snooze_endtime: u64,
    pub snooze_remaining: u64,
}

impl<E: Error> Into<Result<SetSnoozeResponse, SetSnoozeError<E>>> for SetSnoozeResponse {
    fn into(self) -> Result<SetSnoozeResponse, SetSnoozeError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum SetSnoozeError<E: Error> {
    MissingDuration,
    SnoozeFailed,
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
    TooLong,
    FatalError,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetSnoozeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "missing_duration" => SetSnoozeError::MissingDuration,
            "snooze_failed" => SetSnoozeError::SnoozeFailed,
            "not_authed" => SetSnoozeError::NotAuthed,
            "invalid_auth" => SetSnoozeError::InvalidAuth,
            "account_inactive" => SetSnoozeError::AccountInactive,
            "token_revoked" => SetSnoozeError::TokenRevoked,
            "no_permission" => SetSnoozeError::NoPermission,
            "org_login_required" => SetSnoozeError::OrgLoginRequired,
            "user_is_bot" => SetSnoozeError::UserIsBot,
            "invalid_arg_name" => SetSnoozeError::InvalidArgName,
            "invalid_array_arg" => SetSnoozeError::InvalidArrayArg,
            "invalid_charset" => SetSnoozeError::InvalidCharset,
            "invalid_form_data" => SetSnoozeError::InvalidFormData,
            "invalid_post_type" => SetSnoozeError::InvalidPostType,
            "missing_post_type" => SetSnoozeError::MissingPostType,
            "team_added_to_org" => SetSnoozeError::TeamAddedToOrg,
            "invalid_json" => SetSnoozeError::InvalidJson,
            "json_not_object" => SetSnoozeError::JsonNotObject,
            "request_timeout" => SetSnoozeError::RequestTimeout,
            "upgrade_required" => SetSnoozeError::UpgradeRequired,
            "too_long" => SetSnoozeError::TooLong,
            "fatal_error" => SetSnoozeError::FatalError,
            _ => SetSnoozeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetSnoozeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetSnoozeError::MissingDuration => write!(f, "Server returned error missing_duration"),
            SetSnoozeError::SnoozeFailed => write!(f, "Server returned error snooze_failed"),
            SetSnoozeError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetSnoozeError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetSnoozeError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetSnoozeError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            SetSnoozeError::NoPermission => write!(f, "Server returned error no_permission"),
            SetSnoozeError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            SetSnoozeError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            SetSnoozeError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetSnoozeError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            SetSnoozeError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetSnoozeError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            SetSnoozeError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            SetSnoozeError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            SetSnoozeError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetSnoozeError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetSnoozeError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetSnoozeError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetSnoozeError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            SetSnoozeError::TooLong => write!(f, "Server returned error too_long"),
            SetSnoozeError::FatalError => write!(f, "Server returned error fatal_error"),
            SetSnoozeError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetSnoozeError::Unknown(ref s) => write!(f, "{}", s),
            SetSnoozeError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetSnoozeError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetSnoozeError::MalformedResponse(_, ref e) => Some(e),
            SetSnoozeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct EndDndRequest {}

#[derive(Clone, Debug, Deserialize)]
pub struct EndDndResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<EndDndResponse, EndDndError<E>>> for EndDndResponse {
    fn into(self) -> Result<EndDndResponse, EndDndError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum EndDndError<E: Error> {
    UnknownError,
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

impl<'a, E: Error> From<&'a str> for EndDndError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "unknown_error" => EndDndError::UnknownError,
            "not_authed" => EndDndError::NotAuthed,
            "invalid_auth" => EndDndError::InvalidAuth,
            "account_inactive" => EndDndError::AccountInactive,
            "token_revoked" => EndDndError::TokenRevoked,
            "no_permission" => EndDndError::NoPermission,
            "org_login_required" => EndDndError::OrgLoginRequired,
            "user_is_bot" => EndDndError::UserIsBot,
            "invalid_arg_name" => EndDndError::InvalidArgName,
            "invalid_array_arg" => EndDndError::InvalidArrayArg,
            "invalid_charset" => EndDndError::InvalidCharset,
            "invalid_form_data" => EndDndError::InvalidFormData,
            "invalid_post_type" => EndDndError::InvalidPostType,
            "missing_post_type" => EndDndError::MissingPostType,
            "team_added_to_org" => EndDndError::TeamAddedToOrg,
            "invalid_json" => EndDndError::InvalidJson,
            "json_not_object" => EndDndError::JsonNotObject,
            "request_timeout" => EndDndError::RequestTimeout,
            "upgrade_required" => EndDndError::UpgradeRequired,
            "fatal_error" => EndDndError::FatalError,
            _ => EndDndError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EndDndError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EndDndError::UnknownError => write!(f, "Server returned error unknown_error"),
            EndDndError::NotAuthed => write!(f, "Server returned error not_authed"),
            EndDndError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            EndDndError::AccountInactive => write!(f, "Server returned error account_inactive"),
            EndDndError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            EndDndError::NoPermission => write!(f, "Server returned error no_permission"),
            EndDndError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            EndDndError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            EndDndError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            EndDndError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            EndDndError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            EndDndError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            EndDndError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            EndDndError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            EndDndError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            EndDndError::InvalidJson => write!(f, "Server returned error invalid_json"),
            EndDndError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            EndDndError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            EndDndError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            EndDndError::FatalError => write!(f, "Server returned error fatal_error"),
            EndDndError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            EndDndError::Unknown(ref s) => write!(f, "{}", s),
            EndDndError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for EndDndError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            EndDndError::MalformedResponse(_, ref e) => Some(e),
            EndDndError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
