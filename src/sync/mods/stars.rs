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

pub use crate::mod_types::stars_types::*;
use crate::sync::SlackWebRequestSender;

/// Adds a star to an item.
///
/// Wraps https://api.slack.com/methods/stars.add

pub fn add<R>(
    client: &R,
    token: &str,
    request: &AddRequest,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
        request
            .file_comment
            .as_ref()
            .map(|file_comment| ("file_comment", file_comment.to_string())),
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/stars.add");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
}
/// Lists stars for a user.
///
/// Wraps https://api.slack.com/methods/stars.list

pub fn list<R>(
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
            .count
            .as_ref()
            .map(|count| ("count", count.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request.page.as_ref().map(|page| ("page", page.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/stars.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
/// Removes a star from an item.
///
/// Wraps https://api.slack.com/methods/stars.remove

pub fn remove<R>(
    client: &R,
    token: &str,
    request: &RemoveRequest,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
        request
            .file_comment
            .as_ref()
            .map(|file_comment| ("file_comment", file_comment.to_string())),
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/stars.remove");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
}
