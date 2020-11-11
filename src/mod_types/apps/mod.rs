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

pub mod event;
pub mod permissions;

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct UninstallRequest<'a> {
    /// Issued when you created your application.
    pub client_id: Option<Cow<'a, str>>,
    /// Issued when you created your application.
    pub client_secret: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UninstallResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<UninstallResponse, UninstallError<E>>> for UninstallResponse {
    fn into(self) -> Result<UninstallResponse, UninstallError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum UninstallError<E: Error> {
    AccountInactive,
    BadClientSecret,
    ClientIdTokenMismatch,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidClientId,
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

impl<'a, E: Error> From<&'a str> for UninstallError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => UninstallError::AccountInactive,
            "bad_client_secret" => UninstallError::BadClientSecret,
            "client_id_token_mismatch" => UninstallError::ClientIdTokenMismatch,
            "fatal_error" => UninstallError::FatalError,
            "invalid_arg_name" => UninstallError::InvalidArgName,
            "invalid_array_arg" => UninstallError::InvalidArrayArg,
            "invalid_auth" => UninstallError::InvalidAuth,
            "invalid_charset" => UninstallError::InvalidCharset,
            "invalid_client_id" => UninstallError::InvalidClientId,
            "invalid_form_data" => UninstallError::InvalidFormData,
            "invalid_json" => UninstallError::InvalidJson,
            "invalid_post_type" => UninstallError::InvalidPostType,
            "json_not_object" => UninstallError::JsonNotObject,
            "missing_post_type" => UninstallError::MissingPostType,
            "no_permission" => UninstallError::NoPermission,
            "not_authed" => UninstallError::NotAuthed,
            "org_login_required" => UninstallError::OrgLoginRequired,
            "request_timeout" => UninstallError::RequestTimeout,
            "team_added_to_org" => UninstallError::TeamAddedToOrg,
            "token_revoked" => UninstallError::TokenRevoked,
            "upgrade_required" => UninstallError::UpgradeRequired,
            "user_is_bot" => UninstallError::UserIsBot,
            _ => UninstallError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UninstallError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UninstallError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UninstallError::BadClientSecret => write!(f, "Server returned error bad_client_secret"),
            UninstallError::ClientIdTokenMismatch => {
                write!(f, "Server returned error client_id_token_mismatch")
            }
            UninstallError::FatalError => write!(f, "Server returned error fatal_error"),
            UninstallError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UninstallError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UninstallError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UninstallError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UninstallError::InvalidClientId => write!(f, "Server returned error invalid_client_id"),
            UninstallError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UninstallError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UninstallError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UninstallError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UninstallError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UninstallError::NoPermission => write!(f, "Server returned error no_permission"),
            UninstallError::NotAuthed => write!(f, "Server returned error not_authed"),
            UninstallError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            UninstallError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UninstallError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            UninstallError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            UninstallError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            UninstallError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            UninstallError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            UninstallError::Unknown(ref s) => write!(f, "{}", s),
            UninstallError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for UninstallError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UninstallError::MalformedResponse(_, ref e) => Some(e),
            UninstallError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
