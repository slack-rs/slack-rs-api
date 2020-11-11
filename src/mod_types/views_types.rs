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
    /// Exchange a trigger to post to the user.
    pub trigger_id: Cow<'a, str>,
    /// A [view payload](/reference/surfaces/views). This must be a JSON-encoded string.
    pub view: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<OpenResponse, OpenError<E>>> for OpenResponse {
    fn into(self) -> Result<OpenResponse, OpenError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(OpenError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum OpenError<E: Error> {
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
            _ => OpenError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for OpenError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
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

#[derive(Clone, Default, Debug)]
pub struct PublishRequest<'a> {
    /// A string that represents view state to protect against possible race conditions.
    pub hash: Option<Cow<'a, str>>,
    /// `id` of the user you want publish a view to.
    pub user_id: Cow<'a, str>,
    /// A [view payload](/reference/surfaces/views). This must be a JSON-encoded string.
    pub view: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PublishResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<PublishResponse, PublishError<E>>> for PublishResponse {
    fn into(self) -> Result<PublishResponse, PublishError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(PublishError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum PublishError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for PublishError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => PublishError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for PublishError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PublishError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            PublishError::Unknown(ref s) => write!(f, "{}", s),
            PublishError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for PublishError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            PublishError::MalformedResponse(_, ref e) => Some(e),
            PublishError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct PushRequest<'a> {
    /// Exchange a trigger to post to the user.
    pub trigger_id: Cow<'a, str>,
    /// A [view payload](/reference/surfaces/views). This must be a JSON-encoded string.
    pub view: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PushResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<PushResponse, PushError<E>>> for PushResponse {
    fn into(self) -> Result<PushResponse, PushError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(PushError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum PushError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for PushError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => PushError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for PushError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PushError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            PushError::Unknown(ref s) => write!(f, "{}", s),
            PushError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for PushError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            PushError::MalformedResponse(_, ref e) => Some(e),
            PushError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct UpdateRequest<'a> {
    /// A unique identifier of the view set by the developer. Must be unique for all views on a team. Max length of 255 characters. Either `view_id` or `external_id` is required.
    pub external_id: Option<Cow<'a, str>>,
    /// A string that represents view state to protect against possible race conditions.
    pub hash: Option<Cow<'a, str>>,
    /// A [view object](/reference/surfaces/views). This must be a JSON-encoded string.
    pub view: Option<Cow<'a, str>>,
    /// A unique identifier of the view to be updated. Either `view_id` or `external_id` is required.
    pub view_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<UpdateResponse, UpdateError<E>>> for UpdateResponse {
    fn into(self) -> Result<UpdateResponse, UpdateError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(UpdateError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum UpdateError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for UpdateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => UpdateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UpdateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UpdateError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            UpdateError::Unknown(ref s) => write!(f, "{}", s),
            UpdateError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for UpdateError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UpdateError::MalformedResponse(_, ref e) => Some(e),
            UpdateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
