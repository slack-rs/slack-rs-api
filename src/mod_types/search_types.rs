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
pub struct MessagesRequest<'a> {
    /// Pass the number of results you want per "page". Maximum of `100`.
    pub count: Option<u64>,
    /// Pass a value of `true` to enable query highlight markers (see below).
    pub highlight: Option<bool>,
    pub page: Option<u64>,
    /// Search query.
    pub query: Cow<'a, str>,
    /// Return matches sorted by either `score` or `timestamp`.
    pub sort: Option<Cow<'a, str>>,
    /// Change sort direction to ascending (`asc`) or descending (`desc`).
    pub sort_dir: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessagesResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<MessagesResponse, MessagesError<E>>> for MessagesResponse {
    fn into(self) -> Result<MessagesResponse, MessagesError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(MessagesError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum MessagesError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for MessagesError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => MessagesError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MessagesError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MessagesError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            MessagesError::Unknown(ref s) => write!(f, "{}", s),
            MessagesError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for MessagesError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            MessagesError::MalformedResponse(_, ref e) => Some(e),
            MessagesError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
