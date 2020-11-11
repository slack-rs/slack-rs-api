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

pub mod resources;
pub mod scopes;
pub mod users;

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::apps::permissions::*;
use std::borrow::Cow;

/// Returns list of permissions this app has on a team.
///
/// Wraps https://api.slack.com/methods/apps.permissions.info

pub async fn info<R>(
    client: &R,
    token: Option<&str>,
    _request: &InfoRequest,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![token.map(|token| ("token", token))];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.permissions.info");
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
/// Allows an app to request additional scopes
///
/// Wraps https://api.slack.com/methods/apps.permissions.request

pub async fn request<R>(
    client: &R,
    token: &str,
    request: &RequestRequest<'_>,
) -> Result<RequestResponse, RequestError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        Some(("scopes", request.scopes.as_ref())),
        Some(("trigger_id", request.trigger_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.permissions.request");
    client
        .get(&url, &params[..])
        .await
        .map_err(RequestError::Client)
        .and_then(|result| {
            serde_json::from_str::<RequestResponse>(&result)
                .map_err(|e| RequestError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
