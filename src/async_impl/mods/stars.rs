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
pub use crate::mod_types::stars_types::*;
use std::borrow::Cow;

/// Adds a star to an item.
///
/// Wraps https://api.slack.com/methods/stars.add

pub async fn add<R>(
    client: &R,
    token: &str,
    request: &AddRequest<'_>,
) -> Result<AddResponse, AddError<R::Error>>
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
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/stars.add");
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
/// Lists stars for a user.
///
/// Wraps https://api.slack.com/methods/stars.list

pub async fn list<R>(
    client: &R,
    token: Option<&str>,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        token.map(|token| ("token", token)),
        request
            .count
            .as_ref()
            .map(|count| ("count", count.as_ref())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
        request.page.as_ref().map(|page| ("page", page.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/stars.list");
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
/// Removes a star from an item.
///
/// Wraps https://api.slack.com/methods/stars.remove

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
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/stars.remove");
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
