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
pub struct OpenRequest {
    /// The dialog definition. This must be a JSON-encoded string.
    pub dialog: String,
    /// Exchange a trigger to post to the user.
    pub trigger_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<OpenResponse, OpenError<E>>> for OpenResponse {
    fn into(self) -> Result<OpenResponse, OpenError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum OpenError<E: Error> {
    ValidationErrors,
    MissingTrigger,
    MissingDialog,
    TriggerExchanged,
    TriggerExpired,
    InvalidTrigger,
    AppMissingActionUrl,
    CannotCreateDialog,
    FailedSendingDialog,
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

impl<'a, E: Error> From<&'a str> for OpenError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "validation_errors" => OpenError::ValidationErrors,
            "missing_trigger" => OpenError::MissingTrigger,
            "missing_dialog" => OpenError::MissingDialog,
            "trigger_exchanged" => OpenError::TriggerExchanged,
            "trigger_expired" => OpenError::TriggerExpired,
            "invalid_trigger" => OpenError::InvalidTrigger,
            "app_missing_action_url" => OpenError::AppMissingActionUrl,
            "cannot_create_dialog" => OpenError::CannotCreateDialog,
            "failed_sending_dialog" => OpenError::FailedSendingDialog,
            "not_authed" => OpenError::NotAuthed,
            "invalid_auth" => OpenError::InvalidAuth,
            "account_inactive" => OpenError::AccountInactive,
            "token_revoked" => OpenError::TokenRevoked,
            "no_permission" => OpenError::NoPermission,
            "org_login_required" => OpenError::OrgLoginRequired,
            "invalid_arg_name" => OpenError::InvalidArgName,
            "invalid_array_arg" => OpenError::InvalidArrayArg,
            "invalid_charset" => OpenError::InvalidCharset,
            "invalid_form_data" => OpenError::InvalidFormData,
            "invalid_post_type" => OpenError::InvalidPostType,
            "missing_post_type" => OpenError::MissingPostType,
            "team_added_to_org" => OpenError::TeamAddedToOrg,
            "invalid_json" => OpenError::InvalidJson,
            "json_not_object" => OpenError::JsonNotObject,
            "request_timeout" => OpenError::RequestTimeout,
            "upgrade_required" => OpenError::UpgradeRequired,
            "fatal_error" => OpenError::FatalError,
            _ => OpenError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for OpenError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OpenError::ValidationErrors => write!(f, "Server returned error validation_errors"),
            OpenError::MissingTrigger => write!(f, "Server returned error missing_trigger"),
            OpenError::MissingDialog => write!(f, "Server returned error missing_dialog"),
            OpenError::TriggerExchanged => write!(f, "Server returned error trigger_exchanged"),
            OpenError::TriggerExpired => write!(f, "Server returned error trigger_expired"),
            OpenError::InvalidTrigger => write!(f, "Server returned error invalid_trigger"),
            OpenError::AppMissingActionUrl => {
                write!(f, "Server returned error app_missing_action_url")
            }
            OpenError::CannotCreateDialog => {
                write!(f, "Server returned error cannot_create_dialog")
            }
            OpenError::FailedSendingDialog => {
                write!(f, "Server returned error failed_sending_dialog")
            }
            OpenError::NotAuthed => write!(f, "Server returned error not_authed"),
            OpenError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            OpenError::AccountInactive => write!(f, "Server returned error account_inactive"),
            OpenError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            OpenError::NoPermission => write!(f, "Server returned error no_permission"),
            OpenError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            OpenError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            OpenError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            OpenError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            OpenError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            OpenError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            OpenError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            OpenError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            OpenError::InvalidJson => write!(f, "Server returned error invalid_json"),
            OpenError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            OpenError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            OpenError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            OpenError::FatalError => write!(f, "Server returned error fatal_error"),
            OpenError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            OpenError::Unknown(ref s) => write!(f, "{}", s),
            OpenError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for OpenError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            OpenError::MalformedResponse(_, ref e) => Some(e),
            OpenError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
