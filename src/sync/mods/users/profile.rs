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

pub use crate::mod_types::users::profile_types::*;
use crate::sync::SlackWebRequestSender;

/// Retrieves a user's profile information.
///
/// Wraps https://api.slack.com/methods/users.profile.get

pub fn get<R>(
    client: &R,
    token: &str,
    request: &GetRequest,
) -> Result<GetResponse, GetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        request
            .include_labels
            .as_ref()
            .map(|include_labels| ("include_labels", include_labels.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.profile.get");
    client
        .get(&url, &params[..])
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

pub fn set<R>(
    client: &R,
    token: &str,
    request: &SetRequest,
) -> Result<SetResponse, SetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request.name.as_ref().map(|name| ("name", name.to_string())),
        request
            .profile
            .as_ref()
            .map(|profile| ("profile", profile.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
        request
            .value
            .as_ref()
            .map(|value| ("value", value.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.profile.set");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(SetError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetResponse>(&result)
                .map_err(|e| SetError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
