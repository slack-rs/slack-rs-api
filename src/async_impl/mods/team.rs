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

pub use crate::mod_types::team_types::*;
use crate::requests::SlackWebRequestSender;

/// Gets the access logs for the current team.
///
/// Wraps https://api.slack.com/methods/team.accessLogs

pub async fn access_logs<R>(
    client: &R,
    token: &str,
    request: &AccessLogsRequest,
) -> Result<AccessLogsResponse, AccessLogsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let before = request.before.map(|before| before.to_string());
    let params = vec![
        Some(("token", token)),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
        before.as_ref().map(|before| ("before", &before[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("team.accessLogs");
    client
        .send(&url, &params[..])
        .await
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

pub async fn billable_info<R>(
    client: &R,
    token: &str,
    request: &BillableInfoRequest<'_>,
) -> Result<BillableInfoResponse, BillableInfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request.user.map(|user| ("user", user)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("team.billableInfo");
    client
        .send(&url, &params[..])
        .await
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

pub async fn info<R>(client: &R, token: &str) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = crate::get_slack_url_for_method("team.info");
    client
        .send(&url, &params[..])
        .await
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

pub async fn integration_logs<R>(
    client: &R,
    token: &str,
    request: &IntegrationLogsRequest<'_>,
) -> Result<IntegrationLogsResponse, IntegrationLogsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        request
            .service_id
            .map(|service_id| ("service_id", service_id)),
        request.app_id.map(|app_id| ("app_id", app_id)),
        request.user.map(|user| ("user", user)),
        request
            .change_type
            .map(|change_type| ("change_type", change_type)),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("team.integrationLogs");
    client
        .send(&url, &params[..])
        .await
        .map_err(IntegrationLogsError::Client)
        .and_then(|result| {
            serde_json::from_str::<IntegrationLogsResponse>(&result)
                .map_err(|e| IntegrationLogsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
