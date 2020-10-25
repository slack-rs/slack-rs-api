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
pub struct GetRequest {
    /// Include labels for each ID in custom profile fields
    pub include_labels: Option<bool>,
    /// User to retrieve profile info for
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetProfileInner {
    pub always_active: Option<bool>,
    pub api_app_id: Option<String>,
    pub avatar_hash: String,
    pub bot_id: Option<String>,
    pub display_name: String,
    pub display_name_normalized: String,
    pub email: Option<String>,
    pub fields: Vec<serde_json::Value>,
    pub first_name: Option<String>,
    pub guest_expiration_ts: Option<u64>,
    pub guest_invited_by: Option<String>,
    pub image_1024: Option<String>,
    pub image_192: Option<String>,
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_512: Option<String>,
    pub image_72: Option<String>,
    pub image_original: Option<String>,
    pub is_app_user: Option<bool>,
    pub is_custom_image: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub last_avatar_image_hash: Option<String>,
    pub last_name: Option<String>,
    pub memberships_count: Option<u64>,
    pub name: Option<String>,
    pub phone: String,
    pub pronouns: Option<String>,
    pub real_name: String,
    pub real_name_normalized: String,
    pub skype: String,
    pub status_default_emoji: Option<String>,
    pub status_default_text: Option<String>,
    pub status_default_text_canonical: Option<String>,
    pub status_emoji: String,
    pub status_expiration: Option<u64>,
    pub status_text: String,
    pub status_text_canonical: Option<String>,
    pub team: Option<String>,
    pub title: String,
    pub updated: Option<u64>,
    pub user_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub profile: GetProfileInner,
}

impl<E: Error> Into<Result<GetResponse, GetError<E>>> for GetResponse {
    fn into(self) -> Result<GetResponse, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum GetError<E: Error> {
    UserNotFound,
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

impl<'a, E: Error> From<&'a str> for GetError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "user_not_found" => GetError::UserNotFound,
            "not_authed" => GetError::NotAuthed,
            "invalid_auth" => GetError::InvalidAuth,
            "account_inactive" => GetError::AccountInactive,
            "token_revoked" => GetError::TokenRevoked,
            "no_permission" => GetError::NoPermission,
            "org_login_required" => GetError::OrgLoginRequired,
            "user_is_bot" => GetError::UserIsBot,
            "invalid_arg_name" => GetError::InvalidArgName,
            "invalid_array_arg" => GetError::InvalidArrayArg,
            "invalid_charset" => GetError::InvalidCharset,
            "invalid_form_data" => GetError::InvalidFormData,
            "invalid_post_type" => GetError::InvalidPostType,
            "missing_post_type" => GetError::MissingPostType,
            "team_added_to_org" => GetError::TeamAddedToOrg,
            "invalid_json" => GetError::InvalidJson,
            "json_not_object" => GetError::JsonNotObject,
            "request_timeout" => GetError::RequestTimeout,
            "upgrade_required" => GetError::UpgradeRequired,
            "fatal_error" => GetError::FatalError,
            _ => GetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetError::UserNotFound => write!(f, "Server returned error user_not_found"),
            GetError::NotAuthed => write!(f, "Server returned error not_authed"),
            GetError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            GetError::AccountInactive => write!(f, "Server returned error account_inactive"),
            GetError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            GetError::NoPermission => write!(f, "Server returned error no_permission"),
            GetError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            GetError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            GetError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            GetError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            GetError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            GetError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            GetError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            GetError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            GetError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            GetError::InvalidJson => write!(f, "Server returned error invalid_json"),
            GetError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            GetError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            GetError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            GetError::FatalError => write!(f, "Server returned error fatal_error"),
            GetError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            GetError::Unknown(ref s) => write!(f, "{}", s),
            GetError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for GetError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            GetError::MalformedResponse(_, ref e) => Some(e),
            GetError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetRequest {
    /// Name of a single key to set. Usable only if `profile` is not passed.
    pub name: Option<String>,
    /// Collection of key:value pairs presented as a URL-encoded JSON hash. At most 50 fields may be set. Each field name is limited to 255 characters.
    pub profile: Option<String>,
    /// ID of user to change. This argument may only be specified by team admins on paid teams.
    pub user: Option<String>,
    /// Value to set a single key to. Usable only if `profile` is not passed.
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetProfileInner {
    pub always_active: Option<bool>,
    pub api_app_id: Option<String>,
    pub avatar_hash: String,
    pub bot_id: Option<String>,
    pub display_name: String,
    pub display_name_normalized: String,
    pub email: Option<String>,
    pub fields: Vec<serde_json::Value>,
    pub first_name: Option<String>,
    pub guest_expiration_ts: Option<u64>,
    pub guest_invited_by: Option<String>,
    pub image_1024: Option<String>,
    pub image_192: Option<String>,
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_512: Option<String>,
    pub image_72: Option<String>,
    pub image_original: Option<String>,
    pub is_app_user: Option<bool>,
    pub is_custom_image: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub last_avatar_image_hash: Option<String>,
    pub last_name: Option<String>,
    pub memberships_count: Option<u64>,
    pub name: Option<String>,
    pub phone: String,
    pub pronouns: Option<String>,
    pub real_name: String,
    pub real_name_normalized: String,
    pub skype: String,
    pub status_default_emoji: Option<String>,
    pub status_default_text: Option<String>,
    pub status_default_text_canonical: Option<String>,
    pub status_emoji: String,
    pub status_expiration: Option<u64>,
    pub status_text: String,
    pub status_text_canonical: Option<String>,
    pub team: Option<String>,
    pub title: String,
    pub updated: Option<u64>,
    pub user_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetResponse {
    pub callstack: Option<String>,
    pub email_pending: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub profile: SetProfileInner,
    pub username: String,
}

impl<E: Error> Into<Result<SetResponse, SetError<E>>> for SetResponse {
    fn into(self) -> Result<SetResponse, SetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum SetError<E: Error> {
    ReservedName,
    InvalidProfile,
    ProfileSetFailed,
    NotAdmin,
    NotAppAdmin,
    CannotUpdateAdminUser,
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

impl<'a, E: Error> From<&'a str> for SetError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "reserved_name" => SetError::ReservedName,
            "invalid_profile" => SetError::InvalidProfile,
            "profile_set_failed" => SetError::ProfileSetFailed,
            "not_admin" => SetError::NotAdmin,
            "not_app_admin" => SetError::NotAppAdmin,
            "cannot_update_admin_user" => SetError::CannotUpdateAdminUser,
            "not_authed" => SetError::NotAuthed,
            "invalid_auth" => SetError::InvalidAuth,
            "account_inactive" => SetError::AccountInactive,
            "token_revoked" => SetError::TokenRevoked,
            "no_permission" => SetError::NoPermission,
            "org_login_required" => SetError::OrgLoginRequired,
            "user_is_bot" => SetError::UserIsBot,
            "invalid_arg_name" => SetError::InvalidArgName,
            "invalid_array_arg" => SetError::InvalidArrayArg,
            "invalid_charset" => SetError::InvalidCharset,
            "invalid_form_data" => SetError::InvalidFormData,
            "invalid_post_type" => SetError::InvalidPostType,
            "missing_post_type" => SetError::MissingPostType,
            "team_added_to_org" => SetError::TeamAddedToOrg,
            "invalid_json" => SetError::InvalidJson,
            "json_not_object" => SetError::JsonNotObject,
            "request_timeout" => SetError::RequestTimeout,
            "upgrade_required" => SetError::UpgradeRequired,
            "fatal_error" => SetError::FatalError,
            _ => SetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetError::ReservedName => write!(f, "Server returned error reserved_name"),
            SetError::InvalidProfile => write!(f, "Server returned error invalid_profile"),
            SetError::ProfileSetFailed => write!(f, "Server returned error profile_set_failed"),
            SetError::NotAdmin => write!(f, "Server returned error not_admin"),
            SetError::NotAppAdmin => write!(f, "Server returned error not_app_admin"),
            SetError::CannotUpdateAdminUser => {
                write!(f, "Server returned error cannot_update_admin_user")
            }
            SetError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            SetError::NoPermission => write!(f, "Server returned error no_permission"),
            SetError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            SetError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            SetError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            SetError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            SetError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            SetError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            SetError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            SetError::FatalError => write!(f, "Server returned error fatal_error"),
            SetError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetError::Unknown(ref s) => write!(f, "{}", s),
            SetError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetError::MalformedResponse(_, ref e) => Some(e),
            SetError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
