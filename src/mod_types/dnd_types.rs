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
    UnknownError,
    UpgradeRequired,
    UserIsBot,
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
            "account_inactive" => EndDndError::AccountInactive,
            "fatal_error" => EndDndError::FatalError,
            "invalid_arg_name" => EndDndError::InvalidArgName,
            "invalid_array_arg" => EndDndError::InvalidArrayArg,
            "invalid_auth" => EndDndError::InvalidAuth,
            "invalid_charset" => EndDndError::InvalidCharset,
            "invalid_form_data" => EndDndError::InvalidFormData,
            "invalid_json" => EndDndError::InvalidJson,
            "invalid_post_type" => EndDndError::InvalidPostType,
            "json_not_object" => EndDndError::JsonNotObject,
            "missing_post_type" => EndDndError::MissingPostType,
            "no_permission" => EndDndError::NoPermission,
            "not_authed" => EndDndError::NotAuthed,
            "org_login_required" => EndDndError::OrgLoginRequired,
            "request_timeout" => EndDndError::RequestTimeout,
            "team_added_to_org" => EndDndError::TeamAddedToOrg,
            "token_revoked" => EndDndError::TokenRevoked,
            "unknown_error" => EndDndError::UnknownError,
            "upgrade_required" => EndDndError::UpgradeRequired,
            "user_is_bot" => EndDndError::UserIsBot,
            _ => EndDndError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EndDndError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EndDndError::AccountInactive => write!(f, "Server returned error account_inactive"),
            EndDndError::FatalError => write!(f, "Server returned error fatal_error"),
            EndDndError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            EndDndError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            EndDndError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            EndDndError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            EndDndError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            EndDndError::InvalidJson => write!(f, "Server returned error invalid_json"),
            EndDndError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            EndDndError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            EndDndError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            EndDndError::NoPermission => write!(f, "Server returned error no_permission"),
            EndDndError::NotAuthed => write!(f, "Server returned error not_authed"),
            EndDndError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            EndDndError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            EndDndError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            EndDndError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            EndDndError::UnknownError => write!(f, "Server returned error unknown_error"),
            EndDndError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            EndDndError::UserIsBot => write!(f, "Server returned error user_is_bot"),
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
    SnoozeEndFailed,
    SnoozeNotActive,
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

