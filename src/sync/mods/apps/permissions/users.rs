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

pub use crate::mod_types::apps::permissions::users_types::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Returns list of user grants and corresponding scopes this app has on a team.
///
/// Wraps https://api.slack.com/methods/apps.permissions.users.list

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
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.permissions.users.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Enables an app to trigger a permissions modal to grant an app access to a user access scope.
///
/// Wraps https://api.slack.com/methods/apps.permissions.users.request

pub fn request<R>(
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
        Some(("user", request.user.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.permissions.users.request");
    client
        .get(&url, &params[..])
        .map_err(RequestError::Client)
        .and_then(|result| {
            serde_json::from_str::<RequestResponse>(&result)
                .map_err(|e| RequestError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
