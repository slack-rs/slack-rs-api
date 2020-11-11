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
pub struct AccessRequest<'a> {
    /// Issued when you created your application.
    pub client_id: Option<Cow<'a, str>>,
    /// Issued when you created your application.
    pub client_secret: Option<Cow<'a, str>>,
    /// The `code` param returned via the OAuth callback.
    pub code: Cow<'a, str>,
    /// This must match the originally submitted URI (if one was sent).
    pub redirect_uri: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccessResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<AccessResponse, AccessError<E>>> for AccessResponse {
    fn into(self) -> Result<AccessResponse, AccessError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(AccessError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum AccessError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AccessError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => AccessError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AccessError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AccessError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AccessError::Unknown(ref s) => write!(f, "{}", s),
            AccessError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AccessError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AccessError::MalformedResponse(_, ref e) => Some(e),
            AccessError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
