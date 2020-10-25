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
pub struct ExchangeRequest {
    /// A comma-separated list of user ids, up to 400 per request
    pub users: String,
    /// Specify team_id starts with `T` in case of Org Token
    pub team_id: Option<String>,
    /// Specify `true` to convert `W` global user IDs to workspace-specific `U` IDs. Defaults to `false`.
    pub to_old: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExchangeResponse {
    pub callstack: Option<String>,
    pub enterprise_id: String,
    error: Option<String>,
    pub invalid_user_ids: Option<Vec<String>>,
    #[serde(default)]
    ok: bool,
    pub team_id: String,
    pub user_id_map: Option<serde_json::Value>,
}

impl<E: Error> Into<Result<ExchangeResponse, ExchangeError<E>>> for ExchangeResponse {
    fn into(self) -> Result<ExchangeResponse, ExchangeError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum ExchangeError<E: Error> {
    NotEnterpriseTeam,
    TooManyUsers,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    TokenRevoked,
    NoPermission,
    OrgLoginRequired,
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

impl<'a, E: Error> From<&'a str> for ExchangeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_enterprise_team" => ExchangeError::NotEnterpriseTeam,
            "too_many_users" => ExchangeError::TooManyUsers,
            "not_authed" => ExchangeError::NotAuthed,
            "invalid_auth" => ExchangeError::InvalidAuth,
            "account_inactive" => ExchangeError::AccountInactive,
            "token_revoked" => ExchangeError::TokenRevoked,
            "no_permission" => ExchangeError::NoPermission,
            "org_login_required" => ExchangeError::OrgLoginRequired,
            "invalid_arg_name" => ExchangeError::InvalidArgName,
            "invalid_array_arg" => ExchangeError::InvalidArrayArg,
            "invalid_charset" => ExchangeError::InvalidCharset,
            "invalid_form_data" => ExchangeError::InvalidFormData,
            "invalid_post_type" => ExchangeError::InvalidPostType,
            "missing_post_type" => ExchangeError::MissingPostType,
            "team_added_to_org" => ExchangeError::TeamAddedToOrg,
            "invalid_json" => ExchangeError::InvalidJson,
            "json_not_object" => ExchangeError::JsonNotObject,
            "request_timeout" => ExchangeError::RequestTimeout,
            "upgrade_required" => ExchangeError::UpgradeRequired,
            "fatal_error" => ExchangeError::FatalError,
            _ => ExchangeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ExchangeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ExchangeError::NotEnterpriseTeam => {
                write!(f, "Server returned error not_enterprise_team")
            }
            ExchangeError::TooManyUsers => write!(f, "Server returned error too_many_users"),
            ExchangeError::NotAuthed => write!(f, "Server returned error not_authed"),
            ExchangeError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ExchangeError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ExchangeError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ExchangeError::NoPermission => write!(f, "Server returned error no_permission"),
            ExchangeError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            ExchangeError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ExchangeError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ExchangeError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ExchangeError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ExchangeError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ExchangeError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ExchangeError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ExchangeError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ExchangeError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ExchangeError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ExchangeError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            ExchangeError::FatalError => write!(f, "Server returned error fatal_error"),
            ExchangeError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ExchangeError::Unknown(ref s) => write!(f, "{}", s),
            ExchangeError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ExchangeError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ExchangeError::MalformedResponse(_, ref e) => Some(e),
            ExchangeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
