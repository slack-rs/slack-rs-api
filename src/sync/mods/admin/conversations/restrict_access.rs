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

pub use crate::mod_types::admin::conversations::restrict_access_types::*;
use crate::sync::SlackWebRequestSender;

/// Add an allowlist of IDP groups for accessing a channel
///
/// Wraps https://api.slack.com/methods/admin.conversations.restrictAccess.addGroup

pub fn add_group<R>(
    client: &R,
    request: &AddGroupRequest,
) -> Result<AddGroupResponse, AddGroupError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_id", request.channel_id.to_string())),
        Some(("group_id", request.group_id.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.restrictAccess.addGroup");
    client
        .post(&url, &params[..], &[("token", request.token.clone())])
        .map_err(AddGroupError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddGroupResponse>(&result)
                .map_err(|e| AddGroupError::MalformedResponse(result, e))
        })
}
/// List all IDP Groups linked to a channel
///
/// Wraps https://api.slack.com/methods/admin.conversations.restrictAccess.listGroups

pub fn list_groups<R>(
    client: &R,
    request: &ListGroupsRequest,
) -> Result<ListGroupsResponse, ListGroupsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_id", request.channel_id.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.restrictAccess.listGroups");
    client
        .get(&url, &params[..])
        .map_err(ListGroupsError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListGroupsResponse>(&result)
                .map_err(|e| ListGroupsError::MalformedResponse(result, e))
        })
}
/// Remove a linked IDP group linked from a private channel
///
/// Wraps https://api.slack.com/methods/admin.conversations.restrictAccess.removeGroup

pub fn remove_group<R>(
    client: &R,
    request: &RemoveGroupRequest,
) -> Result<RemoveGroupResponse, RemoveGroupError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_id", request.channel_id.to_string())),
        Some(("group_id", request.group_id.to_string())),
        Some(("team_id", request.team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.restrictAccess.removeGroup");
    client
        .post(&url, &params[..], &[("token", request.token.clone())])
        .map_err(RemoveGroupError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveGroupResponse>(&result)
                .map_err(|e| RemoveGroupError::MalformedResponse(result, e))
        })
}
