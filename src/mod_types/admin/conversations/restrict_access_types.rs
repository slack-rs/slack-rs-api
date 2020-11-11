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
pub struct AddGroupRequest<'a> {
    /// The channel to link this group to.
    pub channel_id: Cow<'a, str>,
    /// The [IDP Group](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-Grid-org) ID to be an allowlist for the private channel.
    pub group_id: Cow<'a, str>,
    /// The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization.
    pub team_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddGroupResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<AddGroupResponse, AddGroupError<E>>> for AddGroupResponse {
    fn into(self) -> Result<AddGroupResponse, AddGroupError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(AddGroupError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum AddGroupError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AddGroupError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => AddGroupError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddGroupError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddGroupError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AddGroupError::Unknown(ref s) => write!(f, "{}", s),
            AddGroupError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AddGroupError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AddGroupError::MalformedResponse(_, ref e) => Some(e),
            AddGroupError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ListGroupsRequest<'a> {
    pub channel_id: Cow<'a, str>,
    /// The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization.
    pub team_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListGroupsResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<ListGroupsResponse, ListGroupsError<E>>> for ListGroupsResponse {
    fn into(self) -> Result<ListGroupsResponse, ListGroupsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(ListGroupsError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum ListGroupsError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ListGroupsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => ListGroupsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListGroupsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListGroupsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ListGroupsError::Unknown(ref s) => write!(f, "{}", s),
            ListGroupsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ListGroupsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ListGroupsError::MalformedResponse(_, ref e) => Some(e),
            ListGroupsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RemoveGroupRequest<'a> {
    /// The channel to remove the linked group from.
    pub channel_id: Cow<'a, str>,
    /// The [IDP Group](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-Grid-org) ID to remove from the private channel.
    pub group_id: Cow<'a, str>,
    /// The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization.
    pub team_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveGroupResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RemoveGroupResponse, RemoveGroupError<E>>> for RemoveGroupResponse {
    fn into(self) -> Result<RemoveGroupResponse, RemoveGroupError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(RemoveGroupError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum RemoveGroupError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RemoveGroupError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => RemoveGroupError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RemoveGroupError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RemoveGroupError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RemoveGroupError::Unknown(ref s) => write!(f, "{}", s),
            RemoveGroupError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RemoveGroupError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RemoveGroupError::MalformedResponse(_, ref e) => Some(e),
            RemoveGroupError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
