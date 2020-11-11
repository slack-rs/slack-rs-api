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
pub struct InvalidateRequest<'a> {
    pub session_id: u64,
    /// ID of the team that the session belongs to
    pub team_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InvalidateResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<InvalidateResponse, InvalidateError<E>>> for InvalidateResponse {
    fn into(self) -> Result<InvalidateResponse, InvalidateError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(InvalidateError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum InvalidateError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for InvalidateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => InvalidateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InvalidateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InvalidateError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            InvalidateError::Unknown(ref s) => write!(f, "{}", s),
            InvalidateError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for InvalidateError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            InvalidateError::MalformedResponse(_, ref e) => Some(e),
            InvalidateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ResetRequest<'a> {
    /// Only expire mobile sessions (default: false)
    pub mobile_only: Option<bool>,
    /// The ID of the user to wipe sessions for
    pub user_id: Cow<'a, str>,
    /// Only expire web sessions (default: false)
    pub web_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResetResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<ResetResponse, ResetError<E>>> for ResetResponse {
    fn into(self) -> Result<ResetResponse, ResetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(ResetError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum ResetError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ResetError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => ResetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ResetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ResetError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ResetError::Unknown(ref s) => write!(f, "{}", s),
            ResetError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ResetError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ResetError::MalformedResponse(_, ref e) => Some(e),
            ResetError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
