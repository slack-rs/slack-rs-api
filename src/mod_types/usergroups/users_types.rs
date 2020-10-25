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
    /// Allow results that involve disabled User Groups.
    pub include_disabled: Option<bool>,
    /// The encoded ID of the User Group to update.
    pub usergroup: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub users: Vec<String>,
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
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
    /// The encoded ID of the User Group to update.
    pub usergroup: String,
    /// A comma separated string of encoded user IDs that represent the entire list of users for the User Group.
    pub users: String,
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
