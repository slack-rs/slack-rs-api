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
pub struct ListRequest<'a> {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<Cow<'a, str>>,
    /// The maximum number of items to return.
    pub limit: Option<u64>,
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

#[derive(Clone, Default, Debug)]
pub struct RequestRequest<'a> {
    /// A comma separated list of user scopes to request for
    pub scopes: Cow<'a, str>,
    /// Token used to trigger the request
    pub trigger_id: Cow<'a, str>,
    /// The user this scope is being requested for
    pub user: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RequestResponse, RequestError<E>>> for RequestResponse {
    fn into(self) -> Result<RequestResponse, RequestError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(RequestError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum RequestError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RequestError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => RequestError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RequestError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RequestError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RequestError::Unknown(ref s) => write!(f, "{}", s),
            RequestError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RequestError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RequestError::MalformedResponse(_, ref e) => Some(e),
            RequestError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
