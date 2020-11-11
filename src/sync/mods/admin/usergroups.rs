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

pub use crate::mod_types::admin::usergroups_types::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Add one or more default channels to an IDP group.
///
/// Wraps https://api.slack.com/methods/admin.usergroups.addChannels

pub fn add_channels<R>(
    client: &R,
    token: &str,
    request: &AddChannelsRequest<'_>,
) -> Result<AddChannelsResponse, AddChannelsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_ids", request.channel_ids.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
        Some(("usergroup_id", request.usergroup_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.usergroups.addChannels");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(AddChannelsError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddChannelsResponse>(&result)
                .map_err(|e| AddChannelsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Associate one or more default workspaces with an organization-wide IDP group.
///
/// Wraps https://api.slack.com/methods/admin.usergroups.addTeams

pub fn add_teams<R>(
    client: &R,
    token: &str,
    request: &AddTeamsRequest<'_>,
) -> Result<AddTeamsResponse, AddTeamsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let auto_provision: Option<Cow<'_, str>> = request
        .auto_provision
        .as_ref()
        .map(|auto_provision| auto_provision.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        auto_provision
            .as_ref()
            .map(|auto_provision| ("auto_provision", auto_provision.as_ref())),
        Some(("team_ids", request.team_ids.as_ref())),
        Some(("usergroup_id", request.usergroup_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.usergroups.addTeams");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(AddTeamsError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddTeamsResponse>(&result)
                .map_err(|e| AddTeamsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// List the channels linked to an org-level IDP group (user group).
///
/// Wraps https://api.slack.com/methods/admin.usergroups.listChannels

pub fn list_channels<R>(
    client: &R,
    token: &str,
    request: &ListChannelsRequest<'_>,
) -> Result<ListChannelsResponse, ListChannelsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let include_num_members: Option<Cow<'_, str>> = request
        .include_num_members
        .as_ref()
        .map(|include_num_members| include_num_members.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        include_num_members
            .as_ref()
            .map(|include_num_members| ("include_num_members", include_num_members.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
        Some(("usergroup_id", request.usergroup_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.usergroups.listChannels");
    client
        .get(&url, &params[..])
        .map_err(ListChannelsError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListChannelsResponse>(&result)
                .map_err(|e| ListChannelsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Remove one or more default channels from an org-level IDP group (user group).
///
/// Wraps https://api.slack.com/methods/admin.usergroups.removeChannels

pub fn remove_channels<R>(
    client: &R,
    token: &str,
    request: &RemoveChannelsRequest<'_>,
) -> Result<RemoveChannelsResponse, RemoveChannelsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_ids", request.channel_ids.as_ref())),
        Some(("usergroup_id", request.usergroup_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.usergroups.removeChannels");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(RemoveChannelsError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveChannelsResponse>(&result)
                .map_err(|e| RemoveChannelsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
