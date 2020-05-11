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

//! Get info on your team's User Groups.

pub use crate::mod_types::usergroups_types::*;
use crate::sync::requests::SlackWebRequestSender;
use serde_json;

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
    let params = vec![
        Some(("token", token)),
        Some(("name", request.name)),
        request.handle.map(|handle| ("handle", handle)),
        request
            .description
            .map(|description| ("description", description)),
        request.channels.map(|channels| ("channels", channels)),
        request
            .include_count
            .map(|include_count| ("include_count", if include_count { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("usergroups.create");
    client
        .send(&url, &params[..])
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
    let params = vec![
        Some(("token", token)),
        Some(("usergroup", request.usergroup)),
        request
            .include_count
            .map(|include_count| ("include_count", if include_count { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("usergroups.disable");
    client
        .send(&url, &params[..])
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
    let params = vec![
        Some(("token", token)),
        Some(("usergroup", request.usergroup)),
        request
            .include_count
            .map(|include_count| ("include_count", if include_count { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("usergroups.enable");
    client
        .send(&url, &params[..])
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
    let params = vec![
        Some(("token", token)),
        request
            .include_disabled
            .map(|include_disabled| ("include_disabled", if include_disabled { "1" } else { "0" })),
        request
            .include_count
            .map(|include_count| ("include_count", if include_count { "1" } else { "0" })),
        request
            .include_users
            .map(|include_users| ("include_users", if include_users { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("usergroups.list");
    client
        .send(&url, &params[..])
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
    let params = vec![
        Some(("token", token)),
        Some(("usergroup", request.usergroup)),
        request.name.map(|name| ("name", name)),
        request.handle.map(|handle| ("handle", handle)),
        request
            .description
            .map(|description| ("description", description)),
        request.channels.map(|channels| ("channels", channels)),
        request
            .include_count
            .map(|include_count| ("include_count", if include_count { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("usergroups.update");
    client
        .send(&url, &params[..])
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
