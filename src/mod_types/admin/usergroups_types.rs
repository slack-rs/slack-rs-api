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
pub struct AddChannelsRequest<'a> {
    /// Comma separated string of channel IDs.
    pub channel_ids: Cow<'a, str>,
    /// The workspace to add default channels in.
    pub team_id: Option<Cow<'a, str>>,
    /// ID of the IDP group to add default channels for.
    pub usergroup_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddChannelsResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<AddChannelsResponse, AddChannelsError<E>>> for AddChannelsResponse {
    fn into(self) -> Result<AddChannelsResponse, AddChannelsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(AddChannelsError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum AddChannelsError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AddChannelsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => AddChannelsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddChannelsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddChannelsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AddChannelsError::Unknown(ref s) => write!(f, "{}", s),
            AddChannelsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AddChannelsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AddChannelsError::MalformedResponse(_, ref e) => Some(e),
            AddChannelsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct AddTeamsRequest<'a> {
    /// When `true`, this method automatically creates new workspace accounts for the IDP group members.
    pub auto_provision: Option<bool>,
    /// A comma separated list of encoded team (workspace) IDs. Each workspace *MUST* belong to the organization associated with the token.
    pub team_ids: Cow<'a, str>,
    /// An encoded usergroup (IDP Group) ID.
    pub usergroup_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddTeamsResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<AddTeamsResponse, AddTeamsError<E>>> for AddTeamsResponse {
    fn into(self) -> Result<AddTeamsResponse, AddTeamsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(AddTeamsError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum AddTeamsError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AddTeamsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => AddTeamsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddTeamsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddTeamsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AddTeamsError::Unknown(ref s) => write!(f, "{}", s),
            AddTeamsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AddTeamsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AddTeamsError::MalformedResponse(_, ref e) => Some(e),
            AddTeamsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ListChannelsRequest<'a> {
    /// Flag to include or exclude the count of members per channel.
    pub include_num_members: Option<bool>,
    /// ID of the the workspace.
    pub team_id: Option<Cow<'a, str>>,
    /// ID of the IDP group to list default channels for.
    pub usergroup_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListChannelsResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<ListChannelsResponse, ListChannelsError<E>>> for ListChannelsResponse {
    fn into(self) -> Result<ListChannelsResponse, ListChannelsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(ListChannelsError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum ListChannelsError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ListChannelsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => ListChannelsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListChannelsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListChannelsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ListChannelsError::Unknown(ref s) => write!(f, "{}", s),
            ListChannelsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ListChannelsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ListChannelsError::MalformedResponse(_, ref e) => Some(e),
            ListChannelsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RemoveChannelsRequest<'a> {
    /// Comma-separated string of channel IDs
    pub channel_ids: Cow<'a, str>,
    /// ID of the IDP Group
    pub usergroup_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveChannelsResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RemoveChannelsResponse, RemoveChannelsError<E>>>
    for RemoveChannelsResponse
{
    fn into(self) -> Result<RemoveChannelsResponse, RemoveChannelsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(RemoveChannelsError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum RemoveChannelsError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RemoveChannelsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => RemoveChannelsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RemoveChannelsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RemoveChannelsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RemoveChannelsError::Unknown(ref s) => write!(f, "{}", s),
            RemoveChannelsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RemoveChannelsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RemoveChannelsError::MalformedResponse(_, ref e) => Some(e),
            RemoveChannelsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
