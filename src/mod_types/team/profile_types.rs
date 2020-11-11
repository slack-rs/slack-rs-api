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
    /// Filter by visibility.
    pub visibility: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetOptionsInner {
    pub is_custom: Option<bool>,
    pub is_multiple_entry: Option<bool>,
    pub is_protected: Option<bool>,
    pub is_scim: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetFieldsInner {
    pub field_name: Option<String>,
    pub hint: String,
    pub id: String,
    pub is_hidden: Option<bool>,
    pub label: String,
    pub options: Option<Vec<GetOptionsInner>>,
    pub ordering: f64,
    pub possible_values: Option<Vec<String>>,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetProfileInner {
    pub fields: Vec<GetFieldsInner>,
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
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostTyp,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAuthed,
    RequestTimeou,
    TeamAddedToOrg,
    UpgradeRequired,
    UserIsBot,
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
            "invalid_arg_name" => GetError::InvalidArgName,
            "invalid_array_arg" => GetError::InvalidArrayArg,
            "invalid_auth" => GetError::InvalidAuth,
            "invalid_charset" => GetError::InvalidCharset,
            "invalid_form_data" => GetError::InvalidFormData,
            "invalid_json" => GetError::InvalidJson,
            "invalid_post_typ" => GetError::InvalidPostTyp,
            "json_not_object" => GetError::JsonNotObject,
            "missing_post_type" => GetError::MissingPostType,
            "no_permission" => GetError::NoPermission,
            "not_authed" => GetError::NotAuthed,
            "request_timeou" => GetError::RequestTimeou,
            "team_added_to_org" => GetError::TeamAddedToOrg,
            "upgrade_required" => GetError::UpgradeRequired,
            "user_is_bot" => GetError::UserIsBot,
            _ => GetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetError::AccountInactive => write!(f, "Server returned error account_inactive"),
            GetError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            GetError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            GetError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            GetError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            GetError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            GetError::InvalidJson => write!(f, "Server returned error invalid_json"),
            GetError::InvalidPostTyp => write!(f, "Server returned error invalid_post_typ"),
            GetError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            GetError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            GetError::NoPermission => write!(f, "Server returned error no_permission"),
            GetError::NotAuthed => write!(f, "Server returned error not_authed"),
            GetError::RequestTimeou => write!(f, "Server returned error request_timeou"),
            GetError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            GetError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            GetError::UserIsBot => write!(f, "Server returned error user_is_bot"),
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
