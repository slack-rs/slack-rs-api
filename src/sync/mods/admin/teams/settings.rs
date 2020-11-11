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

pub use crate::mod_types::admin::teams::settings_types::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Fetch information about settings in a workspace
///
/// Wraps https://api.slack.com/methods/admin.teams.settings.info

pub fn info<R>(
    client: &R,
    token: &str,
    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        Some(("team_id", request.team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.teams.settings.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set the default channels of a workspace.
///
/// Wraps https://api.slack.com/methods/admin.teams.settings.setDefaultChannels

pub fn set_default_channels<R>(
    client: &R,
    token: &str,
    request: &SetDefaultChannelsRequest<'_>,
) -> Result<SetDefaultChannelsResponse, SetDefaultChannelsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_ids", request.channel_ids.as_ref())),
        Some(("team_id", request.team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.teams.settings.setDefaultChannels");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(SetDefaultChannelsError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetDefaultChannelsResponse>(&result)
                .map_err(|e| SetDefaultChannelsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set the description of a given workspace.
///
/// Wraps https://api.slack.com/methods/admin.teams.settings.setDescription

pub fn set_description<R>(
    client: &R,
    token: &str,
    request: &SetDescriptionRequest<'_>,
) -> Result<SetDescriptionResponse, SetDescriptionError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("description", request.description.as_ref())),
        Some(("team_id", request.team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.teams.settings.setDescription");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(SetDescriptionError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetDescriptionResponse>(&result)
                .map_err(|e| SetDescriptionError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// An API method that allows admins to set the discoverability of a given workspace
///
/// Wraps https://api.slack.com/methods/admin.teams.settings.setDiscoverability

pub fn set_discoverability<R>(
    client: &R,
    token: &str,
    request: &SetDiscoverabilityRequest<'_>,
) -> Result<SetDiscoverabilityResponse, SetDiscoverabilityError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("discoverability", request.discoverability.as_ref())),
        Some(("team_id", request.team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.teams.settings.setDiscoverability");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(SetDiscoverabilityError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetDiscoverabilityResponse>(&result)
                .map_err(|e| SetDiscoverabilityError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Sets the icon of a workspace.
///
/// Wraps https://api.slack.com/methods/admin.teams.settings.setIcon

pub fn set_icon<R>(
    client: &R,
    token: &str,
    request: &SetIconRequest<'_>,
) -> Result<SetIconResponse, SetIconError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("image_url", request.image_url.as_ref())),
        Some(("team_id", request.team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.teams.settings.setIcon");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(SetIconError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetIconResponse>(&result)
                .map_err(|e| SetIconError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set the name of a given workspace.
///
/// Wraps https://api.slack.com/methods/admin.teams.settings.setName

pub fn set_name<R>(
    client: &R,
    token: &str,
    request: &SetNameRequest<'_>,
) -> Result<SetNameResponse, SetNameError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("name", request.name.as_ref())),
        Some(("team_id", request.team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.teams.settings.setName");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(SetNameError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetNameResponse>(&result)
                .map_err(|e| SetNameError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
