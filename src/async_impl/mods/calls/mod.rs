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

pub mod participants;

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::calls::*;
use std::borrow::Cow;

/// Registers a new Call.
///
/// Wraps https://api.slack.com/methods/calls.add

pub async fn add<R>(
    client: &R,
    token: &str,
    request: &AddRequest<'_>,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let date_start: Option<Cow<'_, str>> = request
        .date_start
        .as_ref()
        .map(|date_start| date_start.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .created_by
            .as_ref()
            .map(|created_by| ("created_by", created_by.as_ref())),
        date_start
            .as_ref()
            .map(|date_start| ("date_start", date_start.as_ref())),
        request
            .desktop_app_join_url
            .as_ref()
            .map(|desktop_app_join_url| ("desktop_app_join_url", desktop_app_join_url.as_ref())),
        request
            .external_display_id
            .as_ref()
            .map(|external_display_id| ("external_display_id", external_display_id.as_ref())),
        Some(("external_unique_id", request.external_unique_id.as_ref())),
        Some(("join_url", request.join_url.as_ref())),
        request
            .title
            .as_ref()
            .map(|title| ("title", title.as_ref())),
        request
            .users
            .as_ref()
            .map(|users| ("users", users.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/calls.add");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Ends a Call.
///
/// Wraps https://api.slack.com/methods/calls.end

pub async fn end<R>(
    client: &R,
    token: &str,
    request: &EndRequest<'_>,
) -> Result<EndResponse, EndError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let duration: Option<Cow<'_, str>> = request
        .duration
        .as_ref()
        .map(|duration| duration.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        duration
            .as_ref()
            .map(|duration| ("duration", duration.as_ref())),
        Some(("id", request.id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/calls.end");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(EndError::Client)
        .and_then(|result| {
            serde_json::from_str::<EndResponse>(&result)
                .map_err(|e| EndError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Returns information about a Call.
///
/// Wraps https://api.slack.com/methods/calls.info

pub async fn info<R>(
    client: &R,
    token: &str,
    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> =
        vec![Some(("token", token)), Some(("id", request.id.as_ref()))];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/calls.info");
    client
        .get(&url, &params[..])
        .await
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Updates information about a Call.
///
/// Wraps https://api.slack.com/methods/calls.update

pub async fn update<R>(
    client: &R,
    token: &str,
    request: &UpdateRequest<'_>,
) -> Result<UpdateResponse, UpdateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .desktop_app_join_url
            .as_ref()
            .map(|desktop_app_join_url| ("desktop_app_join_url", desktop_app_join_url.as_ref())),
        Some(("id", request.id.as_ref())),
        request
            .join_url
            .as_ref()
            .map(|join_url| ("join_url", join_url.as_ref())),
        request
            .title
            .as_ref()
            .map(|title| ("title", title.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/calls.update");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
