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

pub use crate::mod_types::admin::users::session_types::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Invalidate a single session for a user by session_id
///
/// Wraps https://api.slack.com/methods/admin.users.session.invalidate

pub fn invalidate<R>(
    client: &R,
    token: &str,
    request: &InvalidateRequest<'_>,
) -> Result<InvalidateResponse, InvalidateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let session_id: Option<Cow<'_, str>> = Some(request.session_id.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        session_id
            .as_ref()
            .map(|session_id| ("session_id", session_id.as_ref())),
        Some(("team_id", request.team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.session.invalidate");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(InvalidateError::Client)
        .and_then(|result| {
            serde_json::from_str::<InvalidateResponse>(&result)
                .map_err(|e| InvalidateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Wipes all valid sessions on all devices for a given user
///
/// Wraps https://api.slack.com/methods/admin.users.session.reset

pub fn reset<R>(
    client: &R,
    token: &str,
    request: &ResetRequest<'_>,
) -> Result<ResetResponse, ResetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let mobile_only: Option<Cow<'_, str>> = request
        .mobile_only
        .as_ref()
        .map(|mobile_only| mobile_only.to_string().into());
    let web_only: Option<Cow<'_, str>> = request
        .web_only
        .as_ref()
        .map(|web_only| web_only.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        mobile_only
            .as_ref()
            .map(|mobile_only| ("mobile_only", mobile_only.as_ref())),
        Some(("user_id", request.user_id.as_ref())),
        web_only
            .as_ref()
            .map(|web_only| ("web_only", web_only.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.session.reset");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(ResetError::Client)
        .and_then(|result| {
            serde_json::from_str::<ResetResponse>(&result)
                .map_err(|e| ResetError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
