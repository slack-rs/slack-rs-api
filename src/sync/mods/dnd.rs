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

//! Adjust and view Do Not Disturb settings for team members.

pub use crate::mod_types::dnd_types::*;
use crate::sync::requests::SlackWebRequestSender;

/// Ends the current user's Do Not Disturb session immediately.
///
/// Wraps https://api.slack.com/methods/dnd.endDnd

pub fn end_dnd<R>(client: &R, token: &str) -> Result<EndDndResponse, EndDndError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = crate::get_slack_url_for_method("dnd.endDnd");
    client
        .send(&url, &params[..])
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

pub fn end_snooze<R>(client: &R, token: &str) -> Result<EndSnoozeResponse, EndSnoozeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = crate::get_slack_url_for_method("dnd.endSnooze");
    client
        .send(&url, &params[..])
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
    token: &str,
    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request.user.map(|user| ("user", user)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("dnd.info");
    client
        .send(&url, &params[..])
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
    let num_minutes = request.num_minutes.to_string();
    let params = vec![
        Some(("token", token)),
        Some(("num_minutes", &num_minutes[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("dnd.setSnooze");
    client
        .send(&url, &params[..])
        .map_err(SetSnoozeError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetSnoozeResponse>(&result)
                .map_err(|e| SetSnoozeError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Retrieves the Do Not Disturb status for users on a team.
///
/// Wraps https://api.slack.com/methods/dnd.teamInfo

pub fn team_info<R>(
    client: &R,
    token: &str,
    request: &TeamInfoRequest<'_>,
) -> Result<TeamInfoResponse, TeamInfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request.users.map(|users| ("users", users)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("dnd.teamInfo");
    client
        .send(&url, &params[..])
        .map_err(TeamInfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<TeamInfoResponse>(&result)
                .map_err(|e| TeamInfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
