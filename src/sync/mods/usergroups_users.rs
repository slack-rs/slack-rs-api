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

pub use crate::mod_types::usergroups_users_types::*;
use crate::sync::requests::SlackWebRequestSender;

/// List all users in a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.users.list

pub fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("usergroup", request.usergroup)),
        request
            .include_disabled
            .map(|include_disabled| ("include_disabled", if include_disabled { "1" } else { "0" })),
    ];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("usergroups.users.list");
    client
        .send(&url, &params[..])
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
        Some(("users", request.users)),
        request
            .include_count
            .map(|include_count| ("include_count", if include_count { "1" } else { "0" })),
    ];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("usergroups.users.update");
    client
        .send(&url, &params[..])
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
