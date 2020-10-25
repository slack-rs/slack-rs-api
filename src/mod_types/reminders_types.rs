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
pub struct AddRequest {
    /// The content of the reminder
    pub text: String,
    /// When this reminder should happen: the Unix timestamp (up to five years from now), the number of seconds until the reminder (if within 24 hours), or a natural language description (Ex. "in 15 minutes," or "every Thursday")
    pub time: String,
    /// The user who will receive the reminder. If no user is specified, the reminder will go to user who created it.
    pub user: Option<String>,
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
    CannotParse,
    UserNotFound,
    CannotAddBot,
    CannotAddSlackbot,
    CannotAddOthers,
    CannotAddOthersRecurring,
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

impl<'a, E: Error> From<&'a str> for AddError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "cannot_parse" => AddError::CannotParse,
            "user_not_found" => AddError::UserNotFound,
            "cannot_add_bot" => AddError::CannotAddBot,
            "cannot_add_slackbot" => AddError::CannotAddSlackbot,
            "cannot_add_others" => AddError::CannotAddOthers,
            "cannot_add_others_recurring" => AddError::CannotAddOthersRecurring,
            "not_authed" => AddError::NotAuthed,
            "invalid_auth" => AddError::InvalidAuth,
            "account_inactive" => AddError::AccountInactive,
            "token_revoked" => AddError::TokenRevoked,
            "no_permission" => AddError::NoPermission,
            "org_login_required" => AddError::OrgLoginRequired,
            "user_is_bot" => AddError::UserIsBot,
            "invalid_arg_name" => AddError::InvalidArgName,
            "invalid_array_arg" => AddError::InvalidArrayArg,
            "invalid_charset" => AddError::InvalidCharset,
            "invalid_form_data" => AddError::InvalidFormData,
            "invalid_post_type" => AddError::InvalidPostType,
            "missing_post_type" => AddError::MissingPostType,
            "team_added_to_org" => AddError::TeamAddedToOrg,
            "invalid_json" => AddError::InvalidJson,
            "json_not_object" => AddError::JsonNotObject,
            "request_timeout" => AddError::RequestTimeout,
            "upgrade_required" => AddError::UpgradeRequired,
            "fatal_error" => AddError::FatalError,
            _ => AddError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddError::CannotParse => write!(f, "Server returned error cannot_parse"),
            AddError::UserNotFound => write!(f, "Server returned error user_not_found"),
            AddError::CannotAddBot => write!(f, "Server returned error cannot_add_bot"),
            AddError::CannotAddSlackbot => write!(f, "Server returned error cannot_add_slackbot"),
            AddError::CannotAddOthers => write!(f, "Server returned error cannot_add_others"),
            AddError::CannotAddOthersRecurring => {
                write!(f, "Server returned error cannot_add_others_recurring")
            }
            AddError::NotAuthed => write!(f, "Server returned error not_authed"),
            AddError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            AddError::AccountInactive => write!(f, "Server returned error account_inactive"),
            AddError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            AddError::NoPermission => write!(f, "Server returned error no_permission"),
            AddError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            AddError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            AddError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            AddError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            AddError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            AddError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            AddError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            AddError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            AddError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            AddError::InvalidJson => write!(f, "Server returned error invalid_json"),
            AddError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            AddError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            AddError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            AddError::FatalError => write!(f, "Server returned error fatal_error"),
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
pub struct DeleteRequest {
    /// The ID of the reminder
    pub reminder: Option<String>,
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
    NotFound,
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

impl<'a, E: Error> From<&'a str> for DeleteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_found" => DeleteError::NotFound,
            "not_authed" => DeleteError::NotAuthed,
            "invalid_auth" => DeleteError::InvalidAuth,
            "account_inactive" => DeleteError::AccountInactive,
            "token_revoked" => DeleteError::TokenRevoked,
            "no_permission" => DeleteError::NoPermission,
            "org_login_required" => DeleteError::OrgLoginRequired,
            "user_is_bot" => DeleteError::UserIsBot,
            "invalid_arg_name" => DeleteError::InvalidArgName,
            "invalid_array_arg" => DeleteError::InvalidArrayArg,
            "invalid_charset" => DeleteError::InvalidCharset,
            "invalid_form_data" => DeleteError::InvalidFormData,
            "invalid_post_type" => DeleteError::InvalidPostType,
            "missing_post_type" => DeleteError::MissingPostType,
            "team_added_to_org" => DeleteError::TeamAddedToOrg,
            "invalid_json" => DeleteError::InvalidJson,
            "json_not_object" => DeleteError::JsonNotObject,
            "request_timeout" => DeleteError::RequestTimeout,
            "upgrade_required" => DeleteError::UpgradeRequired,
            "fatal_error" => DeleteError::FatalError,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeleteError::NotFound => write!(f, "Server returned error not_found"),
            DeleteError::NotAuthed => write!(f, "Server returned error not_authed"),
            DeleteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            DeleteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            DeleteError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            DeleteError::NoPermission => write!(f, "Server returned error no_permission"),
            DeleteError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            DeleteError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            DeleteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            DeleteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            DeleteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            DeleteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            DeleteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            DeleteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            DeleteError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            DeleteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            DeleteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            DeleteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            DeleteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            DeleteError::FatalError => write!(f, "Server returned error fatal_error"),
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

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => ListError::NotAuthed,
            "invalid_auth" => ListError::InvalidAuth,
            "account_inactive" => ListError::AccountInactive,
            "token_revoked" => ListError::TokenRevoked,
            "no_permission" => ListError::NoPermission,
            "org_login_required" => ListError::OrgLoginRequired,
            "user_is_bot" => ListError::UserIsBot,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_array_arg" => ListError::InvalidArrayArg,
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
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
            ListError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ListError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ListError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ListError::NoPermission => write!(f, "Server returned error no_permission"),
            ListError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            ListError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            ListError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ListError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
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

#[derive(Clone, Default, Debug)]
pub struct CompleteRequest {
    /// The ID of the reminder to be marked as complete
    pub reminder: Option<String>,
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
    NotFound,
    CannotCompleteRecurring,
    CannotCompleteOthers,
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

impl<'a, E: Error> From<&'a str> for CompleteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_found" => CompleteError::NotFound,
            "cannot_complete_recurring" => CompleteError::CannotCompleteRecurring,
            "cannot_complete_others" => CompleteError::CannotCompleteOthers,
            "not_authed" => CompleteError::NotAuthed,
            "invalid_auth" => CompleteError::InvalidAuth,
            "account_inactive" => CompleteError::AccountInactive,
            "token_revoked" => CompleteError::TokenRevoked,
            "no_permission" => CompleteError::NoPermission,
            "org_login_required" => CompleteError::OrgLoginRequired,
            "user_is_bot" => CompleteError::UserIsBot,
            "invalid_arg_name" => CompleteError::InvalidArgName,
            "invalid_array_arg" => CompleteError::InvalidArrayArg,
            "invalid_charset" => CompleteError::InvalidCharset,
            "invalid_form_data" => CompleteError::InvalidFormData,
            "invalid_post_type" => CompleteError::InvalidPostType,
            "missing_post_type" => CompleteError::MissingPostType,
            "team_added_to_org" => CompleteError::TeamAddedToOrg,
            "invalid_json" => CompleteError::InvalidJson,
            "json_not_object" => CompleteError::JsonNotObject,
            "request_timeout" => CompleteError::RequestTimeout,
            "upgrade_required" => CompleteError::UpgradeRequired,
            "fatal_error" => CompleteError::FatalError,
            _ => CompleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CompleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CompleteError::NotFound => write!(f, "Server returned error not_found"),
            CompleteError::CannotCompleteRecurring => {
                write!(f, "Server returned error cannot_complete_recurring")
            }
            CompleteError::CannotCompleteOthers => {
                write!(f, "Server returned error cannot_complete_others")
            }
            CompleteError::NotAuthed => write!(f, "Server returned error not_authed"),
            CompleteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            CompleteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            CompleteError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            CompleteError::NoPermission => write!(f, "Server returned error no_permission"),
            CompleteError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            CompleteError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            CompleteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            CompleteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            CompleteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            CompleteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            CompleteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            CompleteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            CompleteError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            CompleteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            CompleteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            CompleteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            CompleteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            CompleteError::FatalError => write!(f, "Server returned error fatal_error"),
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
pub struct InfoRequest {
    /// The ID of the reminder
    pub reminder: Option<String>,
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
    NotFound,
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
            "not_found" => InfoError::NotFound,
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
            InfoError::NotFound => write!(f, "Server returned error not_found"),
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
