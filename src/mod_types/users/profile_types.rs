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
pub struct GetRequest<'a> {
    /// Include labels for each ID in custom profile fields
    pub include_labels: Option<bool>,
    /// User to retrieve profile info for
    pub user: Option<Cow<'a, str>>,
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
    UserNotFound,
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
            "account_inactive" => GetError::AccountInactive,
            "fatal_error" => GetError::FatalError,
            "invalid_arg_name" => GetError::InvalidArgName,
            "invalid_array_arg" => GetError::InvalidArrayArg,
            "invalid_auth" => GetError::InvalidAuth,
            "invalid_charset" => GetError::InvalidCharset,
            "invalid_form_data" => GetError::InvalidFormData,
            "invalid_json" => GetError::InvalidJson,
            "invalid_post_type" => GetError::InvalidPostType,
            "json_not_object" => GetError::JsonNotObject,
            "missing_post_type" => GetError::MissingPostType,
            "no_permission" => GetError::NoPermission,
            "not_authed" => GetError::NotAuthed,
            "org_login_required" => GetError::OrgLoginRequired,
            "request_timeout" => GetError::RequestTimeout,
            "team_added_to_org" => GetError::TeamAddedToOrg,
            "token_revoked" => GetError::TokenRevoked,
            "upgrade_required" => GetError::UpgradeRequired,
            "user_is_bot" => GetError::UserIsBot,
            "user_not_found" => GetError::UserNotFound,
            _ => GetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetError::AccountInactive => write!(f, "Server returned error account_inactive"),
            GetError::FatalError => write!(f, "Server returned error fatal_error"),
            GetError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            GetError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            GetError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            GetError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            GetError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            GetError::InvalidJson => write!(f, "Server returned error invalid_json"),
            GetError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            GetError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            GetError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            GetError::NoPermission => write!(f, "Server returned error no_permission"),
            GetError::NotAuthed => write!(f, "Server returned error not_authed"),
            GetError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            GetError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            GetError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            GetError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            GetError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            GetError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            GetError::UserNotFound => write!(f, "Server returned error user_not_found"),
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
pub struct SetRequest<'a> {
    /// Name of a single key to set. Usable only if `profile` is not passed.
    pub name: Option<Cow<'a, str>>,
    /// Collection of key:value pairs presented as a URL-encoded JSON hash. At most 50 fields may be set. Each field name is limited to 255 characters.
    pub profile: Option<Cow<'a, str>>,
    /// ID of user to change. This argument may only be specified by team admins on paid teams.
    pub user: Option<Cow<'a, str>>,
    /// Value to set a single key to. Usable only if `profile` is not passed.
    pub value: Option<Cow<'a, str>>,
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
    AccountInactive,
    CannotUpdateAdminUser,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    InvalidProfile,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAdmin,
    NotAppAdmin,
    NotAuthed,
    OrgLoginRequired,
    ProfileSetFailed,
    RequestTimeout,
    ReservedName,
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

impl<'a, E: Error> From<&'a str> for SetError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => SetError::AccountInactive,
            "cannot_update_admin_user" => SetError::CannotUpdateAdminUser,
            "fatal_error" => SetError::FatalError,
            "invalid_arg_name" => SetError::InvalidArgName,
            "invalid_array_arg" => SetError::InvalidArrayArg,
            "invalid_auth" => SetError::InvalidAuth,
            "invalid_charset" => SetError::InvalidCharset,
            "invalid_form_data" => SetError::InvalidFormData,
            "invalid_json" => SetError::InvalidJson,
            "invalid_post_type" => SetError::InvalidPostType,
            "invalid_profile" => SetError::InvalidProfile,
            "json_not_object" => SetError::JsonNotObject,
            "missing_post_type" => SetError::MissingPostType,
            "no_permission" => SetError::NoPermission,
            "not_admin" => SetError::NotAdmin,
            "not_app_admin" => SetError::NotAppAdmin,
            "not_authed" => SetError::NotAuthed,
            "org_login_required" => SetError::OrgLoginRequired,
            "profile_set_failed" => SetError::ProfileSetFailed,
            "request_timeout" => SetError::RequestTimeout,
            "reserved_name" => SetError::ReservedName,
            "team_added_to_org" => SetError::TeamAddedToOrg,
            "token_revoked" => SetError::TokenRevoked,
            "upgrade_required" => SetError::UpgradeRequired,
            "user_is_bot" => SetError::UserIsBot,
            _ => SetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetError::CannotUpdateAdminUser => {
                write!(f, "Server returned error cannot_update_admin_user")
            }
            SetError::FatalError => write!(f, "Server returned error fatal_error"),
            SetError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            SetError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            SetError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            SetError::InvalidProfile => write!(f, "Server returned error invalid_profile"),
            SetError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            SetError::NoPermission => write!(f, "Server returned error no_permission"),
            SetError::NotAdmin => write!(f, "Server returned error not_admin"),
            SetError::NotAppAdmin => write!(f, "Server returned error not_app_admin"),
            SetError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            SetError::ProfileSetFailed => write!(f, "Server returned error profile_set_failed"),
            SetError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetError::ReservedName => write!(f, "Server returned error reserved_name"),
            SetError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            SetError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            SetError::UserIsBot => write!(f, "Server returned error user_is_bot"),
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
