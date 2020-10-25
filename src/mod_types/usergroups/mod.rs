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

pub mod users_types;

use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct EnableRequest {
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
    /// The encoded ID of the User Group to enable.
    pub usergroup: String,
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
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    UserIsBot,
    UserIsRestricted,
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
    UpgradeRequire,
    FatalError,
    MissingCharset,
    SuperfluousCharset,
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
            "not_authed" => EnableError::NotAuthed,
            "invalid_auth" => EnableError::InvalidAuth,
            "account_inactive" => EnableError::AccountInactive,
            "token_revoked" => EnableError::TokenRevoked,
            "no_permission" => EnableError::NoPermission,
            "org_login_required" => EnableError::OrgLoginRequired,
            "user_is_bot" => EnableError::UserIsBot,
            "user_is_restricted" => EnableError::UserIsRestricted,
            "invalid_arg_name" => EnableError::InvalidArgName,
            "invalid_array_arg" => EnableError::InvalidArrayArg,
            "invalid_charset" => EnableError::InvalidCharset,
            "invalid_form_data" => EnableError::InvalidFormData,
            "invalid_post_type" => EnableError::InvalidPostType,
            "missing_post_type" => EnableError::MissingPostType,
            "team_added_to_org" => EnableError::TeamAddedToOrg,
            "invalid_json" => EnableError::InvalidJson,
            "json_not_object" => EnableError::JsonNotObject,
            "request_timeout" => EnableError::RequestTimeout,
            "upgrade_require" => EnableError::UpgradeRequire,
            "fatal_error" => EnableError::FatalError,
            "missing_charset" => EnableError::MissingCharset,
            "superfluous_charset" => EnableError::SuperfluousCharset,
            _ => EnableError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EnableError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EnableError::NotAuthed => write!(f, "Server returned error not_authed"),
            EnableError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            EnableError::AccountInactive => write!(f, "Server returned error account_inactive"),
            EnableError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            EnableError::NoPermission => write!(f, "Server returned error no_permission"),
            EnableError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            EnableError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            EnableError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            EnableError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            EnableError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            EnableError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            EnableError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            EnableError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            EnableError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            EnableError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            EnableError::InvalidJson => write!(f, "Server returned error invalid_json"),
            EnableError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            EnableError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            EnableError::UpgradeRequire => write!(f, "Server returned error upgrade_require"),
            EnableError::FatalError => write!(f, "Server returned error fatal_error"),
            EnableError::MissingCharset => write!(f, "Server returned error missing_charset"),
            EnableError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
            }
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
    /// Include the list of users for each User Group.
    pub include_users: Option<bool>,
    /// Include the number of users in each User Group.
    pub include_count: Option<bool>,
    /// Include disabled User Groups.
    pub include_disabled: Option<bool>,
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
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    UserIsBot,
    UserIsRestricted,
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
    UpgradeRequire,
    FatalError,
    MissingCharset,
    SuperfluousCharset,
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
            "user_is_restricted" => ListError::UserIsRestricted,
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
            "upgrade_require" => ListError::UpgradeRequire,
            "fatal_error" => ListError::FatalError,
            "missing_charset" => ListError::MissingCharset,
            "superfluous_charset" => ListError::SuperfluousCharset,
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
            ListError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
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
            ListError::UpgradeRequire => write!(f, "Server returned error upgrade_require"),
            ListError::FatalError => write!(f, "Server returned error fatal_error"),
            ListError::MissingCharset => write!(f, "Server returned error missing_charset"),
            ListError::SuperfluousCharset => write!(f, "Server returned error superfluous_charset"),
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
pub struct UpdateRequest {
    /// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<String>,
    /// A short description of the User Group.
    pub description: Option<String>,
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<String>,
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
    /// The encoded ID of the User Group to update.
    pub usergroup: String,
    /// A name for the User Group. Must be unique among User Groups.
    pub name: Option<String>,
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
    PermissionDenied,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    UserIsBot,
    UserIsRestricted,
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
    UpgradeRequire,
    FatalError,
    MissingCharset,
    SuperfluousCharset,
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
            "permission_denied" => UpdateError::PermissionDenied,
            "not_authed" => UpdateError::NotAuthed,
            "invalid_auth" => UpdateError::InvalidAuth,
            "account_inactive" => UpdateError::AccountInactive,
            "token_revoked" => UpdateError::TokenRevoked,
            "no_permission" => UpdateError::NoPermission,
            "org_login_required" => UpdateError::OrgLoginRequired,
            "user_is_bot" => UpdateError::UserIsBot,
            "user_is_restricted" => UpdateError::UserIsRestricted,
            "invalid_arg_name" => UpdateError::InvalidArgName,
            "invalid_array_arg" => UpdateError::InvalidArrayArg,
            "invalid_charset" => UpdateError::InvalidCharset,
            "invalid_form_data" => UpdateError::InvalidFormData,
            "invalid_post_type" => UpdateError::InvalidPostType,
            "missing_post_type" => UpdateError::MissingPostType,
            "team_added_to_org" => UpdateError::TeamAddedToOrg,
            "invalid_json" => UpdateError::InvalidJson,
            "json_not_object" => UpdateError::JsonNotObject,
            "request_timeout" => UpdateError::RequestTimeout,
            "upgrade_require" => UpdateError::UpgradeRequire,
            "fatal_error" => UpdateError::FatalError,
            "missing_charset" => UpdateError::MissingCharset,
            "superfluous_charset" => UpdateError::SuperfluousCharset,
            _ => UpdateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UpdateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UpdateError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            UpdateError::NotAuthed => write!(f, "Server returned error not_authed"),
            UpdateError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UpdateError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UpdateError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            UpdateError::NoPermission => write!(f, "Server returned error no_permission"),
            UpdateError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            UpdateError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            UpdateError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            UpdateError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UpdateError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UpdateError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UpdateError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UpdateError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UpdateError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UpdateError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            UpdateError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UpdateError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UpdateError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UpdateError::UpgradeRequire => write!(f, "Server returned error upgrade_require"),
            UpdateError::FatalError => write!(f, "Server returned error fatal_error"),
            UpdateError::MissingCharset => write!(f, "Server returned error missing_charset"),
            UpdateError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
            }
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

