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
pub use crate::mod_types::files::remote_types::*;

/// Adds a file from a remote service
///
/// Wraps https://api.slack.com/methods/files.remote.add

pub async fn add<R>(
    client: &R,
    token: Option<&str>,
    request: &AddRequest,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request
            .external_url
            .as_ref()
            .map(|external_url| ("external_url", external_url.to_string())),
        request
            .filetype
            .as_ref()
            .map(|filetype| ("filetype", filetype.to_string())),
        request
            .indexable_file_contents
            .as_ref()
            .map(|indexable_file_contents| {
                (
                    "indexable_file_contents",
                    indexable_file_contents.to_string(),
                )
            }),
        request
            .preview_image
            .as_ref()
            .map(|preview_image| ("preview_image", preview_image.to_string())),
        request
            .title
            .as_ref()
            .map(|title| ("title", title.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.add");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
}
/// Retrieve information about a remote file added to Slack
///
/// Wraps https://api.slack.com/methods/files.remote.info

pub async fn info<R>(
    client: &R,
    token: Option<&str>,
    request: &InfoRequest,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        token.map(|token| ("token", token.to_string())),
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.info");
    client
        .get(&url, &params[..])
        .await
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
}
/// Retrieve information about a remote file added to Slack
///
/// Wraps https://api.slack.com/methods/files.remote.list

pub async fn list<R>(
    client: &R,
    token: Option<&str>,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        token.map(|token| ("token", token.to_string())),
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request
            .ts_from
            .as_ref()
            .map(|ts_from| ("ts_from", ts_from.to_string())),
        request
            .ts_to
            .as_ref()
            .map(|ts_to| ("ts_to", ts_to.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.list");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
/// Remove a remote file.
///
/// Wraps https://api.slack.com/methods/files.remote.remove

pub async fn remove<R>(
    client: &R,
    token: Option<&str>,
    request: &RemoveRequest,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.remove");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
}
/// Share a remote file into a channel.
///
/// Wraps https://api.slack.com/methods/files.remote.share

pub async fn share<R>(
    client: &R,
    token: Option<&str>,
    request: &ShareRequest,
) -> Result<ShareResponse, ShareError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        token.map(|token| ("token", token.to_string())),
        request
            .channels
            .as_ref()
            .map(|channels| ("channels", channels.to_string())),
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.share");
    client
        .get(&url, &params[..])
        .await
        .map_err(ShareError::Client)
        .and_then(|result| {
            serde_json::from_str::<ShareResponse>(&result)
                .map_err(|e| ShareError::MalformedResponse(result, e))
        })
}
/// Updates an existing remote file.
///
/// Wraps https://api.slack.com/methods/files.remote.update

pub async fn update<R>(
    client: &R,
    token: Option<&str>,
    request: &UpdateRequest,
) -> Result<UpdateResponse, UpdateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request
            .external_url
            .as_ref()
            .map(|external_url| ("external_url", external_url.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
        request
            .filetype
            .as_ref()
            .map(|filetype| ("filetype", filetype.to_string())),
        request
            .indexable_file_contents
            .as_ref()
            .map(|indexable_file_contents| {
                (
                    "indexable_file_contents",
                    indexable_file_contents.to_string(),
                )
            }),
        request
            .preview_image
            .as_ref()
            .map(|preview_image| ("preview_image", preview_image.to_string())),
        request
            .title
            .as_ref()
            .map(|title| ("title", title.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.update");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
}