impl<'a, E: Error> From<&'a str> for EndSnoozeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => EndSnoozeError::AccountInactive,
            "fatal_error" => EndSnoozeError::FatalError,
            "invalid_arg_name" => EndSnoozeError::InvalidArgName,
            "invalid_array_arg" => EndSnoozeError::InvalidArrayArg,
            "invalid_auth" => EndSnoozeError::InvalidAuth,
            "invalid_charset" => EndSnoozeError::InvalidCharset,
            "invalid_form_data" => EndSnoozeError::InvalidFormData,
            "invalid_json" => EndSnoozeError::InvalidJson,
            "invalid_post_type" => EndSnoozeError::InvalidPostType,
            "json_not_object" => EndSnoozeError::JsonNotObject,
            "missing_post_type" => EndSnoozeError::MissingPostType,
            "no_permission" => EndSnoozeError::NoPermission,
            "not_authed" => EndSnoozeError::NotAuthed,
            "org_login_required" => EndSnoozeError::OrgLoginRequired,
            "request_timeout" => EndSnoozeError::RequestTimeout,
            "snooze_end_failed" => EndSnoozeError::SnoozeEndFailed,
            "snooze_not_active" => EndSnoozeError::SnoozeNotActive,
            "team_added_to_org" => EndSnoozeError::TeamAddedToOrg,
            "token_revoked" => EndSnoozeError::TokenRevoked,
            "upgrade_required" => EndSnoozeError::UpgradeRequired,
            "user_is_bot" => EndSnoozeError::UserIsBot,
            _ => EndSnoozeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EndSnoozeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EndSnoozeError::AccountInactive => write!(f, "Server returned error account_inactive"),
            EndSnoozeError::FatalError => write!(f, "Server returned error fatal_error"),
            EndSnoozeError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            EndSnoozeError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            EndSnoozeError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            EndSnoozeError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            EndSnoozeError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            EndSnoozeError::InvalidJson => write!(f, "Server returned error invalid_json"),
            EndSnoozeError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            EndSnoozeError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            EndSnoozeError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            EndSnoozeError::NoPermission => write!(f, "Server returned error no_permission"),
            EndSnoozeError::NotAuthed => write!(f, "Server returned error not_authed"),
            EndSnoozeError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            EndSnoozeError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            EndSnoozeError::SnoozeEndFailed => write!(f, "Server returned error snooze_end_failed"),
            EndSnoozeError::SnoozeNotActive => write!(f, "Server returned error snooze_not_active"),
            EndSnoozeError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            EndSnoozeError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            EndSnoozeError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            EndSnoozeError::UserIsBot => write!(f, "Server returned error user_is_bot"),
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
pub struct InfoRequest<'a> {
    /// User to fetch status for (defaults to current user)
    pub user: Option<Cow<'a, str>>,
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
    UserNotFound,
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
            "user_not_found" => InfoError::UserNotFound,
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
            InfoError::UserNotFound => write!(f, "Server returned error user_not_found"),
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
pub struct SetSnoozeRequest<'a> {
    /// Number of minutes, from now, to snooze until.
    pub num_minutes: Cow<'a, str>,
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
    MissingDuration,
    MissingPostType,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    SnoozeFailed,
    TeamAddedToOrg,
    TokenRevoked,
    TooLong,
    UpgradeRequired,
    UserIsBot,
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
            "account_inactive" => SetSnoozeError::AccountInactive,
            "fatal_error" => SetSnoozeError::FatalError,
            "invalid_arg_name" => SetSnoozeError::InvalidArgName,
            "invalid_array_arg" => SetSnoozeError::InvalidArrayArg,
            "invalid_auth" => SetSnoozeError::InvalidAuth,
            "invalid_charset" => SetSnoozeError::InvalidCharset,
            "invalid_form_data" => SetSnoozeError::InvalidFormData,
            "invalid_json" => SetSnoozeError::InvalidJson,
            "invalid_post_type" => SetSnoozeError::InvalidPostType,
            "json_not_object" => SetSnoozeError::JsonNotObject,
            "missing_duration" => SetSnoozeError::MissingDuration,
            "missing_post_type" => SetSnoozeError::MissingPostType,
            "no_permission" => SetSnoozeError::NoPermission,
            "not_authed" => SetSnoozeError::NotAuthed,
            "org_login_required" => SetSnoozeError::OrgLoginRequired,
            "request_timeout" => SetSnoozeError::RequestTimeout,
            "snooze_failed" => SetSnoozeError::SnoozeFailed,
            "team_added_to_org" => SetSnoozeError::TeamAddedToOrg,
            "token_revoked" => SetSnoozeError::TokenRevoked,
            "too_long" => SetSnoozeError::TooLong,
            "upgrade_required" => SetSnoozeError::UpgradeRequired,
            "user_is_bot" => SetSnoozeError::UserIsBot,
            _ => SetSnoozeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetSnoozeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetSnoozeError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetSnoozeError::FatalError => write!(f, "Server returned error fatal_error"),
            SetSnoozeError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetSnoozeError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            SetSnoozeError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetSnoozeError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetSnoozeError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            SetSnoozeError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetSnoozeError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            SetSnoozeError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetSnoozeError::MissingDuration => write!(f, "Server returned error missing_duration"),
            SetSnoozeError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            SetSnoozeError::NoPermission => write!(f, "Server returned error no_permission"),
            SetSnoozeError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetSnoozeError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            SetSnoozeError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetSnoozeError::SnoozeFailed => write!(f, "Server returned error snooze_failed"),
            SetSnoozeError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetSnoozeError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            SetSnoozeError::TooLong => write!(f, "Server returned error too_long"),
            SetSnoozeError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            SetSnoozeError::UserIsBot => write!(f, "Server returned error user_is_bot"),
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
pub struct TeamInfoRequest<'a> {
    /// Comma-separated list of users to fetch Do Not Disturb status for
    pub users: Option<Cow<'a, str>>,
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
