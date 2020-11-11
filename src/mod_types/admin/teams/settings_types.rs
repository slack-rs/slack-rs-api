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
pub struct InfoRequest<'a> {
    pub team_id: Cow<'a, str>,
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
pub struct SetDefaultChannelsRequest<'a> {
    /// An array of channel IDs.
    pub channel_ids: Cow<'a, str>,
    /// ID for the workspace to set the default channel for.
    pub team_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetDefaultChannelsResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetDefaultChannelsResponse, SetDefaultChannelsError<E>>>
    for SetDefaultChannelsResponse
{
    fn into(self) -> Result<SetDefaultChannelsResponse, SetDefaultChannelsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetDefaultChannelsError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetDefaultChannelsError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetDefaultChannelsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetDefaultChannelsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetDefaultChannelsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetDefaultChannelsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetDefaultChannelsError::Unknown(ref s) => write!(f, "{}", s),
            SetDefaultChannelsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetDefaultChannelsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetDefaultChannelsError::MalformedResponse(_, ref e) => Some(e),
            SetDefaultChannelsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetDescriptionRequest<'a> {
    /// The new description for the workspace.
    pub description: Cow<'a, str>,
    /// ID for the workspace to set the description for.
    pub team_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetDescriptionResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetDescriptionResponse, SetDescriptionError<E>>>
    for SetDescriptionResponse
{
    fn into(self) -> Result<SetDescriptionResponse, SetDescriptionError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetDescriptionError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetDescriptionError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetDescriptionError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetDescriptionError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetDescriptionError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetDescriptionError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetDescriptionError::Unknown(ref s) => write!(f, "{}", s),
            SetDescriptionError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetDescriptionError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetDescriptionError::MalformedResponse(_, ref e) => Some(e),
            SetDescriptionError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetDiscoverabilityRequest<'a> {
    /// This workspace's discovery setting. It must be set to one of `open`, `invite_only`, `closed`, or `unlisted`.
    pub discoverability: Cow<'a, str>,
    /// The ID of the workspace to set discoverability on.
    pub team_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetDiscoverabilityResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetDiscoverabilityResponse, SetDiscoverabilityError<E>>>
    for SetDiscoverabilityResponse
{
    fn into(self) -> Result<SetDiscoverabilityResponse, SetDiscoverabilityError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetDiscoverabilityError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetDiscoverabilityError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetDiscoverabilityError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetDiscoverabilityError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetDiscoverabilityError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetDiscoverabilityError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetDiscoverabilityError::Unknown(ref s) => write!(f, "{}", s),
            SetDiscoverabilityError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetDiscoverabilityError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetDiscoverabilityError::MalformedResponse(_, ref e) => Some(e),
            SetDiscoverabilityError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetIconRequest<'a> {
    /// Image URL for the icon
    pub image_url: Cow<'a, str>,
    /// ID for the workspace to set the icon for.
    pub team_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetIconResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetIconResponse, SetIconError<E>>> for SetIconResponse {
    fn into(self) -> Result<SetIconResponse, SetIconError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetIconError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetIconError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetIconError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetIconError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetIconError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetIconError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetIconError::Unknown(ref s) => write!(f, "{}", s),
            SetIconError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetIconError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetIconError::MalformedResponse(_, ref e) => Some(e),
            SetIconError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetNameRequest<'a> {
    /// The new name of the workspace.
    pub name: Cow<'a, str>,
    /// ID for the workspace to set the name for.
    pub team_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetNameResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetNameResponse, SetNameError<E>>> for SetNameResponse {
    fn into(self) -> Result<SetNameResponse, SetNameError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetNameError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetNameError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetNameError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetNameError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetNameError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetNameError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetNameError::Unknown(ref s) => write!(f, "{}", s),
            SetNameError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetNameError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetNameError::MalformedResponse(_, ref e) => Some(e),
            SetNameError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
