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
pub use crate::mod_types::admin::usergroups_types::*;

/// Add one or more default channels to an IDP group.
///
/// Wraps https://api.slack.com/methods/admin.usergroups.addChannels

pub async fn add_channels<R>(
    client: &R,
    request: &AddChannelsRequest,
) -> Result<AddChannelsResponse, AddChannelsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_ids", request.channel_ids.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
        Some(("usergroup_id", request.usergroup_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.usergroups.addChannels");
    client
        .post(&url, &params[..], &[])
        .await
        .map_err(AddChannelsError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddChannelsResponse>(&result)
                .map_err(|e| AddChannelsError::MalformedResponse(result, e))
        })
}
/// Associate one or more default workspaces with an organization-wide IDP group.
///
/// Wraps https://api.slack.com/methods/admin.usergroups.addTeams

pub async fn add_teams<R>(
    client: &R,
    request: &AddTeamsRequest,
) -> Result<AddTeamsResponse, AddTeamsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .auto_provision
            .as_ref()
            .map(|auto_provision| ("auto_provision", auto_provision.to_string())),
        Some(("team_ids", request.team_ids.to_string())),
        Some(("usergroup_id", request.usergroup_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.usergroups.addTeams");
    client
        .post(&url, &params[..], &[])
        .await
        .map_err(AddTeamsError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddTeamsResponse>(&result)
                .map_err(|e| AddTeamsError::MalformedResponse(result, e))
        })
}
/// List the channels linked to an org-level IDP group (user group).
///
/// Wraps https://api.slack.com/methods/admin.usergroups.listChannels

pub async fn list_channels<R>(
    client: &R,
    request: &ListChannelsRequest,
) -> Result<ListChannelsResponse, ListChannelsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .include_num_members
            .as_ref()
            .map(|include_num_members| ("include_num_members", include_num_members.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
        Some(("usergroup_id", request.usergroup_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.usergroups.listChannels");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListChannelsError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListChannelsResponse>(&result)
                .map_err(|e| ListChannelsError::MalformedResponse(result, e))
        })
}
/// Remove one or more default channels from an org-level IDP group (user group).
///
/// Wraps https://api.slack.com/methods/admin.usergroups.removeChannels

pub async fn remove_channels<R>(
    client: &R,
    request: &RemoveChannelsRequest,
) -> Result<RemoveChannelsResponse, RemoveChannelsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_ids", request.channel_ids.to_string())),
        Some(("usergroup_id", request.usergroup_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.usergroups.removeChannels");
    client
        .post(&url, &params[..], &[])
        .await
        .map_err(RemoveChannelsError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveChannelsResponse>(&result)
                .map_err(|e| RemoveChannelsError::MalformedResponse(result, e))
        })
}
