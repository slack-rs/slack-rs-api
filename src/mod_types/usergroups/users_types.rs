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
pub struct ListRequest<'a> {
    /// Allow results that involve disabled User Groups.
    pub include_disabled: Option<bool>,
    /// The encoded ID of the User Group to update.
    pub usergroup: Cow<'a, str>,
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
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
    /// The encoded ID of the User Group to update.
    pub usergroup: Cow<'a, str>,
    /// A comma separated string of encoded user IDs that represent the entire list of users for the User Group.
    pub users: Cow<'a, str>,
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
