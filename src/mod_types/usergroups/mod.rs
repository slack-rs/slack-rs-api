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

pub mod users_types;

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct CreateRequest<'a> {
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<Cow<'a, str>>,
    /// A short description of the User Group.
    pub description: Option<Cow<'a, str>>,
    /// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<Cow<'a, str>>,
    /// Include the number of users in each User Group.
    pub include_count: Option<bool>,
    /// A name for the User Group. Must be unique among User Groups.
    pub name: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreatePrefsInner {
    pub channels: Vec<String>,
    pub groups: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateUsergroupInner {
    pub auto_provision: bool,
    pub auto_type: Vec<String>,
    pub channel_count: Option<u64>,
    pub created_by: String,
    pub date_create: u64,
    pub date_delete: u64,
    pub date_update: u64,
    pub deleted_by: Vec<String>,
    pub description: String,
    pub enterprise_subteam_id: String,
    pub handle: String,
    pub id: String,
    pub is_external: bool,
    pub is_subteam: bool,
    pub is_usergroup: bool,
    pub name: String,
    pub prefs: CreatePrefsInner,
    pub team_id: String,
    pub updated_by: String,
    pub user_count: Option<u64>,
    pub users: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroup: CreateUsergroupInner,
}

impl<E: Error> Into<Result<CreateResponse, CreateError<E>>> for CreateResponse {
    fn into(self) -> Result<CreateResponse, CreateError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum CreateError<E: Error> {
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
    PermissionDenied,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
    UserIsBot,
    UserIsRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for CreateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => CreateError::AccountInactive,
            "fatal_error" => CreateError::FatalError,
            "invalid_arg_name" => CreateError::InvalidArgName,
            "invalid_array_arg" => CreateError::InvalidArrayArg,
            "invalid_auth" => CreateError::InvalidAuth,
            "invalid_charset" => CreateError::InvalidCharset,
            "invalid_form_data" => CreateError::InvalidFormData,
            "invalid_json" => CreateError::InvalidJson,
            "invalid_post_type" => CreateError::InvalidPostType,
            "json_not_object" => CreateError::JsonNotObject,
            "missing_post_type" => CreateError::MissingPostType,
            "no_permission" => CreateError::NoPermission,
            "not_authed" => CreateError::NotAuthed,
            "org_login_required" => CreateError::OrgLoginRequired,
            "permission_denied" => CreateError::PermissionDenied,
            "request_timeout" => CreateError::RequestTimeout,
            "team_added_to_org" => CreateError::TeamAddedToOrg,
            "token_revoked" => CreateError::TokenRevoked,
            "upgrade_required" => CreateError::UpgradeRequired,
            "user_is_bot" => CreateError::UserIsBot,
            "user_is_restricted" => CreateError::UserIsRestricted,
            _ => CreateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CreateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CreateError::AccountInactive => write!(f, "Server returned error account_inactive"),
            CreateError::FatalError => write!(f, "Server returned error fatal_error"),
            CreateError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            CreateError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            CreateError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            CreateError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            CreateError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            CreateError::InvalidJson => write!(f, "Server returned error invalid_json"),
            CreateError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            CreateError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            CreateError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            CreateError::NoPermission => write!(f, "Server returned error no_permission"),
            CreateError::NotAuthed => write!(f, "Server returned error not_authed"),
            CreateError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            CreateError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            CreateError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            CreateError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            CreateError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            CreateError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            CreateError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            CreateError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            CreateError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            CreateError::Unknown(ref s) => write!(f, "{}", s),
            CreateError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for CreateError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            CreateError::MalformedResponse(_, ref e) => Some(e),
            CreateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct DisableRequest<'a> {
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
    /// The encoded ID of the User Group to disable.
    pub usergroup: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisablePrefsInner {
    pub channels: Vec<String>,
    pub groups: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisableUsergroupInner {
    pub auto_provision: bool,
    pub auto_type: Vec<String>,
    pub channel_count: Option<u64>,
    pub created_by: String,
    pub date_create: u64,
    pub date_delete: u64,
    pub date_update: u64,
    pub deleted_by: Vec<String>,
    pub description: String,
    pub enterprise_subteam_id: String,
    pub handle: String,
    pub id: String,
    pub is_external: bool,
    pub is_subteam: bool,
    pub is_usergroup: bool,
    pub name: String,
    pub prefs: DisablePrefsInner,
    pub team_id: String,
    pub updated_by: String,
    pub user_count: Option<u64>,
    pub users: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisableResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroup: DisableUsergroupInner,
}

impl<E: Error> Into<Result<DisableResponse, DisableError<E>>> for DisableResponse {
    fn into(self) -> Result<DisableResponse, DisableError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum DisableError<E: Error> {
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
    PermissionDenied,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
    UserIsBot,
    UserIsRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for DisableError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => DisableError::AccountInactive,
            "fatal_error" => DisableError::FatalError,
            "invalid_arg_name" => DisableError::InvalidArgName,
            "invalid_array_arg" => DisableError::InvalidArrayArg,
            "invalid_auth" => DisableError::InvalidAuth,
            "invalid_charset" => DisableError::InvalidCharset,
            "invalid_form_data" => DisableError::InvalidFormData,
            "invalid_json" => DisableError::InvalidJson,
            "invalid_post_type" => DisableError::InvalidPostType,
            "json_not_object" => DisableError::JsonNotObject,
            "missing_post_type" => DisableError::MissingPostType,
            "no_permission" => DisableError::NoPermission,
            "not_authed" => DisableError::NotAuthed,
            "org_login_required" => DisableError::OrgLoginRequired,
            "permission_denied" => DisableError::PermissionDenied,
            "request_timeout" => DisableError::RequestTimeout,
            "team_added_to_org" => DisableError::TeamAddedToOrg,
            "token_revoked" => DisableError::TokenRevoked,
            "upgrade_required" => DisableError::UpgradeRequired,
            "user_is_bot" => DisableError::UserIsBot,
            "user_is_restricted" => DisableError::UserIsRestricted,
            _ => DisableError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DisableError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DisableError::AccountInactive => write!(f, "Server returned error account_inactive"),
            DisableError::FatalError => write!(f, "Server returned error fatal_error"),
            DisableError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            DisableError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            DisableError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            DisableError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            DisableError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            DisableError::InvalidJson => write!(f, "Server returned error invalid_json"),
            DisableError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            DisableError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            DisableError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            DisableError::NoPermission => write!(f, "Server returned error no_permission"),
            DisableError::NotAuthed => write!(f, "Server returned error not_authed"),
            DisableError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            DisableError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            DisableError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            DisableError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            DisableError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            DisableError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            DisableError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            DisableError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            DisableError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            DisableError::Unknown(ref s) => write!(f, "{}", s),
            DisableError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for DisableError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DisableError::MalformedResponse(_, ref e) => Some(e),
            DisableError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct EnableRequest<'a> {
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
    /// The encoded ID of the User Group to enable.
    pub usergroup: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EnablePrefsInner {
    pub channels: Vec<String>,
    pub groups: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EnableUsergroupInner {
    pub auto_provision: bool,
    pub auto_type: Vec<String>,
    pub channel_count: Option<u64>,
    pub created_by: String,
    pub date_create: u64,
    pub date_delete: u64,
    pub date_update: u64,
    pub deleted_by: Vec<String>,
    pub description: String,
    pub enterprise_subteam_id: String,
    pub handle: String,
    pub id: String,
    pub is_external: bool,
    pub is_subteam: bool,
    pub is_usergroup: bool,
    pub name: String,
    pub prefs: EnablePrefsInner,
    pub team_id: String,
    pub updated_by: String,
    pub user_count: Option<u64>,
    pub users: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EnableResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroup: EnableUsergroupInner,
}

impl<E: Error> Into<Result<EnableResponse, EnableError<E>>> for EnableResponse {
    fn into(self) -> Result<EnableResponse, EnableError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum EnableError<E: Error> {
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
    MissingCharset,
    MissingPostType,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    SuperfluousCharset,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequire,
    UserIsBot,
    UserIsRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for EnableError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => EnableError::AccountInactive,
            "fatal_error" => EnableError::FatalError,
            "invalid_arg_name" => EnableError::InvalidArgName,
            "invalid_array_arg" => EnableError::InvalidArrayArg,
            "invalid_auth" => EnableError::InvalidAuth,
            "invalid_charset" => EnableError::InvalidCharset,
            "invalid_form_data" => EnableError::InvalidFormData,
            "invalid_json" => EnableError::InvalidJson,
            "invalid_post_type" => EnableError::InvalidPostType,
            "json_not_object" => EnableError::JsonNotObject,
            "missing_charset" => EnableError::MissingCharset,
            "missing_post_type" => EnableError::MissingPostType,
            "no_permission" => EnableError::NoPermission,
            "not_authed" => EnableError::NotAuthed,
            "org_login_required" => EnableError::OrgLoginRequired,
            "request_timeout" => EnableError::RequestTimeout,
            "superfluous_charset" => EnableError::SuperfluousCharset,
            "team_added_to_org" => EnableError::TeamAddedToOrg,
            "token_revoked" => EnableError::TokenRevoked,
            "upgrade_require" => EnableError::UpgradeRequire,
            "user_is_bot" => EnableError::UserIsBot,
            "user_is_restricted" => EnableError::UserIsRestricted,
            _ => EnableError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EnableError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EnableError::AccountInactive => write!(f, "Server returned error account_inactive"),
            EnableError::FatalError => write!(f, "Server returned error fatal_error"),
            EnableError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            EnableError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            EnableError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            EnableError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            EnableError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            EnableError::InvalidJson => write!(f, "Server returned error invalid_json"),
            EnableError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            EnableError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            EnableError::MissingCharset => write!(f, "Server returned error missing_charset"),
            EnableError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            EnableError::NoPermission => write!(f, "Server returned error no_permission"),
            EnableError::NotAuthed => write!(f, "Server returned error not_authed"),
            EnableError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            EnableError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            EnableError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
            }
            EnableError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            EnableError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            EnableError::UpgradeRequire => write!(f, "Server returned error upgrade_require"),
            EnableError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            EnableError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            EnableError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            EnableError::Unknown(ref s) => write!(f, "{}", s),
            EnableError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for EnableError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            EnableError::MalformedResponse(_, ref e) => Some(e),
            EnableError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest {
    /// Include the number of users in each User Group.
    pub include_count: Option<bool>,
    /// Include disabled User Groups.
    pub include_disabled: Option<bool>,
    /// Include the list of users for each User Group.
    pub include_users: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListPrefsInner {
    pub channels: Vec<String>,
    pub groups: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListUsergroupsInner {
    pub auto_provision: bool,
    pub auto_type: Vec<String>,
    pub channel_count: Option<u64>,
    pub created_by: String,
    pub date_create: u64,
    pub date_delete: u64,
    pub date_update: u64,
    pub deleted_by: Vec<String>,
    pub description: String,
    pub enterprise_subteam_id: String,
    pub handle: String,
    pub id: String,
    pub is_external: bool,
    pub is_subteam: bool,
    pub is_usergroup: bool,
    pub name: String,
    pub prefs: ListPrefsInner,
    pub team_id: String,
    pub updated_by: String,
    pub user_count: Option<u64>,
    pub users: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroups: Vec<ListUsergroupsInner>,
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
    MissingCharset,
    MissingPostType,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    SuperfluousCharset,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequire,
    UserIsBot,
    UserIsRestricted,
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
            "missing_charset" => ListError::MissingCharset,
            "missing_post_type" => ListError::MissingPostType,
            "no_permission" => ListError::NoPermission,
            "not_authed" => ListError::NotAuthed,
            "org_login_required" => ListError::OrgLoginRequired,
            "request_timeout" => ListError::RequestTimeout,
            "superfluous_charset" => ListError::SuperfluousCharset,
            "team_added_to_org" => ListError::TeamAddedToOrg,
            "token_revoked" => ListError::TokenRevoked,
            "upgrade_require" => ListError::UpgradeRequire,
            "user_is_bot" => ListError::UserIsBot,
            "user_is_restricted" => ListError::UserIsRestricted,
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
            ListError::MissingCharset => write!(f, "Server returned error missing_charset"),
            ListError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ListError::NoPermission => write!(f, "Server returned error no_permission"),
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
            ListError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            ListError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ListError::SuperfluousCharset => write!(f, "Server returned error superfluous_charset"),
            ListError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ListError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ListError::UpgradeRequire => write!(f, "Server returned error upgrade_require"),
            ListError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            ListError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
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
pub struct UpdateRequest<'a> {
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<Cow<'a, str>>,
    /// A short description of the User Group.
    pub description: Option<Cow<'a, str>>,
    /// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<Cow<'a, str>>,
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
    /// A name for the User Group. Must be unique among User Groups.
    pub name: Option<Cow<'a, str>>,
    /// The encoded ID of the User Group to update.
    pub usergroup: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdatePrefsInner {
    pub channels: Vec<String>,
    pub groups: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateUsergroupInner {
    pub auto_provision: bool,
    pub auto_type: Vec<String>,
    pub channel_count: Option<u64>,
    pub created_by: String,
    pub date_create: u64,
    pub date_delete: u64,
    pub date_update: u64,
    pub deleted_by: Vec<String>,
    pub description: String,
    pub enterprise_subteam_id: String,
    pub handle: String,
    pub id: String,
    pub is_external: bool,
    pub is_subteam: bool,
    pub is_usergroup: bool,
    pub name: String,
    pub prefs: UpdatePrefsInner,
    pub team_id: String,
    pub updated_by: String,
    pub user_count: Option<u64>,
    pub users: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub usergroup: UpdateUsergroupInner,
}

impl<E: Error> Into<Result<UpdateResponse, UpdateError<E>>> for UpdateResponse {
    fn into(self) -> Result<UpdateResponse, UpdateError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum UpdateError<E: Error> {
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
    MissingCharset,
    MissingPostType,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    PermissionDenied,
    RequestTimeout,
    SuperfluousCharset,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequire,
    UserIsBot,
    UserIsRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for UpdateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => UpdateError::AccountInactive,
            "fatal_error" => UpdateError::FatalError,
            "invalid_arg_name" => UpdateError::InvalidArgName,
            "invalid_array_arg" => UpdateError::InvalidArrayArg,
            "invalid_auth" => UpdateError::InvalidAuth,
            "invalid_charset" => UpdateError::InvalidCharset,
            "invalid_form_data" => UpdateError::InvalidFormData,
            "invalid_json" => UpdateError::InvalidJson,
            "invalid_post_type" => UpdateError::InvalidPostType,
            "json_not_object" => UpdateError::JsonNotObject,
            "missing_charset" => UpdateError::MissingCharset,
            "missing_post_type" => UpdateError::MissingPostType,
            "no_permission" => UpdateError::NoPermission,
            "not_authed" => UpdateError::NotAuthed,
            "org_login_required" => UpdateError::OrgLoginRequired,
            "permission_denied" => UpdateError::PermissionDenied,
            "request_timeout" => UpdateError::RequestTimeout,
            "superfluous_charset" => UpdateError::SuperfluousCharset,
            "team_added_to_org" => UpdateError::TeamAddedToOrg,
            "token_revoked" => UpdateError::TokenRevoked,
            "upgrade_require" => UpdateError::UpgradeRequire,
            "user_is_bot" => UpdateError::UserIsBot,
            "user_is_restricted" => UpdateError::UserIsRestricted,
            _ => UpdateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UpdateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UpdateError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UpdateError::FatalError => write!(f, "Server returned error fatal_error"),
            UpdateError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UpdateError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UpdateError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UpdateError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UpdateError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UpdateError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UpdateError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UpdateError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UpdateError::MissingCharset => write!(f, "Server returned error missing_charset"),
            UpdateError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UpdateError::NoPermission => write!(f, "Server returned error no_permission"),
            UpdateError::NotAuthed => write!(f, "Server returned error not_authed"),
            UpdateError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            UpdateError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            UpdateError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UpdateError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
            }
            UpdateError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            UpdateError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            UpdateError::UpgradeRequire => write!(f, "Server returned error upgrade_require"),
            UpdateError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            UpdateError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            UpdateError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            UpdateError::Unknown(ref s) => write!(f, "{}", s),
            UpdateError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for UpdateError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UpdateError::MalformedResponse(_, ref e) => Some(e),
            UpdateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
