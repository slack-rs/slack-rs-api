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

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::admin::emoji_types::*;

/// Add an emoji.
///
/// Wraps https://api.slack.com/methods/admin.emoji.add

pub async fn add<R>(client: &R, request: &AddRequest) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("name", request.name.to_string())),
        Some(("token", request.token.to_string())),
        Some(("url", request.url.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.emoji.add");
    client
        .get(&url, &params[..])
        .await
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
}
/// Add an emoji alias.
///
/// Wraps https://api.slack.com/methods/admin.emoji.addAlias

pub async fn add_alias<R>(
    client: &R,
    request: &AddAliasRequest,
) -> Result<AddAliasResponse, AddAliasError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("alias_for", request.alias_for.to_string())),
        Some(("name", request.name.to_string())),
        Some(("token", request.token.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.emoji.addAlias");
    client
        .get(&url, &params[..])
        .await
        .map_err(AddAliasError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddAliasResponse>(&result)
                .map_err(|e| AddAliasError::MalformedResponse(result, e))
        })
}
/// List emoji for an Enterprise Grid organization.
///
/// Wraps https://api.slack.com/methods/admin.emoji.list

pub async fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.emoji.list");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
/// Remove an emoji across an Enterprise Grid organization
///
/// Wraps https://api.slack.com/methods/admin.emoji.remove

pub async fn remove<R>(
    client: &R,
    request: &RemoveRequest,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("name", request.name.to_string())),
        Some(("token", request.token.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.emoji.remove");
    client
        .get(&url, &params[..])
        .await
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
}
/// Rename an emoji.
///
/// Wraps https://api.slack.com/methods/admin.emoji.rename

pub async fn rename<R>(
    client: &R,
    request: &RenameRequest,
) -> Result<RenameResponse, RenameError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("name", request.name.to_string())),
        Some(("new_name", request.new_name.to_string())),
        Some(("token", request.token.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.emoji.rename");
    client
        .get(&url, &params[..])
        .await
        .map_err(RenameError::Client)
        .and_then(|result| {
            serde_json::from_str::<RenameResponse>(&result)
                .map_err(|e| RenameError::MalformedResponse(result, e))
        })
}
