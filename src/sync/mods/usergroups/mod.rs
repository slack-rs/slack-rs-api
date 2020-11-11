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

pub mod users;

pub use crate::mod_types::usergroups::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Create a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.create

pub fn create<R>(
    client: &R,
    token: &str,
    request: &CreateRequest<'_>,
) -> Result<CreateResponse, CreateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let include_count: Option<Cow<'_, str>> = request
        .include_count
        .as_ref()
        .map(|include_count| include_count.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .channels
            .as_ref()
            .map(|channels| ("channels", channels.as_ref())),
        request
            .description
            .as_ref()
            .map(|description| ("description", description.as_ref())),
        request
            .handle
            .as_ref()
            .map(|handle| ("handle", handle.as_ref())),
        include_count
            .as_ref()
            .map(|include_count| ("include_count", include_count.as_ref())),
        Some(("name", request.name.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/usergroups.create");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(CreateError::Client)
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result)
                .map_err(|e| CreateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Disable an existing User Group
///
/// Wraps https://api.slack.com/methods/usergroups.disable

pub fn disable<R>(
    client: &R,
    token: &str,
    request: &DisableRequest<'_>,
) -> Result<DisableResponse, DisableError<R::Error>>
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
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/usergroups.disable");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(DisableError::Client)
        .and_then(|result| {
            serde_json::from_str::<DisableResponse>(&result)
                .map_err(|e| DisableError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Enable a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.enable

pub fn enable<R>(
    client: &R,
    token: &str,
    request: &EnableRequest<'_>,
) -> Result<EnableResponse, EnableError<R::Error>>
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
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/usergroups.enable");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(EnableError::Client)
        .and_then(|result| {
            serde_json::from_str::<EnableResponse>(&result)
                .map_err(|e| EnableError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// List all User Groups for a team
///
/// Wraps https://api.slack.com/methods/usergroups.list

pub fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let include_count: Option<Cow<'_, str>> = request
        .include_count
        .as_ref()
        .map(|include_count| include_count.to_string().into());
    let include_disabled: Option<Cow<'_, str>> = request
        .include_disabled
        .as_ref()
        .map(|include_disabled| include_disabled.to_string().into());
    let include_users: Option<Cow<'_, str>> = request
        .include_users
        .as_ref()
        .map(|include_users| include_users.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        include_count
            .as_ref()
            .map(|include_count| ("include_count", include_count.as_ref())),
        include_disabled
            .as_ref()
            .map(|include_disabled| ("include_disabled", include_disabled.as_ref())),
        include_users
            .as_ref()
            .map(|include_users| ("include_users", include_users.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/usergroups.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Update an existing User Group
///
/// Wraps https://api.slack.com/methods/usergroups.update

pub fn update<R>(
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
        request
            .channels
            .as_ref()
            .map(|channels| ("channels", channels.as_ref())),
        request
            .description
            .as_ref()
            .map(|description| ("description", description.as_ref())),
        request
            .handle
            .as_ref()
            .map(|handle| ("handle", handle.as_ref())),
        include_count
            .as_ref()
            .map(|include_count| ("include_count", include_count.as_ref())),
        request.name.as_ref().map(|name| ("name", name.as_ref())),
        Some(("usergroup", request.usergroup.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/usergroups.update");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
