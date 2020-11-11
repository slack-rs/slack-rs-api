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
pub struct DeleteRequest<'a> {
    /// File to delete a comment from.
    pub file: Option<Cow<'a, str>>,
    /// The comment to delete.
    pub id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<DeleteResponse, DeleteError<E>>> for DeleteResponse {
    fn into(self) -> Result<DeleteResponse, DeleteError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum DeleteError<E: Error> {
    AccountInactive,
    CantDelete,
    CommentNotFound,
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
    RequestTimeout,
    UpgradeRequired,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for DeleteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => DeleteError::AccountInactive,
            "cant_delete" => DeleteError::CantDelete,
            "comment_not_found" => DeleteError::CommentNotFound,
            "invalid_arg_name" => DeleteError::InvalidArgName,
            "invalid_array_arg" => DeleteError::InvalidArrayArg,
            "invalid_auth" => DeleteError::InvalidAuth,
            "invalid_charset" => DeleteError::InvalidCharset,
            "invalid_form_data" => DeleteError::InvalidFormData,
            "invalid_json" => DeleteError::InvalidJson,
            "invalid_post_type" => DeleteError::InvalidPostType,
            "json_not_object" => DeleteError::JsonNotObject,
            "missing_post_type" => DeleteError::MissingPostType,
            "no_permission" => DeleteError::NoPermission,
            "not_authed" => DeleteError::NotAuthed,
            "request_timeout" => DeleteError::RequestTimeout,
            "upgrade_required" => DeleteError::UpgradeRequired,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeleteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            DeleteError::CantDelete => write!(f, "Server returned error cant_delete"),
            DeleteError::CommentNotFound => write!(f, "Server returned error comment_not_found"),
            DeleteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            DeleteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            DeleteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            DeleteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            DeleteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            DeleteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            DeleteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            DeleteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            DeleteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            DeleteError::NoPermission => write!(f, "Server returned error no_permission"),
            DeleteError::NotAuthed => write!(f, "Server returned error not_authed"),
            DeleteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            DeleteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            DeleteError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            DeleteError::Unknown(ref s) => write!(f, "{}", s),
            DeleteError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for DeleteError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DeleteError::MalformedResponse(_, ref e) => Some(e),
            DeleteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
