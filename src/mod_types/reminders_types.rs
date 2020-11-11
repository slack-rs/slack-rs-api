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
pub struct AddRequest<'a> {
    /// The content of the reminder
    pub text: Cow<'a, str>,
    /// When this reminder should happen: the Unix timestamp (up to five years from now), the number of seconds until the reminder (if within 24 hours), or a natural language description (Ex. "in 15 minutes," or "every Thursday")
    pub time: Cow<'a, str>,
    /// The user who will receive the reminder. If no user is specified, the reminder will go to user who created it.
    pub user: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddReminderInner {
    pub complete_ts: Option<u64>,
    pub creator: String,
    pub id: String,
    pub recurring: bool,
    pub text: String,
    pub time: Option<u64>,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub reminder: AddReminderInner,
}

impl<E: Error> Into<Result<AddResponse, AddError<E>>> for AddResponse {
    fn into(self) -> Result<AddResponse, AddError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum AddError<E: Error> {
    AccountInactive,
    CannotAddBot,
    CannotAddOthers,
    CannotAddOthersRecurring,
    CannotAddSlackbot,
    CannotParse,
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
    UserNotFound,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AddError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => AddError::AccountInactive,
            "cannot_add_bot" => AddError::CannotAddBot,
            "cannot_add_others" => AddError::CannotAddOthers,
            "cannot_add_others_recurring" => AddError::CannotAddOthersRecurring,
            "cannot_add_slackbot" => AddError::CannotAddSlackbot,
            "cannot_parse" => AddError::CannotParse,
            "fatal_error" => AddError::FatalError,
            "invalid_arg_name" => AddError::InvalidArgName,
            "invalid_array_arg" => AddError::InvalidArrayArg,
            "invalid_auth" => AddError::InvalidAuth,
            "invalid_charset" => AddError::InvalidCharset,
            "invalid_form_data" => AddError::InvalidFormData,
            "invalid_json" => AddError::InvalidJson,
            "invalid_post_type" => AddError::InvalidPostType,
            "json_not_object" => AddError::JsonNotObject,
            "missing_post_type" => AddError::MissingPostType,
            "no_permission" => AddError::NoPermission,
            "not_authed" => AddError::NotAuthed,
            "org_login_required" => AddError::OrgLoginRequired,
            "request_timeout" => AddError::RequestTimeout,
            "team_added_to_org" => AddError::TeamAddedToOrg,
            "token_revoked" => AddError::TokenRevoked,
            "upgrade_required" => AddError::UpgradeRequired,
            "user_is_bot" => AddError::UserIsBot,
            "user_not_found" => AddError::UserNotFound,
            _ => AddError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddError::AccountInactive => write!(f, "Server returned error account_inactive"),
            AddError::CannotAddBot => write!(f, "Server returned error cannot_add_bot"),
            AddError::CannotAddOthers => write!(f, "Server returned error cannot_add_others"),
            AddError::CannotAddOthersRecurring => {
                write!(f, "Server returned error cannot_add_others_recurring")
            }
            AddError::CannotAddSlackbot => write!(f, "Server returned error cannot_add_slackbot"),
            AddError::CannotParse => write!(f, "Server returned error cannot_parse"),
            AddError::FatalError => write!(f, "Server returned error fatal_error"),
            AddError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            AddError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            AddError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            AddError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            AddError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            AddError::InvalidJson => write!(f, "Server returned error invalid_json"),
            AddError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            AddError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            AddError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            AddError::NoPermission => write!(f, "Server returned error no_permission"),
            AddError::NotAuthed => write!(f, "Server returned error not_authed"),
            AddError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            AddError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            AddError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            AddError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            AddError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            AddError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            AddError::UserNotFound => write!(f, "Server returned error user_not_found"),
            AddError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AddError::Unknown(ref s) => write!(f, "{}", s),
            AddError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AddError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AddError::MalformedResponse(_, ref e) => Some(e),
            AddError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct CompleteRequest<'a> {
    /// The ID of the reminder to be marked as complete
    pub reminder: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CompleteResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<CompleteResponse, CompleteError<E>>> for CompleteResponse {
    fn into(self) -> Result<CompleteResponse, CompleteError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum CompleteError<E: Error> {
    AccountInactive,
    CannotCompleteOthers,
    CannotCompleteRecurring,
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
    NotFound,
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

impl<'a, E: Error> From<&'a str> for CompleteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => CompleteError::AccountInactive,
            "cannot_complete_others" => CompleteError::CannotCompleteOthers,
            "cannot_complete_recurring" => CompleteError::CannotCompleteRecurring,
            "fatal_error" => CompleteError::FatalError,
            "invalid_arg_name" => CompleteError::InvalidArgName,
            "invalid_array_arg" => CompleteError::InvalidArrayArg,
            "invalid_auth" => CompleteError::InvalidAuth,
            "invalid_charset" => CompleteError::InvalidCharset,
            "invalid_form_data" => CompleteError::InvalidFormData,
            "invalid_json" => CompleteError::InvalidJson,
            "invalid_post_type" => CompleteError::InvalidPostType,
            "json_not_object" => CompleteError::JsonNotObject,
            "missing_post_type" => CompleteError::MissingPostType,
            "no_permission" => CompleteError::NoPermission,
            "not_authed" => CompleteError::NotAuthed,
            "not_found" => CompleteError::NotFound,
            "org_login_required" => CompleteError::OrgLoginRequired,
            "request_timeout" => CompleteError::RequestTimeout,
            "team_added_to_org" => CompleteError::TeamAddedToOrg,
            "token_revoked" => CompleteError::TokenRevoked,
            "upgrade_required" => CompleteError::UpgradeRequired,
            "user_is_bot" => CompleteError::UserIsBot,
            _ => CompleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CompleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CompleteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            CompleteError::CannotCompleteOthers => {
                write!(f, "Server returned error cannot_complete_others")
            }
            CompleteError::CannotCompleteRecurring => {
                write!(f, "Server returned error cannot_complete_recurring")
            }
            CompleteError::FatalError => write!(f, "Server returned error fatal_error"),
            CompleteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            CompleteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            CompleteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            CompleteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            CompleteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            CompleteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            CompleteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            CompleteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            CompleteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            CompleteError::NoPermission => write!(f, "Server returned error no_permission"),
            CompleteError::NotAuthed => write!(f, "Server returned error not_authed"),
            CompleteError::NotFound => write!(f, "Server returned error not_found"),
            CompleteError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            CompleteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            CompleteError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            CompleteError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            CompleteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            CompleteError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            CompleteError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            CompleteError::Unknown(ref s) => write!(f, "{}", s),
            CompleteError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for CompleteError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            CompleteError::MalformedResponse(_, ref e) => Some(e),
            CompleteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest<'a> {
    /// The ID of the reminder
    pub reminder: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<DeleteResponse, DeleteError<E>>> for DeleteResponse {
    fn into(self) -> Result<DeleteResponse, DeleteError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum DeleteError<E: Error> {
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
    NotFound,
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

impl<'a, E: Error> From<&'a str> for DeleteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => DeleteError::AccountInactive,
            "fatal_error" => DeleteError::FatalError,
            "invalid_arg_name" => DeleteError::InvalidArgName,
            "invalid_array_arg" => DeleteError::InvalidArrayArg,
            "invalid_auth" => DeleteError::InvalidAuth,
            "invalid_charset" => DeleteError::InvalidCharset,
            "invalid_form_data" => DeleteError::InvalidFormData,
            "invalid_json" => DeleteError::InvalidJson,
            "invalid_post_type" => DeleteError::InvalidPostType,
            "json_not_object" => DeleteError::JsonNotObject,
            "missing_post_type" => DeleteError::MissingPostType,
            "no_permission" => DeleteError::NoPermission,
            "not_authed" => DeleteError::NotAuthed,
            "not_found" => DeleteError::NotFound,
            "org_login_required" => DeleteError::OrgLoginRequired,
            "request_timeout" => DeleteError::RequestTimeout,
            "team_added_to_org" => DeleteError::TeamAddedToOrg,
            "token_revoked" => DeleteError::TokenRevoked,
            "upgrade_required" => DeleteError::UpgradeRequired,
            "user_is_bot" => DeleteError::UserIsBot,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeleteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            DeleteError::FatalError => write!(f, "Server returned error fatal_error"),
            DeleteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            DeleteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            DeleteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            DeleteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            DeleteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            DeleteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            DeleteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            DeleteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            DeleteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            DeleteError::NoPermission => write!(f, "Server returned error no_permission"),
            DeleteError::NotAuthed => write!(f, "Server returned error not_authed"),
            DeleteError::NotFound => write!(f, "Server returned error not_found"),
            DeleteError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            DeleteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            DeleteError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            DeleteError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            DeleteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            DeleteError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            DeleteError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            DeleteError::Unknown(ref s) => write!(f, "{}", s),
            DeleteError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for DeleteError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DeleteError::MalformedResponse(_, ref e) => Some(e),
            DeleteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// The ID of the reminder
    pub reminder: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoReminderInner {
    pub complete_ts: Option<u64>,
    pub creator: String,
    pub id: String,
    pub recurring: bool,
    pub text: String,
    pub time: Option<u64>,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub reminder: InfoReminderInner,
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
    NotFound,
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
            "not_found" => InfoError::NotFound,
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
            InfoError::NotFound => write!(f, "Server returned error not_found"),
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
pub struct ListRequest {}

#[derive(Clone, Debug, Deserialize)]
pub struct ListRemindersInner {
    pub complete_ts: Option<u64>,
    pub creator: String,
    pub id: String,
    pub recurring: bool,
    pub text: String,
    pub time: Option<u64>,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub reminders: Vec<ListRemindersInner>,
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

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => ListError::AccountInactive,
            "fatal_error" => ListError::FatalError,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_array_arg" => ListError::InvalidArrayArg,
            "invalid_auth" => ListError::InvalidAuth,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_json" => ListError::InvalidJson,
            "invalid_post_type" => ListError::InvalidPostType,
            "json_not_object" => ListError::JsonNotObject,
            "missing_post_type" => ListError::MissingPostType,
            "no_permission" => ListError::NoPermission,
            "not_authed" => ListError::NotAuthed,
            "org_login_required" => ListError::OrgLoginRequired,
            "request_timeout" => ListError::RequestTimeout,
            "team_added_to_org" => ListError::TeamAddedToOrg,
            "token_revoked" => ListError::TokenRevoked,
            "upgrade_required" => ListError::UpgradeRequired,
            "user_is_bot" => ListError::UserIsBot,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ListError::FatalError => write!(f, "Server returned error fatal_error"),
            ListError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ListError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ListError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ListError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ListError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ListError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ListError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ListError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ListError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ListError::NoPermission => write!(f, "Server returned error no_permission"),
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
            ListError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            ListError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ListError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ListError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ListError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            ListError::UserIsBot => write!(f, "Server returned error user_is_bot"),
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
