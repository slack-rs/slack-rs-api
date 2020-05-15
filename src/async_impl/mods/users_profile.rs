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

pub use crate::mod_types::users_profile_types::*;
use crate::requests::SlackWebRequestSender;

/// Retrieves a user's profile information.
///
/// Wraps https://api.slack.com/methods/users.profile.get

pub async fn get<R>(
    client: &R,
    token: &str,
    request: &GetRequest<'_>,
) -> Result<GetResponse, GetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request.user.map(|user| ("user", user)),
        request
            .include_labels
            .map(|include_labels| ("include_labels", if include_labels { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("users.profile.get");
    client
        .send(&url, &params[..])
        .await
        .map_err(GetError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result)
                .map_err(|e| GetError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Set the profile information for a user.
///
/// Wraps https://api.slack.com/methods/users.profile.set

pub async fn set<R>(
    client: &R,
    token: &str,
    request: &SetRequest<'_>,
) -> Result<SetResponse, SetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request.user.map(|user| ("user", user)),
        request.profile.map(|profile| ("profile", profile)),
        request.name.map(|name| ("name", name)),
        request.value.map(|value| ("value", value)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("users.profile.set");
    client
        .send(&url, &params[..])
        .await
        .map_err(SetError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetResponse>(&result)
                .map_err(|e| SetError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
