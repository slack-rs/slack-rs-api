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
pub struct InfoRequest {
    /// Bot user to get info on
    pub bot: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoBotInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: InfoIconsInner,
    pub id: String,
    pub name: String,
    pub updated: u64,
    pub user_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub bot: InfoBotInner,
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
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
    BotNotFound,
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

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "bot_not_found" => InfoError::BotNotFound,
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
            "token_revoked" => InfoError::TokenRevoked,
            "no_permission" => InfoError::NoPermission,
            "org_login_required" => InfoError::OrgLoginRequired,
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
            InfoError::BotNotFound => write!(f, "Server returned error bot_not_found"),
            InfoError::NotAuthed => write!(f, "Server returned error not_authed"),
            InfoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InfoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InfoError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            InfoError::NoPermission => write!(f, "Server returned error no_permission"),
            InfoError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
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
