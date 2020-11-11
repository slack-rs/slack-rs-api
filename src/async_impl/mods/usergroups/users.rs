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

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::usergroups::users_types::*;
use std::borrow::Cow;

/// List all users in a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.users.list

pub async fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let include_disabled: Option<Cow<'_, str>> = request
        .include_disabled
        .as_ref()
        .map(|include_disabled| include_disabled.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        include_disabled
            .as_ref()
            .map(|include_disabled| ("include_disabled", include_disabled.as_ref())),
        Some(("usergroup", request.usergroup.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/usergroups.users.list");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Update the list of users for a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.users.update

pub async fn update<R>(
    client: &R,
    token: &str,
    request: &UpdateRequest<'_>,
) -> Result<UpdateResponse, UpdateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let include_count: Option<Cow<'_, str>> = request
        .include_count
        .as_ref()
        .map(|include_count| include_count.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        include_count
            .as_ref()
            .map(|include_count| ("include_count", include_count.as_ref())),
        Some(("usergroup", request.usergroup.as_ref())),
        Some(("users", request.users.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/usergroups.users.update");
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
