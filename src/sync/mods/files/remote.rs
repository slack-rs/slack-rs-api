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

pub use crate::mod_types::files::remote_types::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Adds a file from a remote service
///
/// Wraps https://api.slack.com/methods/files.remote.add

pub fn add<R>(
    client: &R,
    token: Option<&str>,
    request: &AddRequest<'_>,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.as_ref())),
        request
            .external_url
            .as_ref()
            .map(|external_url| ("external_url", external_url.as_ref())),
        request
            .filetype
            .as_ref()
            .map(|filetype| ("filetype", filetype.as_ref())),
        request
            .indexable_file_contents
            .as_ref()
            .map(|indexable_file_contents| {
                ("indexable_file_contents", indexable_file_contents.as_ref())
            }),
        request
            .preview_image
            .as_ref()
            .map(|preview_image| ("preview_image", preview_image.as_ref())),
        request
            .title
            .as_ref()
            .map(|title| ("title", title.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.add");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t)]),
        )
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Retrieve information about a remote file added to Slack
///
/// Wraps https://api.slack.com/methods/files.remote.info

pub fn info<R>(
    client: &R,
    token: Option<&str>,
    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        token.map(|token| ("token", token)),
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.as_ref())),
        request.file.as_ref().map(|file| ("file", file.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Retrieve information about a remote file added to Slack
///
/// Wraps https://api.slack.com/methods/files.remote.list

pub fn list<R>(
    client: &R,
    token: Option<&str>,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let ts_from: Option<Cow<'_, str>> = request
        .ts_from
        .as_ref()
        .map(|ts_from| ts_from.to_string().into());
    let ts_to: Option<Cow<'_, str>> = request.ts_to.as_ref().map(|ts_to| ts_to.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        token.map(|token| ("token", token)),
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.as_ref())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
        ts_from
            .as_ref()
            .map(|ts_from| ("ts_from", ts_from.as_ref())),
        ts_to.as_ref().map(|ts_to| ("ts_to", ts_to.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Remove a remote file.
///
/// Wraps https://api.slack.com/methods/files.remote.remove

pub fn remove<R>(
    client: &R,
    token: Option<&str>,
    request: &RemoveRequest<'_>,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.as_ref())),
        request.file.as_ref().map(|file| ("file", file.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.remove");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t)]),
        )
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Share a remote file into a channel.
///
/// Wraps https://api.slack.com/methods/files.remote.share

pub fn share<R>(
    client: &R,
    token: Option<&str>,
    request: &ShareRequest<'_>,
) -> Result<ShareResponse, ShareError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        token.map(|token| ("token", token)),
        request
            .channels
            .as_ref()
            .map(|channels| ("channels", channels.as_ref())),
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.as_ref())),
        request.file.as_ref().map(|file| ("file", file.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.share");
    client
        .get(&url, &params[..])
        .map_err(ShareError::Client)
        .and_then(|result| {
            serde_json::from_str::<ShareResponse>(&result)
                .map_err(|e| ShareError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Updates an existing remote file.
///
/// Wraps https://api.slack.com/methods/files.remote.update

pub fn update<R>(
    client: &R,
    token: Option<&str>,
    request: &UpdateRequest<'_>,
) -> Result<UpdateResponse, UpdateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.as_ref())),
        request
            .external_url
            .as_ref()
            .map(|external_url| ("external_url", external_url.as_ref())),
        request.file.as_ref().map(|file| ("file", file.as_ref())),
        request
            .filetype
            .as_ref()
            .map(|filetype| ("filetype", filetype.as_ref())),
        request
            .indexable_file_contents
            .as_ref()
            .map(|indexable_file_contents| {
                ("indexable_file_contents", indexable_file_contents.as_ref())
            }),
        request
            .preview_image
            .as_ref()
            .map(|preview_image| ("preview_image", preview_image.as_ref())),
        request
            .title
            .as_ref()
            .map(|title| ("title", title.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.update");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t)]),
        )
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
