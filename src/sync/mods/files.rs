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

//! Get info on files uploaded to Slack, upload new files to Slack.

pub use crate::mod_types::files_types::*;
use crate::sync::requests::SlackWebRequestSender;
use serde_json;

/// Deletes a file.
///
/// Wraps https://api.slack.com/methods/files.delete

pub fn delete<R>(
    client: &R,
    token: &str,
    request: &DeleteRequest<'_>,
) -> Result<DeleteResponse, DeleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("file", request.file))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("files.delete");
    client
        .send(&url, &params[..])
        .map_err(DeleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|e| DeleteError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Gets information about a team file.
///
/// Wraps https://api.slack.com/methods/files.info

pub fn info<R>(
    client: &R,
    token: &str,
    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        Some(("file", request.file)),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("files.info");
    client
        .send(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Lists & filters team files.
///
/// Wraps https://api.slack.com/methods/files.list

pub fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let ts_from = request.ts_from.map(|ts_from| ts_from.to_string());
    let ts_to = request.ts_to.map(|ts_to| ts_to.to_string());
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        request.user.map(|user| ("user", user)),
        request.channel.map(|channel| ("channel", channel)),
        ts_from.as_ref().map(|ts_from| ("ts_from", &ts_from[..])),
        ts_to.as_ref().map(|ts_to| ("ts_to", &ts_to[..])),
        request.types.map(|types| ("types", types)),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("files.list");
    client
        .send(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Revokes public/external sharing access for a file
///
/// Wraps https://api.slack.com/methods/files.revokePublicURL

pub fn revoke_public_url<R>(
    client: &R,
    token: &str,
    request: &RevokePublicURLRequest<'_>,
) -> Result<RevokePublicURLResponse, RevokePublicURLError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("file", request.file))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("files.revokePublicURL");
    client
        .send(&url, &params[..])
        .map_err(RevokePublicURLError::Client)
        .and_then(|result| {
            serde_json::from_str::<RevokePublicURLResponse>(&result)
                .map_err(|e| RevokePublicURLError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Enables a file for public/external sharing.
///
/// Wraps https://api.slack.com/methods/files.sharedPublicURL

pub fn shared_public_url<R>(
    client: &R,
    token: &str,
    request: &SharedPublicURLRequest<'_>,
) -> Result<SharedPublicURLResponse, SharedPublicURLError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("file", request.file))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("files.sharedPublicURL");
    client
        .send(&url, &params[..])
        .map_err(SharedPublicURLError::Client)
        .and_then(|result| {
            serde_json::from_str::<SharedPublicURLResponse>(&result)
                .map_err(|e| SharedPublicURLError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
