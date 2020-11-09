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

pub use crate::mod_types::pins_types::*;
use crate::sync::SlackWebRequestSender;

/// Pins an item to a channel.
///
/// Wraps https://api.slack.com/methods/pins.add

pub fn add<R>(
    client: &R,
    token: &str,
    request: &AddRequest,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel", request.channel.to_string())),
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/pins.add");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
}
/// Lists items pinned to a channel.
///
/// Wraps https://api.slack.com/methods/pins.list

pub fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        Some(("channel", request.channel.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/pins.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
/// Un-pins an item from a channel.
///
/// Wraps https://api.slack.com/methods/pins.remove

pub fn remove<R>(
    client: &R,
    token: &str,
    request: &RemoveRequest,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel", request.channel.to_string())),
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/pins.remove");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
}
