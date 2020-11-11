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
pub mod denied;

pub use crate::mod_types::admin::invite_requests::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Approve a workspace invite request.
///
/// Wraps https://api.slack.com/methods/admin.inviteRequests.approve

pub fn approve<R>(
    client: &R,
    token: &str,
    request: &ApproveRequest<'_>,
) -> Result<ApproveResponse, ApproveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("invite_request_id", request.invite_request_id.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.inviteRequests.approve");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(ApproveError::Client)
        .and_then(|result| {
            serde_json::from_str::<ApproveResponse>(&result)
                .map_err(|e| ApproveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Deny a workspace invite request.
///
/// Wraps https://api.slack.com/methods/admin.inviteRequests.deny

pub fn deny<R>(
    client: &R,
    token: &str,
    request: &DenyRequest<'_>,
) -> Result<DenyResponse, DenyError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("invite_request_id", request.invite_request_id.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.inviteRequests.deny");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(DenyError::Client)
        .and_then(|result| {
            serde_json::from_str::<DenyResponse>(&result)
                .map_err(|e| DenyError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// List all pending workspace invite requests.
///
/// Wraps https://api.slack.com/methods/admin.inviteRequests.list

pub fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.inviteRequests.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
