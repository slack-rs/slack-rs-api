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

pub use crate::mod_types::apps::permissions::users_types::*;
use crate::sync::SlackWebRequestSender;

/// Returns list of user grants and corresponding scopes this app has on a team.
///
/// Wraps https://api.slack.com/methods/apps.permissions.users.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.permissions.users.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
/// Enables an app to trigger a permissions modal to grant an app access to a user access scope.
///
/// Wraps https://api.slack.com/methods/apps.permissions.users.request

pub fn request<R>(
    client: &R,
    request: &RequestRequest,
) -> Result<RequestResponse, RequestError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("scopes", request.scopes.to_string())),
        Some(("trigger_id", request.trigger_id.to_string())),
        Some(("user", request.user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.permissions.users.request");
    client
        .get(&url, &params[..])
        .map_err(RequestError::Client)
        .and_then(|result| {
            serde_json::from_str::<RequestResponse>(&result)
                .map_err(|e| RequestError::MalformedResponse(result, e))
        })
}
