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
pub struct ListOriginalConnectedChannelInfoRequest<'a> {
    /// A comma-separated list of channels to filter to.
    pub channel_ids: Option<Cow<'a, str>>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    pub cursor: Option<Cow<'a, str>>,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    pub limit: Option<u64>,
    /// A comma-separated list of the workspaces to which the channels you would like returned belong.
    pub team_ids: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListOriginalConnectedChannelInfoResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error>
    Into<Result<ListOriginalConnectedChannelInfoResponse, ListOriginalConnectedChannelInfoError<E>>>
    for ListOriginalConnectedChannelInfoResponse
{
    fn into(
        self,
    ) -> Result<ListOriginalConnectedChannelInfoResponse, ListOriginalConnectedChannelInfoError<E>>
    {
        if self.ok {
            Ok(self)
        } else {
            Err(ListOriginalConnectedChannelInfoError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum ListOriginalConnectedChannelInfoError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ListOriginalConnectedChannelInfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => ListOriginalConnectedChannelInfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListOriginalConnectedChannelInfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListOriginalConnectedChannelInfoError::MalformedResponse(_, ref e) => {
                write!(f, "{}", e)
            }
            ListOriginalConnectedChannelInfoError::Unknown(ref s) => write!(f, "{}", s),
            ListOriginalConnectedChannelInfoError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ListOriginalConnectedChannelInfoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ListOriginalConnectedChannelInfoError::MalformedResponse(_, ref e) => Some(e),
            ListOriginalConnectedChannelInfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
