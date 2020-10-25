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

pub mod participants_types;

use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct UpdateRequest {
    /// `id` returned by the [`calls.add`](/methods/calls.add) method.
    pub id: String,
    /// The name of the Call.
    pub title: Option<String>,
    /// The URL required for a client to join the Call.
    pub join_url: Option<String>,
    /// When supplied, available Slack clients will attempt to directly launch the 3rd-party Call with this URL.
    pub desktop_app_join_url: Option<String>,
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

#[derive(Clone, Default, Debug)]
pub struct EndRequest {
    /// `id` returned when registering the call using the [`calls.add`](/methods/calls.add) method.
    pub id: String,
    /// Call duration in seconds
    pub duration: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EndResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<EndResponse, EndError<E>>> for EndResponse {
    fn into(self) -> Result<EndResponse, EndError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(EndError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum EndError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for EndError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => EndError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for EndError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EndError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            EndError::Unknown(ref s) => write!(f, "{}", s),
            EndError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for EndError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            EndError::MalformedResponse(_, ref e) => Some(e),
            EndError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest {
    /// `id` of the Call returned by the [`calls.add`](/methods/calls.add) method.
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<InfoResponse, InfoError<E>>> for InfoResponse {
    fn into(self) -> Result<InfoResponse, InfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(InfoError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum InfoError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InfoError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            InfoError::Unknown(ref s) => write!(f, "{}", s),
            InfoError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for InfoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            InfoError::MalformedResponse(_, ref e) => Some(e),
            InfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct AddRequest {
    /// An ID supplied by the 3rd-party Call provider. It must be unique across all Calls from that service.
    pub external_unique_id: String,
    /// An optional, human-readable ID supplied by the 3rd-party Call provider. If supplied, this ID will be displayed in the Call object.
    pub external_display_id: Option<String>,
    /// The URL required for a client to join the Call.
    pub join_url: String,
    /// When supplied, available Slack clients will attempt to directly launch the 3rd-party Call with this URL.
    pub desktop_app_join_url: Option<String>,
    /// Call start time in UTC UNIX timestamp format
    pub date_start: Option<u64>,
    /// The name of the Call.
    pub title: Option<String>,
    /// The valid Slack user ID of the user who created this Call. When this method is called with a user token, the `created_by` field is optional and defaults to the authed user of the token. Otherwise, the field is required.
    pub created_by: Option<String>,
    /// The list of users to register as participants in the Call. [Read more on how to specify users here](/apis/calls#users).
    pub users: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<AddResponse, AddError<E>>> for AddResponse {
    fn into(self) -> Result<AddResponse, AddError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(AddError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum AddError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AddError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => AddError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AddError::Unknown(ref s) => write!(f, "{}", s),
            AddError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AddError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AddError::MalformedResponse(_, ref e) => Some(e),
            AddError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
