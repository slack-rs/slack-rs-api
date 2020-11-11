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
pub use crate::mod_types::admin::conversations::restrict_access_types::*;
use std::borrow::Cow;

/// Add an allowlist of IDP groups for accessing a channel
///
/// Wraps https://api.slack.com/methods/admin.conversations.restrictAccess.addGroup

pub async fn add_group<R>(
    client: &R,
    token: &str,
    request: &AddGroupRequest<'_>,
) -> Result<AddGroupResponse, AddGroupError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_id", request.channel_id.as_ref())),
        Some(("group_id", request.group_id.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.restrictAccess.addGroup");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(AddGroupError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddGroupResponse>(&result)
                .map_err(|e| AddGroupError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// List all IDP Groups linked to a channel
///
/// Wraps https://api.slack.com/methods/admin.conversations.restrictAccess.listGroups

pub async fn list_groups<R>(
    client: &R,
    token: &str,
    request: &ListGroupsRequest<'_>,
) -> Result<ListGroupsResponse, ListGroupsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        Some(("channel_id", request.channel_id.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.restrictAccess.listGroups");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListGroupsError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListGroupsResponse>(&result)
                .map_err(|e| ListGroupsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Remove a linked IDP group linked from a private channel
///
/// Wraps https://api.slack.com/methods/admin.conversations.restrictAccess.removeGroup

pub async fn remove_group<R>(
    client: &R,
    token: &str,
    request: &RemoveGroupRequest<'_>,
) -> Result<RemoveGroupResponse, RemoveGroupError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_id", request.channel_id.as_ref())),
        Some(("group_id", request.group_id.as_ref())),
        Some(("team_id", request.team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.restrictAccess.removeGroup");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(RemoveGroupError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveGroupResponse>(&result)
                .map_err(|e| RemoveGroupError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
