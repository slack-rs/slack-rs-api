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

pub mod approved_types;
pub mod requests_types;
pub mod restricted_types;

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct ApproveRequest<'a> {
    /// The id of the app to approve.
    pub app_id: Option<Cow<'a, str>>,
    /// The ID of the enterprise to approve the app on
    pub enterprise_id: Option<Cow<'a, str>>,
    /// The id of the request to approve.
    pub request_id: Option<Cow<'a, str>>,
    pub team_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApproveResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<ApproveResponse, ApproveError<E>>> for ApproveResponse {
    fn into(self) -> Result<ApproveResponse, ApproveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(ApproveError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum ApproveError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ApproveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => ApproveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ApproveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ApproveError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ApproveError::Unknown(ref s) => write!(f, "{}", s),
            ApproveError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ApproveError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ApproveError::MalformedResponse(_, ref e) => Some(e),
            ApproveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RestrictRequest<'a> {
    /// The id of the app to restrict.
    pub app_id: Option<Cow<'a, str>>,
    /// The id of the request to restrict.
    pub request_id: Option<Cow<'a, str>>,
    pub team_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RestrictResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RestrictResponse, RestrictError<E>>> for RestrictResponse {
    fn into(self) -> Result<RestrictResponse, RestrictError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(RestrictError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum RestrictError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RestrictError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => RestrictError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RestrictError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RestrictError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RestrictError::Unknown(ref s) => write!(f, "{}", s),
            RestrictError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RestrictError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RestrictError::MalformedResponse(_, ref e) => Some(e),
            RestrictError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
