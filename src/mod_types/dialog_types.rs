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
pub struct OpenRequest<'a> {
    /// The dialog definition. This must be a JSON-encoded string.
    pub dialog: Cow<'a, str>,
    /// Exchange a trigger to post to the user.
    pub trigger_id: Cow<'a, str>,
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
    AccountInactive,
    AppMissingActionUrl,
    CannotCreateDialog,
    FailedSendingDialog,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    InvalidTrigger,
    JsonNotObject,
    MissingDialog,
    MissingPostType,
    MissingTrigger,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    TriggerExchanged,
    TriggerExpired,
    UpgradeRequired,
    ValidationErrors,
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
            "account_inactive" => OpenError::AccountInactive,
            "app_missing_action_url" => OpenError::AppMissingActionUrl,
            "cannot_create_dialog" => OpenError::CannotCreateDialog,
            "failed_sending_dialog" => OpenError::FailedSendingDialog,
            "fatal_error" => OpenError::FatalError,
            "invalid_arg_name" => OpenError::InvalidArgName,
            "invalid_array_arg" => OpenError::InvalidArrayArg,
            "invalid_auth" => OpenError::InvalidAuth,
            "invalid_charset" => OpenError::InvalidCharset,
            "invalid_form_data" => OpenError::InvalidFormData,
            "invalid_json" => OpenError::InvalidJson,
            "invalid_post_type" => OpenError::InvalidPostType,
            "invalid_trigger" => OpenError::InvalidTrigger,
            "json_not_object" => OpenError::JsonNotObject,
            "missing_dialog" => OpenError::MissingDialog,
            "missing_post_type" => OpenError::MissingPostType,
            "missing_trigger" => OpenError::MissingTrigger,
            "no_permission" => OpenError::NoPermission,
            "not_authed" => OpenError::NotAuthed,
            "org_login_required" => OpenError::OrgLoginRequired,
            "request_timeout" => OpenError::RequestTimeout,
            "team_added_to_org" => OpenError::TeamAddedToOrg,
            "token_revoked" => OpenError::TokenRevoked,
            "trigger_exchanged" => OpenError::TriggerExchanged,
            "trigger_expired" => OpenError::TriggerExpired,
            "upgrade_required" => OpenError::UpgradeRequired,
            "validation_errors" => OpenError::ValidationErrors,
            _ => OpenError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for OpenError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OpenError::AccountInactive => write!(f, "Server returned error account_inactive"),
            OpenError::AppMissingActionUrl => {
                write!(f, "Server returned error app_missing_action_url")
            }
            OpenError::CannotCreateDialog => {
                write!(f, "Server returned error cannot_create_dialog")
            }
            OpenError::FailedSendingDialog => {
                write!(f, "Server returned error failed_sending_dialog")
            }
            OpenError::FatalError => write!(f, "Server returned error fatal_error"),
            OpenError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            OpenError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            OpenError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            OpenError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            OpenError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            OpenError::InvalidJson => write!(f, "Server returned error invalid_json"),
            OpenError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            OpenError::InvalidTrigger => write!(f, "Server returned error invalid_trigger"),
            OpenError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            OpenError::MissingDialog => write!(f, "Server returned error missing_dialog"),
            OpenError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            OpenError::MissingTrigger => write!(f, "Server returned error missing_trigger"),
            OpenError::NoPermission => write!(f, "Server returned error no_permission"),
            OpenError::NotAuthed => write!(f, "Server returned error not_authed"),
            OpenError::OrgLoginRequired => write!(f, "Server returned error org_login_required"),
            OpenError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            OpenError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            OpenError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            OpenError::TriggerExchanged => write!(f, "Server returned error trigger_exchanged"),
            OpenError::TriggerExpired => write!(f, "Server returned error trigger_expired"),
            OpenError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            OpenError::ValidationErrors => write!(f, "Server returned error validation_errors"),
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
