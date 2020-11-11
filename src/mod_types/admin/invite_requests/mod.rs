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
pub mod denied_types;

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct ApproveRequest<'a> {
    /// ID of the request to invite.
    pub invite_request_id: Cow<'a, str>,
    /// ID for the workspace where the invite request was made.
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
pub struct DenyRequest<'a> {
    /// ID of the request to invite.
    pub invite_request_id: Cow<'a, str>,
    /// ID for the workspace where the invite request was made.
    pub team_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DenyResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<DenyResponse, DenyError<E>>> for DenyResponse {
    fn into(self) -> Result<DenyResponse, DenyError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(DenyError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum DenyError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for DenyError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => DenyError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DenyError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DenyError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            DenyError::Unknown(ref s) => write!(f, "{}", s),
            DenyError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for DenyError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DenyError::MalformedResponse(_, ref e) => Some(e),
            DenyError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Value of the `next_cursor` field sent as part of the previous API response
    pub cursor: Option<Cow<'a, str>>,
    /// The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive
    pub limit: Option<u64>,
    /// ID for the workspace where the invite requests were made.
    pub team_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<ListResponse, ListError<E>>> for ListResponse {
    fn into(self) -> Result<ListResponse, ListError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(ListError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum ListError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ListError::Unknown(ref s) => write!(f, "{}", s),
            ListError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ListError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ListError::MalformedResponse(_, ref e) => Some(e),
            ListError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
