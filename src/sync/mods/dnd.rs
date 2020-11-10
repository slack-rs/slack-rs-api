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

pub use crate::mod_types::dnd_types::*;
use crate::sync::SlackWebRequestSender;

/// Ends the current user's Do Not Disturb session immediately.
///
/// Wraps https://api.slack.com/methods/dnd.endDnd

pub fn end_dnd<R>(
    client: &R,
    token: &str,
    _request: &EndDndRequest,
) -> Result<EndDndResponse, EndDndError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/dnd.endDnd");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(EndDndError::Client)
        .and_then(|result| {
            serde_json::from_str::<EndDndResponse>(&result)
                .map_err(|e| EndDndError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Ends the current user's snooze mode immediately.
///
/// Wraps https://api.slack.com/methods/dnd.endSnooze

pub fn end_snooze<R>(
    client: &R,
    token: &str,
    _request: &EndSnoozeRequest,
) -> Result<EndSnoozeResponse, EndSnoozeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/dnd.endSnooze");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(EndSnoozeError::Client)
        .and_then(|result| {
            serde_json::from_str::<EndSnoozeResponse>(&result)
                .map_err(|e| EndSnoozeError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Retrieves a user's current Do Not Disturb status.
///
/// Wraps https://api.slack.com/methods/dnd.info

pub fn info<R>(
    client: &R,
    token: Option<&str>,
    request: &InfoRequest,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        token.map(|token| ("token", token.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/dnd.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Turns on Do Not Disturb mode for the current user, or changes its duration.
///
/// Wraps https://api.slack.com/methods/dnd.setSnooze

pub fn set_snooze<R>(
    client: &R,
    token: &str,
    request: &SetSnoozeRequest,
) -> Result<SetSnoozeResponse, SetSnoozeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("num_minutes", request.num_minutes.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/dnd.setSnooze");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(SetSnoozeError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetSnoozeResponse>(&result)
                .map_err(|e| SetSnoozeError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Retrieves the Do Not Disturb status for up to 50 users on a team.
///
/// Wraps https://api.slack.com/methods/dnd.teamInfo

pub fn team_info<R>(
    client: &R,
    token: Option<&str>,
    request: &TeamInfoRequest,
) -> Result<TeamInfoResponse, TeamInfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        token.map(|token| ("token", token.to_string())),
        request
            .users
            .as_ref()
            .map(|users| ("users", users.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/dnd.teamInfo");
    client
        .get(&url, &params[..])
        .map_err(TeamInfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<TeamInfoResponse>(&result)
                .map_err(|e| TeamInfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
