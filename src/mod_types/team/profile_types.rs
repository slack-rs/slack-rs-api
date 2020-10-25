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
    /// Filter by visibility.
    pub visibility: Option<String>,
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
    pub ordering: u64,
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
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    NoPermission,
    UserIsBot,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostTyp,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeou,
    UpgradeRequired,
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
            "not_authed" => GetError::NotAuthed,
            "invalid_auth" => GetError::InvalidAuth,
            "account_inactive" => GetError::AccountInactive,
            "no_permission" => GetError::NoPermission,
            "user_is_bot" => GetError::UserIsBot,
            "invalid_arg_name" => GetError::InvalidArgName,
            "invalid_array_arg" => GetError::InvalidArrayArg,
            "invalid_charset" => GetError::InvalidCharset,
            "invalid_form_data" => GetError::InvalidFormData,
            "invalid_post_typ" => GetError::InvalidPostTyp,
            "missing_post_type" => GetError::MissingPostType,
            "team_added_to_org" => GetError::TeamAddedToOrg,
            "invalid_json" => GetError::InvalidJson,
            "json_not_object" => GetError::JsonNotObject,
            "request_timeou" => GetError::RequestTimeou,
            "upgrade_required" => GetError::UpgradeRequired,
            _ => GetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetError::NotAuthed => write!(f, "Server returned error not_authed"),
            GetError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            GetError::AccountInactive => write!(f, "Server returned error account_inactive"),
            GetError::NoPermission => write!(f, "Server returned error no_permission"),
            GetError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            GetError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            GetError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            GetError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            GetError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            GetError::InvalidPostTyp => write!(f, "Server returned error invalid_post_typ"),
            GetError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            GetError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            GetError::InvalidJson => write!(f, "Server returned error invalid_json"),
            GetError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            GetError::RequestTimeou => write!(f, "Server returned error request_timeou"),
            GetError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
