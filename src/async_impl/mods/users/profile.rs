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

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::users::profile_types::*;

/// Retrieves a user's profile information.
///
/// Wraps https://api.slack.com/methods/users.profile.get

pub async fn get<R>(client: &R, request: &GetRequest) -> Result<GetResponse, GetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
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
        .await
        .map_err(GetError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result)
                .map_err(|e| GetError::MalformedResponse(result, e))
        })
}
/// Set the profile information for a user.
///
/// Wraps https://api.slack.com/methods/users.profile.set

pub async fn set<R>(client: &R, request: &SetRequest) -> Result<SetResponse, SetError<R::Error>>
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
        .get(&url, &params[..])
        .await
        .map_err(SetError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetResponse>(&result)
                .map_err(|e| SetError::MalformedResponse(result, e))
        })
}
