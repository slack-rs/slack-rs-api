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

pub mod profile;

pub use crate::mod_types::team::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Gets the access logs for the current team.
///
/// Wraps https://api.slack.com/methods/team.accessLogs

pub fn access_logs<R>(
    client: &R,
    token: &str,
    request: &AccessLogsRequest<'_>,
) -> Result<AccessLogsResponse, AccessLogsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        request
            .before
            .as_ref()
            .map(|before| ("before", before.as_ref())),
        request
            .count
            .as_ref()
            .map(|count| ("count", count.as_ref())),
        request.page.as_ref().map(|page| ("page", page.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/team.accessLogs");
    client
        .get(&url, &params[..])
        .map_err(AccessLogsError::Client)
        .and_then(|result| {
            serde_json::from_str::<AccessLogsResponse>(&result)
                .map_err(|e| AccessLogsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Gets billable users information for the current team.
///
/// Wraps https://api.slack.com/methods/team.billableInfo

pub fn billable_info<R>(
    client: &R,
    token: &str,
    request: &BillableInfoRequest<'_>,
) -> Result<BillableInfoResponse, BillableInfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        request.user.as_ref().map(|user| ("user", user.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/team.billableInfo");
    client
        .get(&url, &params[..])
        .map_err(BillableInfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<BillableInfoResponse>(&result)
                .map_err(|e| BillableInfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Gets information about the current team.
///
/// Wraps https://api.slack.com/methods/team.info

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
        request.team.as_ref().map(|team| ("team", team.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/team.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Gets the integration logs for the current team.
///
/// Wraps https://api.slack.com/methods/team.integrationLogs

pub fn integration_logs<R>(
    client: &R,
    token: &str,
    request: &IntegrationLogsRequest<'_>,
) -> Result<IntegrationLogsResponse, IntegrationLogsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        request
            .app_id
            .as_ref()
            .map(|app_id| ("app_id", app_id.as_ref())),
        request
            .change_type
            .as_ref()
            .map(|change_type| ("change_type", change_type.as_ref())),
        request
            .count
            .as_ref()
            .map(|count| ("count", count.as_ref())),
        request.page.as_ref().map(|page| ("page", page.as_ref())),
        request
            .service_id
            .as_ref()
            .map(|service_id| ("service_id", service_id.as_ref())),
        request.user.as_ref().map(|user| ("user", user.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/team.integrationLogs");
    client
        .get(&url, &params[..])
        .map_err(IntegrationLogsError::Client)
        .and_then(|result| {
            serde_json::from_str::<IntegrationLogsResponse>(&result)
                .map_err(|e| IntegrationLogsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
