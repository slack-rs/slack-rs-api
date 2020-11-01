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

pub mod approved;
pub mod requests;
pub mod restricted;

pub use crate::mod_types::admin::apps::*;
use crate::sync::SlackWebRequestSender;

/// Approve an app for installation on a workspace.
///
/// Wraps https://api.slack.com/methods/admin.apps.approve

pub fn approve<R>(
    client: &R,
    request: &ApproveRequest,
) -> Result<ApproveResponse, ApproveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .app_id
            .as_ref()
            .map(|app_id| ("app_id", app_id.to_string())),
        request
            .request_id
            .as_ref()
            .map(|request_id| ("request_id", request_id.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.apps.approve");
    client
        .post(&url, &params[..], &[])
        .map_err(ApproveError::Client)
        .and_then(|result| {
            serde_json::from_str::<ApproveResponse>(&result)
                .map_err(|e| ApproveError::MalformedResponse(result, e))
        })
}
/// Restrict an app for installation on a workspace.
///
/// Wraps https://api.slack.com/methods/admin.apps.restrict

pub fn restrict<R>(
    client: &R,
    request: &RestrictRequest,
) -> Result<RestrictResponse, RestrictError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .app_id
            .as_ref()
            .map(|app_id| ("app_id", app_id.to_string())),
        request
            .request_id
            .as_ref()
            .map(|request_id| ("request_id", request_id.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.apps.restrict");
    client
        .post(&url, &params[..], &[])
        .map_err(RestrictError::Client)
        .and_then(|result| {
            serde_json::from_str::<RestrictResponse>(&result)
                .map_err(|e| RestrictError::MalformedResponse(result, e))
        })
}