#[derive(Clone, Default, Debug)]
pub struct CreateRequest {
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<String>,
    /// A short description of the User Group.
    pub description: Option<String>,
    /// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<String>,
    /// Include the number of users in each User Group.
    pub include_count: Option<bool>,
    /// A name for the User Group. Must be unique among User Groups.
    pub name: String,
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
    PermissionDenied,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    UserIsBot,
    UserIsRestricted,
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

impl<'a, E: Error> From<&'a str> for CreateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "permission_denied" => CreateError::PermissionDenied,
            "not_authed" => CreateError::NotAuthed,
            "invalid_auth" => CreateError::InvalidAuth,
            "account_inactive" => CreateError::AccountInactive,
            "token_revoked" => CreateError::TokenRevoked,
            "no_permission" => CreateError::NoPermission,
            "org_login_required" => CreateError::OrgLoginRequired,
            "user_is_bot" => CreateError::UserIsBot,
            "user_is_restricted" => CreateError::UserIsRestricted,
            "invalid_arg_name" => CreateError::InvalidArgName,
            "invalid_array_arg" => CreateError::InvalidArrayArg,
            "invalid_charset" => CreateError::InvalidCharset,
            "invalid_form_data" => CreateError::InvalidFormData,
            "invalid_post_type" => CreateError::InvalidPostType,
            "missing_post_type" => CreateError::MissingPostType,
            "team_added_to_org" => CreateError::TeamAddedToOrg,
            "invalid_json" => CreateError::InvalidJson,
            "json_not_object" => CreateError::JsonNotObject,
            "request_timeout" => CreateError::RequestTimeout,
            "upgrade_required" => CreateError::UpgradeRequired,
            "fatal_error" => CreateError::FatalError,
            _ => CreateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CreateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CreateError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            CreateError::NotAuthed => write!(f, "Server returned error not_authed"),
            CreateError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            CreateError::AccountInactive => write!(f, "Server returned error account_inactive"),
            CreateError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            CreateError::NoPermission => write!(f, "Server returned error no_permission"),
            CreateError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            CreateError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            CreateError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            CreateError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            CreateError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            CreateError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            CreateError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            CreateError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            CreateError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            CreateError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            CreateError::InvalidJson => write!(f, "Server returned error invalid_json"),
            CreateError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            CreateError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            CreateError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            CreateError::FatalError => write!(f, "Server returned error fatal_error"),
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
pub struct DisableRequest {
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
    /// The encoded ID of the User Group to disable.
    pub usergroup: String,
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
    PermissionDenied,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
    UserIsBot,
    UserIsRestricted,
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

impl<'a, E: Error> From<&'a str> for DisableError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "permission_denied" => DisableError::PermissionDenied,
            "not_authed" => DisableError::NotAuthed,
            "invalid_auth" => DisableError::InvalidAuth,
            "account_inactive" => DisableError::AccountInactive,
            "token_revoked" => DisableError::TokenRevoked,
            "no_permission" => DisableError::NoPermission,
            "org_login_required" => DisableError::OrgLoginRequired,
            "user_is_bot" => DisableError::UserIsBot,
            "user_is_restricted" => DisableError::UserIsRestricted,
            "invalid_arg_name" => DisableError::InvalidArgName,
            "invalid_array_arg" => DisableError::InvalidArrayArg,
            "invalid_charset" => DisableError::InvalidCharset,
            "invalid_form_data" => DisableError::InvalidFormData,
            "invalid_post_type" => DisableError::InvalidPostType,
            "missing_post_type" => DisableError::MissingPostType,
            "team_added_to_org" => DisableError::TeamAddedToOrg,
            "invalid_json" => DisableError::InvalidJson,
            "json_not_object" => DisableError::JsonNotObject,
            "request_timeout" => DisableError::RequestTimeout,
            "upgrade_required" => DisableError::UpgradeRequired,
            "fatal_error" => DisableError::FatalError,
            _ => DisableError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DisableError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DisableError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            DisableError::NotAuthed => write!(f, "Server returned error not_authed"),
            DisableError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            DisableError::AccountInactive => write!(f, "Server returned error account_inactive"),
            DisableError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            DisableError::NoPermission => write!(f, "Server returned error no_permission"),
            DisableError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            DisableError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            DisableError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            DisableError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            DisableError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            DisableError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            DisableError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            DisableError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            DisableError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            DisableError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            DisableError::InvalidJson => write!(f, "Server returned error invalid_json"),
            DisableError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            DisableError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            DisableError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            DisableError::FatalError => write!(f, "Server returned error fatal_error"),
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
