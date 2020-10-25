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

pub mod event;
pub mod permissions;

use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct UninstallRequest {
    /// Issued when you created your application.
    pub client_id: Option<String>,
    /// Issued when you created your application.
    pub client_secret: Option<String>,
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
    InvalidClientId,
    BadClientSecret,
    ClientIdTokenMismatch,
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

impl<'a, E: Error> From<&'a str> for UninstallError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "invalid_client_id" => UninstallError::InvalidClientId,
            "bad_client_secret" => UninstallError::BadClientSecret,
            "client_id_token_mismatch" => UninstallError::ClientIdTokenMismatch,
            "not_authed" => UninstallError::NotAuthed,
            "invalid_auth" => UninstallError::InvalidAuth,
            "account_inactive" => UninstallError::AccountInactive,
            "token_revoked" => UninstallError::TokenRevoked,
            "no_permission" => UninstallError::NoPermission,
            "org_login_required" => UninstallError::OrgLoginRequired,
            "user_is_bot" => UninstallError::UserIsBot,
            "invalid_arg_name" => UninstallError::InvalidArgName,
            "invalid_array_arg" => UninstallError::InvalidArrayArg,
            "invalid_charset" => UninstallError::InvalidCharset,
            "invalid_form_data" => UninstallError::InvalidFormData,
            "invalid_post_type" => UninstallError::InvalidPostType,
            "missing_post_type" => UninstallError::MissingPostType,
            "team_added_to_org" => UninstallError::TeamAddedToOrg,
            "invalid_json" => UninstallError::InvalidJson,
            "json_not_object" => UninstallError::JsonNotObject,
            "request_timeout" => UninstallError::RequestTimeout,
            "upgrade_required" => UninstallError::UpgradeRequired,
            "fatal_error" => UninstallError::FatalError,
            _ => UninstallError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UninstallError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UninstallError::InvalidClientId => write!(f, "Server returned error invalid_client_id"),
            UninstallError::BadClientSecret => write!(f, "Server returned error bad_client_secret"),
            UninstallError::ClientIdTokenMismatch => {
                write!(f, "Server returned error client_id_token_mismatch")
            }
            UninstallError::NotAuthed => write!(f, "Server returned error not_authed"),
            UninstallError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UninstallError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UninstallError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            UninstallError::NoPermission => write!(f, "Server returned error no_permission"),
            UninstallError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            UninstallError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            UninstallError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UninstallError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UninstallError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UninstallError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UninstallError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UninstallError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UninstallError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            UninstallError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UninstallError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UninstallError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UninstallError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            UninstallError::FatalError => write!(f, "Server returned error fatal_error"),
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
