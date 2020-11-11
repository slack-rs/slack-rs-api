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

pub mod approved;
pub mod requests;
pub mod restricted;

pub use crate::mod_types::admin::apps::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Approve an app for installation on a workspace.
///
/// Wraps https://api.slack.com/methods/admin.apps.approve

pub fn approve<R>(
    client: &R,
    token: &str,
    request: &ApproveRequest<'_>,
) -> Result<ApproveResponse, ApproveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .app_id
            .as_ref()
            .map(|app_id| ("app_id", app_id.as_ref())),
        request
            .enterprise_id
            .as_ref()
            .map(|enterprise_id| ("enterprise_id", enterprise_id.as_ref())),
        request
            .request_id
            .as_ref()
            .map(|request_id| ("request_id", request_id.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.apps.approve");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(ApproveError::Client)
        .and_then(|result| {
            serde_json::from_str::<ApproveResponse>(&result)
                .map_err(|e| ApproveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Restrict an app for installation on a workspace.
///
/// Wraps https://api.slack.com/methods/admin.apps.restrict

pub fn restrict<R>(
    client: &R,
    token: &str,
    request: &RestrictRequest<'_>,
) -> Result<RestrictResponse, RestrictError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .app_id
            .as_ref()
            .map(|app_id| ("app_id", app_id.as_ref())),
        request
            .request_id
            .as_ref()
            .map(|request_id| ("request_id", request_id.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.apps.restrict");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(RestrictError::Client)
        .and_then(|result| {
            serde_json::from_str::<RestrictResponse>(&result)
                .map_err(|e| RestrictError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
