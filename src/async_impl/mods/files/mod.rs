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

pub mod comments;
pub mod remote;

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::files::*;

/// Deletes a file.
///
/// Wraps https://api.slack.com/methods/files.delete

pub async fn delete<R>(
    client: &R,
    token: Option<&str>,
    request: &DeleteRequest,
) -> Result<DeleteResponse, DeleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request.file.as_ref().map(|file| ("file", file.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.delete");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(DeleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|e| DeleteError::MalformedResponse(result, e))
        })
}
/// Gets information about a file.
///
/// Wraps https://api.slack.com/methods/files.info

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
            .count
            .as_ref()
            .map(|count| ("count", count.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request.page.as_ref().map(|page| ("page", page.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.info");
    client
        .get(&url, &params[..])
        .await
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
}
/// List for a team, in a channel, or from a user with applied filters.
///
/// Wraps https://api.slack.com/methods/files.list

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
            .count
            .as_ref()
            .map(|count| ("count", count.to_string())),
        request.page.as_ref().map(|page| ("page", page.to_string())),
        request
            .show_files_hidden_by_limit
            .as_ref()
            .map(|show_files_hidden_by_limit| {
                (
                    "show_files_hidden_by_limit",
                    show_files_hidden_by_limit.to_string(),
                )
            }),
        request
            .ts_from
            .as_ref()
            .map(|ts_from| ("ts_from", ts_from.to_string())),
        request
            .ts_to
            .as_ref()
            .map(|ts_to| ("ts_to", ts_to.to_string())),
        request
            .types
            .as_ref()
            .map(|types| ("types", types.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.list");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
/// Revokes public/external sharing access for a file
///
/// Wraps https://api.slack.com/methods/files.revokePublicURL

pub async fn revoke_public_url<R>(
    client: &R,
    token: Option<&str>,
    request: &RevokePublicURLRequest,
) -> Result<RevokePublicURLResponse, RevokePublicURLError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request.file.as_ref().map(|file| ("file", file.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.revokePublicURL");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(RevokePublicURLError::Client)
        .and_then(|result| {
            serde_json::from_str::<RevokePublicURLResponse>(&result)
                .map_err(|e| RevokePublicURLError::MalformedResponse(result, e))
        })
}
/// Enables a file for public/external sharing.
///
/// Wraps https://api.slack.com/methods/files.sharedPublicURL

pub async fn shared_public_url<R>(
    client: &R,
    token: Option<&str>,
    request: &SharedPublicURLRequest,
) -> Result<SharedPublicURLResponse, SharedPublicURLError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request.file.as_ref().map(|file| ("file", file.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.sharedPublicURL");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(SharedPublicURLError::Client)
        .and_then(|result| {
            serde_json::from_str::<SharedPublicURLResponse>(&result)
                .map_err(|e| SharedPublicURLError::MalformedResponse(result, e))
        })
}
/// Uploads or creates a file.
///
/// Wraps https://api.slack.com/methods/files.upload

pub async fn upload<R>(
    client: &R,
    token: Option<&str>,
    request: &UploadRequest,
) -> Result<UploadResponse, UploadError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channels
            .as_ref()
            .map(|channels| ("channels", channels.to_string())),
        request
            .content
            .as_ref()
            .map(|content| ("content", content.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
        request
            .filename
            .as_ref()
            .map(|filename| ("filename", filename.to_string())),
        request
            .filetype
            .as_ref()
            .map(|filetype| ("filetype", filetype.to_string())),
        request
            .initial_comment
            .as_ref()
            .map(|initial_comment| ("initial_comment", initial_comment.to_string())),
        request
            .thread_ts
            .as_ref()
            .map(|thread_ts| ("thread_ts", thread_ts.to_string())),
        request
            .title
            .as_ref()
            .map(|title| ("title", title.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.upload");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(UploadError::Client)
        .and_then(|result| {
            serde_json::from_str::<UploadResponse>(&result)
                .map_err(|e| UploadError::MalformedResponse(result, e))
        })
}
