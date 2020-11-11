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

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::reactions_types::*;
use std::borrow::Cow;

/// Adds a reaction to an item.
///
/// Wraps https://api.slack.com/methods/reactions.add

pub async fn add<R>(
    client: &R,
    token: &str,
    request: &AddRequest<'_>,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel", request.channel.as_ref())),
        Some(("name", request.name.as_ref())),
        Some(("timestamp", request.timestamp.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reactions.add");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Gets reactions for an item.
///
/// Wraps https://api.slack.com/methods/reactions.get

pub async fn get<R>(
    client: &R,
    token: &str,
    request: &GetRequest<'_>,
) -> Result<GetResponse, GetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let full: Option<Cow<'_, str>> = request.full.as_ref().map(|full| full.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.as_ref())),
        request.file.as_ref().map(|file| ("file", file.as_ref())),
        request
            .file_comment
            .as_ref()
            .map(|file_comment| ("file_comment", file_comment.as_ref())),
        full.as_ref().map(|full| ("full", full.as_ref())),
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reactions.get");
    client
        .get(&url, &params[..])
        .await
        .map_err(GetError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result)
                .map_err(|e| GetError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Lists reactions made by a user.
///
/// Wraps https://api.slack.com/methods/reactions.list

pub async fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count: Option<Cow<'_, str>> = request.count.as_ref().map(|count| count.to_string().into());
    let full: Option<Cow<'_, str>> = request.full.as_ref().map(|full| full.to_string().into());
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let page: Option<Cow<'_, str>> = request.page.as_ref().map(|page| page.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        count.as_ref().map(|count| ("count", count.as_ref())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        full.as_ref().map(|full| ("full", full.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
        page.as_ref().map(|page| ("page", page.as_ref())),
        request.user.as_ref().map(|user| ("user", user.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reactions.list");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Removes a reaction from an item.
///
/// Wraps https://api.slack.com/methods/reactions.remove

pub async fn remove<R>(
    client: &R,
    token: &str,
    request: &RemoveRequest<'_>,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.as_ref())),
        request.file.as_ref().map(|file| ("file", file.as_ref())),
        request
            .file_comment
            .as_ref()
            .map(|file_comment| ("file_comment", file_comment.as_ref())),
        Some(("name", request.name.as_ref())),
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reactions.remove");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
